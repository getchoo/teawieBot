{
  inputs,
  flake-parts-lib,
  withSystem,
  ...
}: {
  flake.nixosModules.default = flake-parts-lib.importApply ./module.nix {
    inherit withSystem;
  };

  perSystem = {
    lib,
    pkgs,
    system,
    config,
    inputs',
    self',
    ...
  }: let
    crossPkgs = with pkgs.pkgsCross; {
      x86_64 = musl64;
      aarch64 = aarch64-multiplatform.pkgsStatic;
    };

    teawieFor = arch:
      pkgs.callPackage ./static.nix {
        inherit (self'.packages) teawiebot;
        pkgsStatic = crossPkgs.${arch};
        fenix = inputs'.fenix.packages;
        naersk = inputs.naersk.lib.${system};
      };

    containerFor = arch:
      pkgs.dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest-${arch}";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [
          (lib.getExe self'.packages."teawiebot-static-${arch}")
        ];

        architecture = crossPkgs.${arch}.go.GOARCH;
      };
  in {
    packages = {
      teawiebot-static-x86_64 = teawieFor "x86_64";
      teawiebot-static-aarch64 = teawieFor "aarch64";
      container-x86_64 = containerFor "x86_64";
      container-aarch64 = containerFor "aarch64";
    };
  };
}
