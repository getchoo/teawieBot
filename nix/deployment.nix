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
    containerFor = arch: let
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
      inherit (crossPkgs.${system}.${arch}.stdenv) cc;

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

      teawiebot =
        (config.packages.teawiebot.override {
          inherit naersk;
          optimizeSize = true;
        })
        .overrideAttrs (new:
          lib.const {
            CARGO_BUILD_TARGET = target;
            "CC_${target'}" = "${cc}/bin/${cc.targetPrefix}cc";
            "CARGO_TARGET_${targetUpper}_RUSTFLAGS" = "-C target-feature=+crt-static";
            "CARGO_TARGET_${targetUpper}_LINKER" = new."CC_${target'}";
          });
    in
      pkgs.dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest-${arch}";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [(lib.getExe teawiebot)];
      };
  in {
    packages = {
      container-x86_64 = containerFor "x86_64";
      container-aarch64 = containerFor "aarch64";
    };
  };
}
