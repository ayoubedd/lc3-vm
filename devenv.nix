{ pkgs, lib, config, inputs, ... }:
let 
  files = [ "rogue" "2028" "hello-world" ];
  runner = file: "lc3as ./examples/${file} && cargo run -- ./examples/${builtins.baseNameOf file}.obj";
  scripts = builtins.listToAttrs (map (file: { name = file; value = { exec = runner file; }; }) files);
in
{
  inherit scripts;
  packages = with pkgs; [ lc3tools ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
