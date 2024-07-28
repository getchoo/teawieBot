{
  lib,
  pkgsCross,
  rust-overlay,
  teawie-bot,
}:
let
  rustVersion = "1_79_0";

  crossTargetFor = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };

  toolchain = rust-overlay."rust_${rustVersion}".minimal.override {
    extensions = [ "rust-src" ];
    targets = lib.mapAttrsToList (_: pkgs: pkgs.stdenv.hostPlatform.rust.rustcTarget) crossTargetFor;
  };

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
