{
  projectRootFile = ".git/config";

  programs = {
    actionlint.enable = true;
    deadnix.enable = true;
    nixfmt.enable = true;
    rustfmt.enable = true;
    statix.enable = true;
  };
}
