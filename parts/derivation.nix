{
  lib,
  rustPlatform,
  self,
  lto ? true,
  optimizeSize ? false,
}: let
  filter = path: type: let
    path' = toString path;
    base = baseNameOf path';
    parent = baseNameOf (dirOf path');

    dirBlocklist = ["parts"];

    matches = lib.any (suffix: lib.hasSuffix suffix base) [".rs"];
    isCargo = base == "Cargo.lock" || base == "Cargo.toml";
    isCopypasta = parent == "copypastas";
    isAllowedDir = !(builtins.elem base dirBlocklist);
  in
    (type == "directory" && isAllowedDir) || matches || isCargo || isCopypasta;

  filterSource = src:
    lib.cleanSourceWith {
      src = lib.cleanSource src;
      inherit filter;
    };
in
  rustPlatform.buildRustPackage {
    pname = "teawiebot";
    version = builtins.substring 0 8 self.lastModifiedDate or "dirty";

    src = filterSource self;

    cargoLock.lockFile = ../Cargo.lock;

    RUSTFLAGS =
      lib.optionalString lto " -C lto=thin -C embed-bitcode=yes"
      + lib.optionalString optimizeSize " -C codegen-units=1 -C strip=symbols -C opt-level=z";

    meta = with lib; {
      mainProgram = "teawiebot";
      description = "funni bot";
      homepage = "https://github.com/getchoo/teawiebot";
      license = licenses.mit;
      platforms = with platforms; unix;
      maintainers = with maintainers; [getchoo];
    };
  }
