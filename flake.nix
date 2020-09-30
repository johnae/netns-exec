{
  description = "netns-exec - run a process in a Linux network namespace";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nix-misc = {
    url = "github:johnae/nix-misc";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, ... }@inputs:
    inputs.flake-utils.lib.simpleFlake {
      inherit self nixpkgs;
      name = "snowflake";
      preOverlays = [
        inputs.nix-misc.overlay
      ];
      systems = inputs.flake-utils.lib.defaultSystems;
      shell = ./shell.nix;
    }
  ;
}
