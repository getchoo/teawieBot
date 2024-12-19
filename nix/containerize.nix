{ lib, dockerTools }:
let
  containerize =
    chill-discord-bot:
    let
      inherit (chill-discord-bot.passthru) crossPkgs;
      architecture = crossPkgs.go.GOARCH;
    in
    dockerTools.buildLayeredImage {
      name = "chill-discord-bot";
      tag = "latest-${architecture}";
      contents = [ dockerTools.caCertificates ];
      config.Cmd = [ (lib.getExe chill-discord-bot) ];
      inherit architecture;
    };
in
containerize
