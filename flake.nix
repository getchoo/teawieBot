{
  description = "teawie moment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
      inputs.flake-compat.follows = "flake-compat";
      inputs.flake-utils.follows = "flake-utils";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    pre-commit-hooks,
    flake-utils,
    ...
  }: let
    version = "0.0.1";
    supportedSystems = with flake-utils.lib.system; [x86_64-linux x86_64-darwin aarch64-linux aarch64-darwin];
  in
    flake-utils.lib.eachSystem supportedSystems (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages =
        {
          teawiebot = with pkgs;
            python39Packages.buildPythonPackage {
              pname = "teawiebot";
              inherit version;
              src = ./.;
              format = "flit";
              propagatedBuildInputs = with pkgs.python39Packages; [hatchling discordpy requests];
            };
          container = with pkgs.dockerTools;
            buildImage {
              name = "teawiebot";
              tag = "latest";
              copyToRoot = [caCertificates];
              config.Cmd = ["${self.packages.${system}.teawiebot}/bin/teawiebot"];
            };
        }
        // {default = self.packages.${system}.teawiebot;};

      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            isort.enable = true;
            pylint.enable = true;
            yapf = {
              enable = true;
              name = "yapf";
              entry = "${pkgs.python39Packages.yapf}/bin/yapf -i";
              types = ["file" "python"];
            };
          };
        };
      };

      devShells = with pkgs; {
        default = mkShell {
          packages = with pkgs.python39Packages; [python39 discordpy flit pylint requests toml yapf];
          inherit (self.checks.${system}.pre-commit-check) shellHook;
        };
      };
    });
}
