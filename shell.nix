{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    gcc
    just
    rustup

    # optional
    binaryen
    twiggy
  ];
}
