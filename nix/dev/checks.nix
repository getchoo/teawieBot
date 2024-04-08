{
  perSystem = {
    lib,
    pkgs,
    ...
  }: {
    checks = {
      actionlint = pkgs.runCommand "check-actionlint" {} ''
        ${lib.getExe pkgs.actionlint} ${./.github/workflows}/*
        touch $out
      '';

      nil = pkgs.runCommand "check-nil" {nativeBuildInputs = [pkgs.findutils pkgs.nil];} ''
        find ${./.} -type f -name '*.nix' | while read -r file; do
          nil diagnostics "$file"
        done
        touch $out
      '';

      statix = pkgs.runCommand "check-statix" {} ''
        ${lib.getExe pkgs.statix} check ${./.}
        touch $out
      '';
    };
  };
}
