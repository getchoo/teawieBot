{
  lib,
  stdenv,
  rustPlatform,
  darwin,
  self ? {
    inherit ((lib.importTOML ../Cargo.toml).package) version;
  },
  lto ? true,
  optimizeSize ? false,
}:
rustPlatform.buildRustPackage {
  pname = "teawiebot";
  version =
    (lib.importTOML ../Cargo.toml).package.version
    + "-"
    + self.shortRev or self.dirtyShortRev or self.version or "unknown";

  __structuredAttrs = true;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../Cargo.toml
      ../Cargo.lock
    ];
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

  meta = with lib; {
    mainProgram = "teawiebot";
    description = "funni bot";
    homepage = "https://github.com/getchoo/teawiebot";
    license = licenses.mit;
    maintainers = with maintainers; [ getchoo ];
  };
}
