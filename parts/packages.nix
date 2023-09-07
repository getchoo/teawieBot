{self, ...}: {
  perSystem = {
    lib,
    pkgs,
    system,
    ...
  }: {
    packages = lib.fix (f: {
      teawiebot = pkgs.callPackage ./derivation.nix {inherit self;};
      teawiebot-smol = f.teawiebot.override {optimizeSize = true;};
      default = self.packages.${system}.teawiebot;
    });
  };
}
