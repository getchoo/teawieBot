{
  perSystem = {
    lib,
    pkgs,
    inputs',
    teawiebot',
    ...
  }: let
    crossTargets = with pkgs.pkgsCross; {
      x86_64 = musl64.pkgsStatic;
      aarch64 = aarch64-multiplatform.pkgsStatic;
    };

    rustStdFor = pkgs: inputs'.fenix.packages.targets.${pkgs.stdenv.hostPlatform.rust.rustcTarget}.stable.rust-std;
    toolchain = with inputs'.fenix.packages;
      combine (lib.flatten [
        stable.cargo
        stable.rustc
        (map rustStdFor (lib.attrValues crossTargets))
      ]);

    rustPlatformFor = pkgs:
      pkgs.makeRustPlatform (
        lib.genAttrs ["cargo" "rustc"] (lib.const toolchain)
      );
    crossPlatforms = lib.mapAttrs (lib.const rustPlatformFor) crossTargets;

    buildTeawieWith = rustPlatform:
      teawiebot'.packages.teawiebot.override {
        inherit rustPlatform;
        optimizeSize = true;
      };
  in {
    packages = {
      teawiebot-static-x86_64 = buildTeawieWith crossPlatforms.x86_64;
      teawiebot-static-aarch64 = buildTeawieWith crossPlatforms.aarch64;
    };
  };
}
