{
  inputs,
  self,
  ...
}: {
  perSystem = {
    config,
    craneLib,
    pkgs,
    system,
    toolchain,
    ...
  }: {
    pre-commit = {
      settings.hooks = {
        actionlint.enable = true;
        alejandra.enable = true;
        deadnix.enable = true;
        nil.enable = true;
        statix.enable = true;
      };
    };

    checks = let
      inherit (craneLib) cargoAudit cargoClippy cleanCargoSource cargoFmt;

      commonArgs = {
        src = cleanCargoSource self;
      };
    in {
      inherit (self.packages.${system}) teawiebot;

      audit = cargoAudit (commonArgs // {inherit (inputs) advisory-db;});

      clippy = cargoClippy (commonArgs
        // {
          inherit (self.packages.${system}) cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets";
        });

      fmt = cargoFmt commonArgs;
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

          toolchain
        ];
      };
    };

    formatter = pkgs.alejandra;
  };
}
