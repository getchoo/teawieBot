{
  lib,
  rustPlatform,
  self ? { },
  lto ? true,
  optimizeSize ? false,
}:

let
  fs = lib.fileset;
in

rustPlatform.buildRustPackage {
  pname = "chill-discord-bot";
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

  cargoLock.lockFile = ../Cargo.lock;

  RUSTFLAGS =
    lib.optionals lto [
      "-C"
      "lto=thin"
    ]
    ++ lib.optionals optimizeSize [
      "-C"
      "codegen-units=1"
      "-C"
      "opt-level=s"
      "-C"
      "panic=abort"
      "-C"
      "strip=symbols"
    ];

  GIT_SHA = self.shortRev or self.dirtyShortRev or "unknown";

  meta = {
    description = "funni bot";
    homepage = "https://github.com/getchoo/chill";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ getchoo ];
    mainProgram = "chill";
  };
}
