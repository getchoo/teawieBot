{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    get-flake.url = "github:ursi/get-flake";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    procfile-nix = {
      url = "github:getchoo/procfile-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [
        # dev utils
        ./checks.nix
        ./procfile.nix
        ./shell.nix
        ./treefmt.nix

        # special, private builds
        ./docker.nix
        ./static.nix

        inputs.treefmt-nix.flakeModule
        inputs.procfile-nix.flakeModule
      ];

      perSystem = {
        lib,
        system,
        ...
      }: {
        _module.args = {
          teawiebot' = lib.mapAttrs (lib.const (v: v.${system} or v)) (inputs.get-flake ../../.);
        };
      };
    };
}
