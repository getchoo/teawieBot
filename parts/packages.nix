{self, ...}: {
  perSystem = {
    craneLib,
    pkgs,
    system,
    ...
  }: {
    packages = {
      cargoArtifacts = craneLib.buildDepsOnly {src = craneLib.cleanCargoSource self;};

      teawiebot = pkgs.callPackage ./derivation.nix {inherit craneLib self;};

      teawiebot-smol =
        self.packages.${system}.teawiebot.overrideAttrs (_: {
            # statically link musl, optimize for size
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";

            CARGO_BUILD_RUSTFLAGS = "-C lto=fat -C embed-bitcode=yes \
									-C target-feature=+crt-static -C opt-level=z -C strip=symbols -C codegen-units=1";

            CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = let
              inherit (pkgs.pkgsStatic.stdenv) cc;
            in "${cc}/bin/${cc.targetPrefix}cc";
          });

      default = self.packages.${system}.teawiebot;
    };
  };
}
