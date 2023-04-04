use epub::doc::EpubDoc;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use tl::{Node, Parser};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        println!("{}", load_and_count(&path));
    } else {
        eprintln!("error: no file specified");
    }
}

fn count_node<'a>(parser: &Parser<'a>, node: &Node<'a>) -> Option<String> {
    match node {
        Node::Raw(raw) => Some(raw.as_utf8_str().to_string()),
        Node::Tag(ruby) => {
            if ruby.name() == "ruby" {
                Some(
                    ruby.children()
                        .top()
                        .iter()
                        .filter_map(|ruby_child| match ruby_child.get(parser).unwrap() {
                            Node::Raw(raw) => Some(raw.as_utf8_str()),
                            Node::Tag(rb) => {
                                if rb.name() == "rb" {
                                    Some(rb.inner_text(parser))
                                } else {
                                    None
                                }
                            }
                            Node::Comment(_) => None,
                        })
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        Node::Comment(_) => None,
    }
}

fn load_and_count(path: &str) -> usize {
    let mut doc = EpubDoc::new(path).expect("failed to access path");
    let num_pages = doc.get_num_pages();
    let mut total_text = 0;

    for _ in 0..num_pages {
        doc.go_next();

        let s = doc
            .get_current_str()
            .expect("cannot get html for current page")
            .0;

        let dom = tl::parse(&s, tl::ParserOptions::default()).expect("could not create parser");
        let parser = dom.parser();

        let xs: Vec<String> = dom
            .nodes()
            .iter()
            .filter_map(|n| {
                if let Some(tag) = n.as_tag() {
                    if tag.name() == "p" || tag.name() == "span" {
                        let inner = tag
                            .children()
                            .top()
                            .iter()
                            .filter_map(|child| count_node(parser, child.get(parser).unwrap()))
                            .collect::<String>();
                        return Some(inner);
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        // need to convert this to streaming
        // ... if that's even possible (it should be?)

        let len: usize = xs
            .into_par_iter()
            .map(|s| {
                // the scx here makes this count punctuation too
                // rust's regex crate doesn't support \CJK_Symbols
                let ja_regex = Regex::new(r"[\p{Han}\p{scx=Hiragana}\p{scx=Katakana}]+").unwrap();
                let num_ja_chars: usize = ja_regex
                    .find_iter(&s)
                    .map(|c| c.as_str().chars().count())
                    .sum::<usize>();
                num_ja_chars
            })
            .sum();

        total_text += len;
    }

    total_text
}
