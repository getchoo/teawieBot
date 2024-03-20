{
  lib,
  pkgsStatic,
  fenix,
  naersk,
  teawiebot,
}: let
  inherit (pkgsStatic.stdenv) cc;

  target = pkgsStatic.stdenv.hostPlatform.config;
  target' = builtins.replaceStrings ["-"] ["_"] target;
  targetUpper = lib.toUpper target';

  toolchain = with fenix;
    combine [
      minimal.cargo
      minimal.rustc
      targets.${target}.latest.rust-std
    ];

  naersk' = naersk.override {
    cargo = toolchain;
    rustc = toolchain;
  };
in
  (teawiebot.override {
    naersk = naersk';
    lto = true;
    optimizeSize = true;
  })
  .overrideAttrs (new: old: {
    env = {
      "CC_${target'}" = "${cc}/bin/${cc.targetPrefix}cc";
      CARGO_BUILD_TARGET = target;
      CARGO_BUILD_RUSTFLAGS = old.env.CARGO_BUILD_RUSTFLAGS + " -C target-feature=+crt-static";
      "CARGO_TARGET_${targetUpper}_LINKER" = new.env."CC_${target'}";
    };
  })
