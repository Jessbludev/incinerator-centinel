{ config, lib, pkgs, ... }:

let
  cfg = config.services.incinerator-sentinel;
in {
  options.services.incinerator-sentinel = {
    enable = lib.mkEnableOption "Incinerator Sentinel";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.incinerator-sentinel;
      description = "Paquete de incinerator-sentinel a ejecutar.";
    };

    policyFile = lib.mkOption {
      type = lib.types.path;
      description = "Ruta al policy bundle firmado.";
    };

    configFile = lib.mkOption {
      type = lib.types.path;
      description = "Ruta al archivo de configuracion del servicio.";
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.incinerator-sentinel = {
      description = "Incinerator Sentinel";
      after = [ "network-pre.target" ];
      wants = [ "network-pre.target" ];
      wantedBy = [ "multi-user.target" ];

      serviceConfig = {
        Type = "notify";
        ExecStart = ''
          ${cfg.package}/bin/incinerator-sentinel serve \
            --config ${cfg.configFile} \
            --policy ${cfg.policyFile}
        '';
        Restart = "always";
        RestartSec = "1s";
        TimeoutStopSec = "5s";

        DynamicUser = true;
        NoNewPrivileges = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        RestrictNamespaces = true;
        MemoryDenyWriteExecute = true;
        SystemCallFilter = [ "@system-service" ];
        SystemCallErrorNumber = "EPERM";
        CapabilityBoundingSet = [ "" ];
        AmbientCapabilities = [ "" ];
        DevicePolicy = "strict";
        UMask = "0077";
        LimitCORE = 0;
      };
    };
  };
}
