{
  perSystem = {
    lib,
    pkgs,
    ...
  }: {
    checks = {
      actionlint = pkgs.runCommand "check-actionlint" {} ''
        ${lib.getExe pkgs.actionlint} ${../../.github/workflows}/*
        touch $out
      '';

      editorconfig = pkgs.runCommand "check-editorconfig" {} ''
        ${lib.getExe pkgs.editorconfig-checker} \
          -exclude '.git' ${../../.}
      '';

      statix = pkgs.runCommand "check-statix" {} ''
        ${lib.getExe pkgs.statix} check ${../../.}
        touch $out
      '';
    };
  };
}
