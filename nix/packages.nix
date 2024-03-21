{self, ...}: {
  perSystem = {
    pkgs,
    self',
    ...
  }: {
    packages = {
      teawiebot = pkgs.callPackage ./derivation.nix {inherit self;};
      default = self'.packages.teawiebot;
    };
  };
}
