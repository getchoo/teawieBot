{
  pkgs,
  arch,
  inputs,
}:
let
  inherit (pkgs) lib;
  inputs' = lib.mapAttrs (_: lib.mapAttrs (_: v: v.${pkgs.system} or v)) inputs;

  crossTargets = with pkgs.pkgsCross; {
    x86_64 = musl64.pkgsStatic;
    aarch64 = aarch64-multiplatform.pkgsStatic;
  };

  rustStdFor =
    pkgs: inputs'.fenix.packages.targets.${pkgs.stdenv.hostPlatform.rust.rustcTarget}.stable.rust-std;
  toolchain =
    with inputs'.fenix.packages;
    combine (
      lib.flatten [
        stable.cargo
        stable.rustc
        (map rustStdFor (lib.attrValues crossTargets))
      ]
    );

  rustPlatformFor =
    pkgs:
    pkgs.makeRustPlatform (
      lib.genAttrs [
        "cargo"
        "rustc"
      ] (lib.const toolchain)
    );
  crossPlatforms = lib.mapAttrs (lib.const rustPlatformFor) crossTargets;
in
{
  "teawiebot-static-${arch}" = inputs'.self.packages.teawiebot.override {
    rustPlatform = crossPlatforms.${arch};
    optimizeSize = true;
  };

  "container-${arch}" = pkgs.dockerTools.buildLayeredImage {
    name = "teawiebot";
    tag = "latest-${arch}";
    contents = [ pkgs.dockerTools.caCertificates ];
    config.Cmd = [ (lib.getExe inputs'.self.packages."teawiebot-static-${arch}") ];

    architecture = inputs.nixpkgs.legacyPackages."${arch}-linux".pkgsStatic.go.GOARCH;
  };
}
