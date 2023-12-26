{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            gcc
            cmake
            pkg-config
            convco
            (rust-bin.stable."1.74.1".default.override {
              extensions = [ "rust-src" "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];
              targets = [ "x86_64-unknown-linux-gnu"];
            })
          ];

        };
      }
    );
}
