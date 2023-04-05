{
  description = "teawie moment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
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
    supportedSystems = with flake-utils.lib.system; [
      x86_64-linux
      x86_64-darwin
      aarch64-linux
      aarch64-darwin
    ];
  in
    flake-utils.lib.eachSystem supportedSystems (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages = let
        inherit (pkgs.lib) maintainers licenses;
        inherit (pkgs.dockerTools) buildLayeredImage caCertificates;
        inherit (pkgs.rustPlatform) buildRustPackage;
      in
        rec {
          teawiebot = buildRustPackage {
            pname = "teawiebot";
            inherit version;

            src = ./.;

            RUSTFLAGS = "-C lto=thin -C embed-bitcode=yes";
            cargoSha256 = "sha256-TQThvhD2psA5+VGSMl3+dBOs8K33Fs5q42RovXnYYhY=";

            buildInputs = with pkgs; [
              openssl.dev
            ];
            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            meta = {
              description = "funni bot";
              homepage = "https://github.com/getchoo/teawiebot";
              license = licenses.mit;
              maintainers = with maintainers; [getchoo];
            };
          };
          container = let
            bot = teawiebot.overrideAttrs (prev: {
              RUSTFLAGS = prev.RUSTFLAGS + " -C opt-level=s";
            });
          in
            buildLayeredImage {
              name = "teawiebot";
              tag = "latest";
              contents = [caCertificates];
              config.Cmd = ["${bot}/bin/teawiebot"];
            };
        }
        // {default = self.packages.${system}.teawiebot;};

      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            actionlint.enable = true;
            alejandra.enable = true;
            cargo-check.enable = true;
            deadnix.enable = true;
            rustfmt.enable = true;
            statix.enable = true;
          };
        };
      };

      devShells = let
        inherit (pkgs) mkShell;
      in {
        default = mkShell {
          packages = with pkgs; [
            actionlint
            alejandra
            cargo
            clippy
            deadnix
            openssl.dev
            pkg-config
            rustfmt
            statix
          ];
          inherit (self.checks.${system}.pre-commit-check) shellHook;
        };
      };

      formatter = pkgs.alejandra;
    });
}
