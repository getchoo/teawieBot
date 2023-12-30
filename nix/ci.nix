{
  perSystem = {
    pkgs,
    lib,
    self',
    ...
  }: {
    /*
    require packages, checks, and devShells for ci to be considered a success

    also thanks DetSys for showing me i don't need to use runCommand, symlinkJoin, or linkFarm!
    https://determinate.systems/posts/hydra-deployment-source-of-truth
    */

    packages.ciGate = pkgs.writeText "ci-gate" (
      lib.concatMapStringsSep "\n" (s: toString (lib.attrValues s)) [
        self'.checks
        self'.devShells
        (builtins.removeAttrs self'.packages ["default" "ciGate"])
      ]
    );
  };
}
