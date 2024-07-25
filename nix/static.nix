{
  lib,
  fenix,
  pkgsCross,
  teawie-bot,
}:
let
  crossTargetFor = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };

  rustStdFor = lib.mapAttrs (
    _: pkgs: fenix.targets.${pkgs.stdenv.hostPlatform.rust.rustcTarget}.stable.rust-std
  ) crossTargetFor;

  toolchain = fenix.combine (
    [
      fenix.stable.cargo
      fenix.stable.rustc
    ]
    ++ lib.attrValues rustStdFor
  );

  crossPlatformFor = lib.mapAttrs (
    _: pkgs:
    pkgs.makeRustPlatform (
      lib.genAttrs [
        "cargo"
        "rustc"
      ] (_: toolchain)
    )
  ) crossTargetFor;
in
arch:
teawie-bot.override {
  rustPlatform = crossPlatformFor.${arch};
  optimizeSize = true;
}
