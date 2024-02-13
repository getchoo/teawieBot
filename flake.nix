{
  description = "teawie moment";

  nixConfig = {
    extra-substituters = ["https://cache.garnix.io"];
    extra-trusted-public-keys = ["cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="];
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

    pre-commit = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };

    procfile-nix = {
      url = "github:getchoo/procfile-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {parts, ...} @ inputs:
    parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.pre-commit.flakeModule
        inputs.procfile-nix.flakeModule

        ./nix/ci.nix
        ./nix/deployment.nix
        ./nix/dev.nix
        ./nix/packages.nix
      ];

      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
    };
}
