{
  perSystem = {
    lib,
    pkgs,
    config,
    self',
    ...
  }: {
    pre-commit.settings = {
      hooks = {
        actionlint.enable = true;
        ${self'.formatter.pname}.enable = true;
        deadnix.enable = true;
        nil.enable = true;
        prettier.enable = true;
        rustfmt.enable = true;
        statix.enable = true;
      };
    };

    proc.groups.daemons.processes = {
      redis.command = lib.getExe' pkgs.redis "redis-server";
    };

    devShells = {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # general
          actionlint
          nodePackages_latest.prettier
          config.proc.groups.daemons.package

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

    formatter = pkgs.alejandra;
  };
}
