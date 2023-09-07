{self, ...}: {
  perSystem = {
    lib,
    pkgs,
    system,
    ...
  }: {
    packages = {
      container = pkgs.dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [(lib.getExe self.packages.${system}.teawiebot-smol)];
      };
    };
  };

  flake.nixosModules.default = import ./module.nix self;
}
