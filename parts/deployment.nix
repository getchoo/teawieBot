{self, ...}: {
  perSystem = {
    lib,
    pkgs,
    self',
    ...
  }: {
    packages = {
      container = pkgs.dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [(lib.getExe self'.packages.teawiebot-smol)];
      };
    };
  };

  flake.nixosModules.default = import ./module.nix self;
}
