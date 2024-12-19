{
  lib,
  pkgsCross,
}:

let
  crossPkgsFor = with pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };
in

{ arch }:

let
  crossPkgs = crossPkgsFor.${arch};
in

(crossPkgs.callPackage ./derivation.nix { optimizeSize = true; }).overrideAttrs (old: {
  passthru = old.passthru or { } // {
    inherit crossPkgs;
  };
})
