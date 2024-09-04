{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;
  languages.rust.channel = "stable";
  packages = [
    pkgs.vscode-extensions.vadimcn.vscode-lldb.adapter
    pkgs.taplo
    pkgs.mdbook
    pkgs.mdbook-plantuml
    pkgs.plantuml
    pkgs.librsvg
  ];
  languages.texlive.enable = true;
  languages.texlive.packages = [
    "collection-basic"
    "collection-latex"
    "collection-pictures"
    "collection-fontutils"
    "collection-latexextra"
    "collection-latexrecommended"
    "collection-bibtexextra"
    "collection-mathscience"
    "standalone"
    "latexmk"
  ];
}
