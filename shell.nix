{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    clippy
    rustfmt
    rustc
  ];
}