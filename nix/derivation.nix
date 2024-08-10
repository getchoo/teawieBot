{
  lib,
  stdenv,
  rustPlatform,
  darwin,
  self ? { },
  lto ? true,
  optimizeSize ? false,
}:
let
  fs = lib.fileset;
in
rustPlatform.buildRustPackage {
  pname = "teawie-bot";
  version = (lib.importTOML ../Cargo.toml).package.version or "unknown";

  src = fs.toSource {
    root = ../.;
    fileset = fs.intersection (fs.gitTracked ../.) (
      lib.fileset.unions [
        ../src
        ../Cargo.toml
        ../Cargo.lock
      ]
    );
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = lib.optionals stdenv.isDarwin (
    with darwin.apple_sdk.frameworks;
    [
      CoreFoundation
      Security
      SystemConfiguration
      darwin.libiconv
    ]
  );

  env =
    let
      toRustFlags = lib.mapAttrs' (
        name:
        lib.nameValuePair "CARGO_BUILD_RELEASE_${
          lib.toUpper (builtins.replaceStrings [ "-" ] [ "_" ] name)
        }"
      );
    in
    {
      GIT_SHA = self.shortRev or self.dirtyShortRev or "unknown";
    }
    // lib.optionalAttrs lto (toRustFlags {
      lto = "thin";
    })
    // lib.optionalAttrs optimizeSize (toRustFlags {
      codegen-units = 1;
      opt-level = "s";
      panic = "abort";
      strip = "symbols";
    });

  meta = {
    description = "funni bot";
    homepage = "https://github.com/getchoo/teawiebot";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ getchoo ];
    mainProgram = "teawie-bot";
  };
}
