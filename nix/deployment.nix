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
    ...
  }: let
    crossPkgs = {
      "x86_64-linux" = {
        "x86_64" = pkgs.pkgsStatic;
        "aarch64" = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic;
      };

      "aarch64-linux" = {
        "x86_64" = pkgs.pkgsCross.musl64;
        "aarch64" = pkgs.pkgsStatic;
      };

      "x86_64-darwin" = {
        "x86_64" = pkgs.pkgsCross.musl64;
        "aarch64" = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic;
      };

      "aarch64-darwin" = crossPkgs."x86_64-darwin";
    };

    teawieFor = arch: let
      inherit (crossPkgs.${system}.${arch}.llvmPackages.stdenv) cc;

      target = "${arch}-unknown-linux-musl";
      target' = builtins.replaceStrings ["-"] ["_"] target;
      targetUpper = lib.toUpper target';

      toolchain = with inputs'.fenix.packages;
        combine [
          minimal.cargo
          minimal.rustc
          targets.${target}.latest.rust-std
        ];

      naersk = inputs.naersk.lib.${system}.override {
        cargo = toolchain;
        rustc = toolchain;
      };
    in
      (config.packages.teawiebot.override {
        inherit naersk;
        lto = true;
        optimizeSize = true;
      })
      .overrideAttrs (new:
        lib.const {
          CARGO_BUILD_TARGET = target;
          "CC_${target'}" = "${cc}/bin/${cc.targetPrefix}cc";
          "CARGO_TARGET_${targetUpper}_RUSTFLAGS" = "-C target-feature=+crt-static";
          "CARGO_TARGET_${targetUpper}_LINKER" = new."CC_${target'}";
        });

    containerFor = arch:
      pkgs.dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest-${arch}";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [
          (lib.getExe (teawieFor arch))
        ];

        architecture = crossPkgs.${system}.${arch}.go.GOARCH;
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
