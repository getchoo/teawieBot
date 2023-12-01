self: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.teawiebot;

  inherit
    (lib)
    getExe
    literalExpression
    mdDoc
    mkDefault
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    types
    ;
in {
  options.services.teawiebot = {
    enable = mkEnableOption "teawiebot";
    package = mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "teawiebot" {};

    redisUrl = mkOption {
      description = mdDoc ''
        Redis URL for teawieBot
      '';
      type = types.str;
      default = "unix:${config.services.redis.servers.teawiebot.unixSocket}";
      example = literalExpression ''
        "redis://localhost/"
      '';
    };

    environmentFile = mkOption {
      description = mdDoc ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
      type = types.nullOr types.path;
      default = null;
      example = literalExpression ''
        "/run/agenix.d/1/teawieBot"
      '';
    };
  };

  config = mkIf cfg.enable {
    services.redis.servers.teawiebot.enable = true;

    systemd.services."teawiebot" = {
      enable = true;
      wantedBy = mkDefault ["multi-user.target"];
      after = mkDefault ["network.target"];
      script = ''
        ${getExe cfg.package}
      '';

      serviceConfig = {
        Type = "simple";
        Restart = "always";

        EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;
        Environment = ["REDIS_URL=${cfg.redisUrl}"];

        # hardening
        DynamicUser = true;
        PrivateTmp = true;
        NoNewPrivileges = true;
        RestrictNamespaces = "uts ipc pid user cgroup";
        ProtectSystem = "strict";
        ProtectHome = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        PrivateDevices = true;
        RestrictSUIDSGID = true;
      };
    };
  };
}
