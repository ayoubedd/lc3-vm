{ pkgs, lib, config, inputs, ... }:

{
  packages = with pkgs; [ lc3tools ];

  scripts.rc.exec = "lc3as examples/hello-world.asm && cargo run -- examples/hello-world.obj";

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
