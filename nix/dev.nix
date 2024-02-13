{
  perSystem = {
    lib,
    pkgs,
    config,
    self',
    ...
  }: let
    enableAll = lib.flip lib.genAttrs (lib.const {enable = true;});
  in {
    treefmt = {
      projectRootFile = "flake.nix";

      programs = enableAll [
        "alejandra"
        "deadnix"
        "prettier"
        "rustfmt"
      ];

      settings.global = {
        excludes = [
          "./target"
          "./flake.lock"
          "./Cargo.lock"
        ];
      };
    };

    pre-commit.settings = {
      settings.treefmt.package = config.treefmt.build.wrapper;

      hooks = enableAll [
        "actionlint"
        "nil"
        "statix"
        "treefmt"
      ];
    };

    procfiles.daemons.processes = {
      redis = lib.getExe' pkgs.redis "redis-server";
    };

    devShells = {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # general
          actionlint
          nodePackages_latest.prettier
          config.procfiles.daemons.package

          # rust
          cargo
          rustc
          clippy
          rustfmt
          rust-analyzer

          # nix
          self'.formatter
          deadnix
          nil
          statix
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    };
  };
}
