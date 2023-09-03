{
  description = "teawie moment";

  nixConfig = {
    extra-substituters = [
      "https://cache.garnix.io"
    ];
    extra-trusted-public-keys = [
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    # used for cargo audit
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    # our build framework
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-compat.follows = "compat";
      inputs.flake-utils.follows = "utils";
    };

    # toolchain management
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
      inputs.flake-compat.follows = "compat";
      inputs.flake-utils.follows = "utils";
    };

    # this is just to avoid having multiple versions in flake.lock
    compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    # ditto
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    parts,
    pre-commit,
    ...
  } @ inputs:
    parts.lib.mkFlake {inherit inputs;} {
      imports = [
        pre-commit.flakeModule
        ./parts
      ];
    };
}
