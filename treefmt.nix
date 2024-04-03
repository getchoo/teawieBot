{
  projectRootFile = "flake.nix";

  programs = {
    alejandra.enable = true;
    deadnix.enable = true;
    prettier.enable = true;
    rustfmt.enable = true;
  };

  settings.global = {
    excludes = [
      "./target"
      "./flake.lock"
      "./Cargo.lock"
    ];
  };
}
