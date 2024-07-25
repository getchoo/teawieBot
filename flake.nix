{
  description = "teawie moment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    ## Everything below this is optional
    ## `inputs.<name>.follows = ""`

    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-analyzer-src.follows = "";
      };
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      treefmt-nix,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
      treefmtFor = forAllSystems (system: treefmt-nix.lib.evalModule nixpkgsFor.${system} ./treefmt.nix);
    in
    {
      checks = forAllSystems (system: {
        treefmt = treefmtFor.${system}.config.build.check self;
      });

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = [
              # rust tools
              pkgs.clippy
              pkgs.rustfmt
              pkgs.rust-analyzer

              # nix tools
              pkgs.deadnix
              pkgs.nil
              pkgs.statix

              # misc formatter/linters
              pkgs.actionlint
              self.formatter.${system}

              pkgs.redis
            ];

            inputsFrom = [ self.packages.${system}.teawie-bot ];
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };

          ci = pkgs.mkShell {
            packages = [
              pkgs.clippy
              pkgs.rustfmt

              self.formatter.${system}
            ];

            inputsFrom = [ self.packages.${system}.teawie-bot ];
          };
        }
      );

      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-rfc-style);

      nixosModules.default = import ./nix/module.nix self;

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          packages' = self.packages.${system};

          staticFor = pkgs.callPackage ./nix/static.nix {
            inherit (packages') teawie-bot;
            fenix = fenix.packages.${system};
          };

          containerize =
            teawie-bot:
            let
              architecture = teawie-bot.stdenv.hostPlatform.ubootArch;
            in
            pkgs.dockerTools.buildLayeredImage {
              name = "teawie-bot";
              tag = "latest-${architecture}";
              contents = [ pkgs.dockerTools.caCertificates ];
              config.Cmd = [ (lib.getExe teawie-bot) ];
              inherit architecture;
            };
        in
        {
          container-x86_64 = containerize packages'.static-x86_64;
          container-aarch64 = containerize packages'.static-aarch64;

          static-x86_64 = staticFor "x86_64";
          static-aarch64 = staticFor "aarch64";

          teawie-bot = pkgs.callPackage ./nix/derivation.nix { inherit self; };

          default = self.packages.${system}.teawie-bot;
        }
      );
    };
}
