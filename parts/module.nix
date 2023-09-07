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
    mkDefault
    mkDoc
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
    environmentFile = mkOption {
      description = mkDoc ''
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
    systemd.services = {
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
