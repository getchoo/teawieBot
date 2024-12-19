{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
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
    in
    {
      checks = forAllSystems (
        system:

        let
          pkgs = nixpkgsFor.${system};

          mkCheck =
            name: nativeBuildInputs: script:
            pkgs.runCommand name { inherit nativeBuildInputs; } ''
              ${script} | tee $out
            '';
        in

        {
          actionlint = mkCheck "actionlint" [ pkgs.actionlint ] "actionlint ${self}/.github/workflows/*";
          deadnix = mkCheck "deadnix" [ pkgs.deadnix ] "deadnix check ${self}";
          nixfmt = mkCheck "nixfmt" [
            pkgs.nixfmt-rfc-style
          ] "find ${self} -type f -name '*.nix' | xargs nixfmt --check";
          rustfmt = mkCheck "rustfmt" [ pkgs.cargo pkgs.rustfmt ] "cd ${self} && cargo fmt -- --check";
          statix = mkCheck "statix" [ pkgs.statix ] "statix check ${self}";
        }
      );

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
              pkgs.nil
              pkgs.statix

              # misc formatter/linters
              pkgs.actionlint
              self.formatter.${system}

              pkgs.redis
            ];

            inputsFrom = [ self.packages.${system}.chill-discord-bot ];
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };

          ci = pkgs.mkShell {
            packages = [
              pkgs.clippy
              pkgs.rustfmt

              self.formatter.${system}
            ];

            inputsFrom = [ self.packages.${system}.chill-discord-bot ];
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

          staticWith = pkgs.callPackage ./nix/static.nix { inherit (packages') chill-discord-bot; };
          containerize = pkgs.callPackage ./nix/containerize.nix { };
        in
        {
          container-amd64 = containerize packages'.static-x86_64;
          container-arm64 = containerize packages'.static-aarch64;

          static-x86_64 = staticWith { arch = "x86_64"; };
          static-aarch64 = staticWith { arch = "aarch64"; };

          chill-discord-bot = pkgs.callPackage ./nix/derivation.nix { inherit self; };

          default = self.packages.${system}.chill-discord-bot;
        }
      );
    };
}
