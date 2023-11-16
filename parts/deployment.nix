{
  inputs,
  self,
  ...
}: {
  flake.nixosModules.default = import ./module.nix self;

  perSystem = {
    lib,
    pkgs,
    system,
    config,
    inputs',
    ...
  }: let
    name = "getchoo/teawiebot";

    crossPkgsFor = lib.fix (finalAttrs: {
      "x86_64-linux" = {
        "amd64" = pkgs.pkgsStatic;
        "arm64v8" = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic;
      };

      "aarch64-linux" = {
        "amd64" = pkgs.pkgsCross.musl64;
        "arm64v8" = pkgs.pkgsStatic;
      };

      "x86_64-darwin" = {
        "amd64" = pkgs.pkgsCross.musl64;
        "arm64v8" = pkgs.pkgsCross.aarch64-multiplatform.pkgsStatic;
      };

      "aarch64-darwin" = finalAttrs."x86_64-darwin";
    });

    nativeArchFor = {
      "amd64" = "x86_64";
      "arm64v8" = "aarch64";
    };

    wieFor = arch: let
      target = "${nativeArchFor.${arch}}-unknown-linux-musl";
      target' = builtins.replaceStrings ["-"] ["_"] target;
      targetUpper = lib.toUpper target';

      toolchain = with inputs'.fenix.packages;
        combine [
          minimal.cargo
          minimal.rustc
          targets.${target}.latest.rust-std
        ];

      naersk' = inputs.naersk.lib.${system}.override {
        cargo = toolchain;
        rustc = toolchain;
      };

      teawiebot = config.packages.teawiebot.override {
        naersk = naersk';
        optimizeSize = true;
      };

      inherit (crossPkgsFor.${system}.${arch}.stdenv) cc;
    in
      lib.getExe (
        teawiebot.overrideAttrs (_:
          lib.fix (finalAttrs: {
            CARGO_BUILD_TARGET = target;
            "CC_${target'}" = "${cc}/bin/${cc.targetPrefix}cc";
            "CARGO_TARGET_${targetUpper}_RUSTFLAGS" = "-C target-feature=+crt-static";
            "CARGO_TARGET_${targetUpper}_LINKER" = finalAttrs."CC_${target'}";
          }))
      );

    toContainer = arch:
      assert lib.assertMsg (
        arch == "arch64" -> pkgs.stdenv.isLinux
      ) "aarch64 images are only supported on linux!";
        pkgs.dockerTools.buildLayeredImage {
          inherit name;
          tag = "latest-${arch}";
          contents = [pkgs.dockerTools.caCertificates];
          config.Cmd = [(wieFor arch)];

          architecture = crossPkgsFor.${system}.${arch}.go.GOARCH;
        };
  in {
    packages = {
      container-amd64 = toContainer "amd64";
      container-arm64v8 = toContainer "arm64v8";
    };
  };
}
