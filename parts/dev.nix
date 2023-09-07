{
  perSystem = {
    config,
    pkgs,
    ...
  }: {
    pre-commit = {
      settings.hooks = {
        actionlint.enable = true;
        alejandra.enable = true;
        deadnix.enable = true;
        nil.enable = true;
        rustfmt.enable = true;
        statix.enable = true;
      };
    };

    devShells = {
      default = pkgs.mkShell {
        shellHook = config.pre-commit.installationScript;
        packages = with pkgs; [
          actionlint
          alejandra
          deadnix
          nil
          statix

          rustc
          cargo
          rustfmt
          clippy
        ];
      };
    };

    formatter = pkgs.alejandra;
  };
}
