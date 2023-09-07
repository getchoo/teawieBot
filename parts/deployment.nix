{self, ...}: {
  perSystem = {
    lib,
    pkgs,
    system,
    ...
  }: let
    inherit (pkgs) dockerTools;
    inherit (self.packages.${system}) teawiebot-smol;
  in {
    packages = {
      container = dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest";
        contents = [dockerTools.caCertificates];
        config.Cmd = ["${lib.getExe teawiebot-smol}"];
      };
    };
  };

  flake.nixosModules.default = import ./module.nix self;
}
