{self, ...}: {
  perSystem = {
    craneLib,
    pkgs,
    src,
    system,
    ...
  }: let
    inherit (pkgs.lib) licenses maintainers platforms;
    inherit (craneLib) buildPackage;
  in {
    packages = {
      cargoArtifacts = craneLib.buildDepsOnly {inherit src;};

      teawiebot = buildPackage {
        inherit src;
        inherit (self.packages.${system}) cargoArtifacts;

        meta = {
          description = "funni bot";
          homepage = "https://github.com/getchoo/teawiebot";
          license = licenses.mit;
          platforms = platforms.unix;
          maintainers = with maintainers; [getchoo];
        };
      };

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
