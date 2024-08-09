{ lib, dockerTools }:
let
  containerize =
    teawie-bot:
    let
      inherit (teawie-bot.passthru) crossPkgs;
      architecture = crossPkgs.go.GOARCH;
    in
    dockerTools.buildLayeredImage {
      name = "teawie-bot";
      tag = "latest-${architecture}";
      contents = [ dockerTools.caCertificates ];
      config.Cmd = [ (lib.getExe teawie-bot) ];
      inherit architecture;
    };
in
containerize
