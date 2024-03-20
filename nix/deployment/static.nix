{
  perSystem = {
    lib,
    pkgs,
    inputs',
    self',
    ...
  }: let
    targets = with pkgs.pkgsCross; {
      x86_64 = musl64.pkgsStatic;
      aarch64 = aarch64-multiplatform.pkgsStatic;
    };

    toolchain = let
      fenix = inputs'.fenix.packages;
    in
      with fenix;
        combine (
          [minimal.cargo minimal.rustc]
          ++ map (
            pkgs:
              fenix.targets.${pkgs.stdenv.hostPlatform.config}.latest.rust-std
          ) (lib.attrValues targets)
        );

    rustPlatforms =
      lib.mapAttrs (
        lib.const (pkgs:
          pkgs.makeRustPlatform (
            lib.genAttrs ["cargo" "rustc"] (lib.const toolchain)
          ))
      )
      targets;

    buildTeawieWith = rustPlatform:
      self'.packages.teawiebot.override {
        inherit rustPlatform;
        lto = true;
        optimizeSize = true;
      };
  in {
    packages = lib.optionalAttrs pkgs.stdenv.isLinux (
      lib.mapAttrs' (
        target: rustPlatform:
          lib.nameValuePair "teawiebot-static-${target}" (buildTeawieWith rustPlatform)
      )
      rustPlatforms
    );
  };
}
