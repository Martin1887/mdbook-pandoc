{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;
  languages.rust.channel = "stable";
  packages = [
    pkgs.vscode-extensions.vadimcn.vscode-lldb.adapter
    pkgs.taplo
  ];
}
