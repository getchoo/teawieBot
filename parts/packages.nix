{self, ...}: {
  perSystem = {
    pkgs,
    self',
    ...
  }: {
    packages = {
      teawiebot = pkgs.callPackage ./derivation.nix {inherit self;};
      teawiebot-smol = self'.packages.teawiebot.override {optimizeSize = true;};
      default = self'.packages.teawiebot;
    };
  };
}
