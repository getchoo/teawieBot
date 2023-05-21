{inputs, ...}: {
  perSystem = {system, ...}: let
    pkgs = import inputs.nixpkgs {
      inherit system;
      overlays = [inputs.fenix.overlays.default];
    };

    toolchain = with pkgs.fenix;
    with stable;
      combine [
        cargo
        rustc
        rustfmt
        clippy
        targets."x86_64-unknown-linux-musl".stable.rust-std
      ];
  in {
    _module.args = {
      inherit pkgs toolchain;

      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;
    };
  };
}
