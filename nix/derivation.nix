{
  lib,
  stdenv,
  naersk,
  darwin,
  self,
  lto ? false,
  optimizeSize ? false,
}:
naersk.buildPackage {
  pname = "teawiebot";
  version =
    toString (lib.importTOML ../Cargo.toml).package.version
    + "-${self.shortRev or self.dirtyShortRev or "dirty"}";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../Cargo.toml
      ../Cargo.lock
      ../build.rs
    ];
  };

  buildInputs = lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
    CoreFoundation
    Security
    SystemConfiguration
  ]);

  env = {
    GIT_SHA = self.shortRev or self.dirtyShortRev or "unknown-dirty";
    CARGO_BUILD_RUSTFLAGS = lib.concatStringsSep " " (
      lib.optionals lto [
        "-C"
        "lto=thin"
        "-C"
        "embed-bitcode=yes"
        "-Zdylib-lto"
      ]
      ++ lib.optionals optimizeSize [
        "-C"
        "codegen-units=1"
        "-C"
        "panic=abort"
        "-C"
        "strip=symbols"
        "-C"
        "opt-level=z"
      ]
    );
  };

  meta = with lib; {
    mainProgram = "teawiebot";
    description = "funni bot";
    homepage = "https://github.com/getchoo/teawiebot";
    license = licenses.mit;
    maintainers = with maintainers; [getchoo];
  };
}
