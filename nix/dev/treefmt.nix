{
  perSystem = {
    treefmt = {
      projectRootFile = "flake.nix";

      programs = {
        alejandra.enable = true;
        deadnix.enable = true;
        rustfmt.enable = true;
      };

      settings.global = {
        excludes = [
          "./target"
          "./flake.lock"
          "./Cargo.lock"
        ];
      };
    };
  };
}
