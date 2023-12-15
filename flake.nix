{
  description = "teawie moment";

  nixConfig = {
    extra-substituters = ["https://cache.mydadleft.me/teawiebot"];
    extra-trusted-public-keys = ["teawiebot:vp7AaQ042O/3326DMMtLF4MOUa5/kCBAq+YApy5GWXA="];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    proc-flake.url = "github:srid/proc-flake";
    flake-root.url = "github:srid/flake-root";

    nix2workflow = {
      url = "github:getchoo/nix2workflow";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };
  };

  outputs = {parts, ...} @ inputs:
    parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.pre-commit.flakeModule

        inputs.proc-flake.flakeModule
        inputs.flake-root.flakeModule

        inputs.nix2workflow.flakeModule

        ./parts/deployment.nix
        ./parts/dev.nix
        ./parts/packages.nix
        ./parts/workflow.nix
      ];

      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
    };
}
