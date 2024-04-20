{
  perSystem = {
    config,
    pkgs,
    self',
    teawiebot',
    ...
  }: {
    devShells = {
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
          self'.formatter

          config.procfiles.daemons.package
        ];

        inputsFrom = [teawiebot'.packages.teawiebot];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };

      ci = pkgs.mkShell {
        packages = [
          pkgs.clippy
          pkgs.rustfmt

          self'.formatter
        ];

        inputsFrom = [teawiebot'.packages.teawiebot];
      };
    };
  };
}
