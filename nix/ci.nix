{
  perSystem = {
    pkgs,
    lib,
    config,
    ...
  }: {
    /*
    require packages, checks, and devShells for ci to be considered a success

    also thanks DetSys for showing me i don't need to use runCommand, symlinkJoin, or linkFarm!
    https://determinate.systems/posts/hydra-deployment-source-of-truth
    */

    packages.ciGate = pkgs.writeText "ci-gate" ''
      ${
        lib.concatMapStringsSep "\n" (s: toString (builtins.attrValues s)) [
          config.checks
          config.devShells
          (builtins.removeAttrs config.packages ["default" "ciGate"])
        ]
      }
    '';
  };
}
