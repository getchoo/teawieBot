{
  inputs,
  self,
  ...
}: {
  perSystem = {
    craneLib,
    pkgs,
    system,
    src,
    toolchain,
    ...
  }: {
    checks = let
      commonArgs = {
        inherit src;
      };

      inherit (craneLib) cargoClippy cargoFmt;
    in {
      inherit (self.packages.${system}) teawiebot;

      clippy = cargoClippy (commonArgs
        // {
          inherit (self.packages.${system}) cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets";
        });

      fmt = cargoFmt commonArgs;

      pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
        inherit src;
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
