{withSystem, ...}: {
  perSystem = {
    lib,
    pkgs,
    self',
    ...
  }: let
    containerFor = arch:
      pkgs.dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest-${arch}";
        contents = [pkgs.dockerTools.caCertificates];
        config.Cmd = [
          (lib.getExe self'.packages."teawiebot-static-${arch}")
        ];

        architecture = withSystem "${arch}-linux" ({pkgs, ...}: pkgs.pkgsStatic.go.GOARCH);
      };
  in {
    packages = {
      container-x86_64 = containerFor "x86_64";
      container-aarch64 = containerFor "aarch64";
    };
  };
}
