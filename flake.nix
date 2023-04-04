{
  description = "epub character counter";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = inputs@{ flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
      in
      {
        devShells.default =
          pkgs.mkShell {
            name = "shell";
            packages = with pkgs; [
              pkgconfig
              openssl
              gcc
              rustup
              cargo-watch
              trunk
              wasm-bindgen-cli
              nodePackages.sass
              nodePackages.npm
              rust-analyzer
              nodejs
              librsvg
              webkitgtk
              libsoup
              rlwrap
              entr
              cargo-generate
            ];
          };
      });
}
