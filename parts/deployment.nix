{self, ...}: let
  bin = teawiebot-smol: "${teawiebot-smol}/bin/teawiebot";
  service = pkgs: cmd:
    pkgs.writeTextFile {
      name = "teawiebot.service";
      text = ''
        [Unit]
        Description=teawiebot service

        [Service]
        Environment="TOKEN="
        ExecStart="${cmd}"
        DynamicUser=yes
        ProtectSystem=strict
        ProtectHome=yes
        ProtectKernelTunables=yes
        ProtectKernelModules=yes
        ProtectControlGroups=yes
        SystemCallFilter=@system-service
        SystemCallErrorNumber=EPERM
        NoNewPrivileges=yes
        PrivateTmp=yes

        [Install]
        WantedBy=multi-user.target
      '';
    };
in {
  perSystem = {
    pkgs,
    system,
    ...
  }: let
    inherit (pkgs) cacert dockerTools portableService;
    inherit (self.packages.${system}) teawiebot teawiebot-smol;
    cmd = bin teawiebot-smol;
  in {
    packages = {
      container = dockerTools.buildLayeredImage {
        name = "teawiebot";
        tag = "latest";
        contents = [dockerTools.caCertificates];
        config.Cmd = ["${cmd}"];
      };

      service = portableService {
        inherit (teawiebot) pname;
        inherit (teawiebot-smol) version;
        description = "portable service for teawiebot!";
        units = [(service pkgs cmd)];
        symlinks = [
          {
            object = "${cacert}/etc/ssl";
            symlink = "/etc/ssl";
          }
        ];
      };
    };
  };

  flake = {
    nixosModules = {
      default = {
        config,
        lib,
        pkgs,
        ...
      }: let
        cfg = config.services.teawiebot;
        inherit (lib) mkEnableOption mkIf;
      in {
        options.services.teawiebot.enable = mkEnableOption "teawiebot";

        config.systemd.services = mkIf cfg.enable {
          teawiebot = {
            text = service pkgs (bin pkgs.teawiebot-smol);
          };
        };
      };
    };
  };
}
