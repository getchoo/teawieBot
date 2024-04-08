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
        packages = with pkgs; [
          # rust tools
          clippy
          rustfmt
          rust-analyzer

          # misc formatter/linters
          actionlint
          self'.formatter
          nil
          statix

          config.procfiles.daemons.package
        ];

        inputsFrom = [teawiebot'.packages.teawiebot];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };

      ci = pkgs.mkShell {
        packages = with pkgs; [
          clippy
          rustfmt
          self.formatter.${system}
        ];

        inputsFrom = [teawiebot'.packages.teawiebot];
      };
    };
  };
}
