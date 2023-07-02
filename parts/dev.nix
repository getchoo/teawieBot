{
  inputs,
  self,
  ...
}: {
  perSystem = {
    craneLib,
    pkgs,
    system,
    toolchain,
    ...
  }: {
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

      pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
        src = self;
        hooks = {
          actionlint.enable = true;
          alejandra.enable = true;
          deadnix.enable = true;
          nil.enable = true;
          statix.enable = true;
        };
      };
    };

    devShells = let
      inherit (pkgs) mkShell;
    in {
      default = mkShell {
        inherit (self.checks.${system}.pre-commit-check) shellHook;
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
