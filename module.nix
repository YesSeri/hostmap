{
  pkgs,
  config,
  lib,
  ...
}:
let
  cfg = config.services.hostmap;
  vhostName = config.networking.hostName;
  activationLogFolder = "/var/log/hostmap-activation-logs";
  activationLogFile = activationLogFolder + "/hostmap-activation-logs.csv";
  activationLoggerUrl = "/hostmap/hostmap-activation-logs.csv";
in
{
  options.services.hostmap = with lib; {
    activationLogger = {
      enable = mkEnableOption "activation logger";
      port = mkOption {
        type = types.port;
        description = "port for activation logger to serve on";
      };
    };
    scraper = {
      enable = mkEnableOption "hostmap scraper";
      targetHosts = mkOption {
        type = types.listOf (
          types.submodule {
            options = {
              hostname = lib.mkOption {
                type = types.str;
                description = "name of host";
              };
              host_url = lib.mkOption {
                type = types.str;
                description = "url of host";
              };
              metadata = lib.mkOption {
                type = types.attrsOf types.str;
                default = { };
                description = "metadata (keys are strings and values are strings).";
              };
            };
          }
        );
        default = [ ];
        example = [
          {
            hostname = "foo";
            host_url = "foo.pzz.dk";
            metadata = {
              host_group_name = "hosts-prod";
              environment = "production";
            };
          }
        ];
        description = "list of host objects to scrape.";
      };
      concurrentRequests = mkOption {
        type = types.int;
        default = 8;
        description = "number of servers scraped concurrently";
      };
      scrapeInterval = mkOption {
        type = types.int;
        default = 15;
        description = "seconds spent waiting between each scrape attempts. servers are scraped concurrently 8 at a time by default";
      };
      apiKeyFile = mkOption {
        type = types.str;
        description = "path to file with api key";
      };
      serverUrl = mkOption {
        type = types.str;
        default = "http://127.0.0.1:3000";
        description = "the servers url that the scraper will send data to";
      };
      activationLoggerPort = mkOption {
        type = types.port;
        description = "port to scrape the activation logger on server to listen on";
      };
    };
    server = {
      enable = mkEnableOption "hostmap server";
      databaseUrl = mkOption {
        type = types.str;
        default = "postgres://hostmap@127.0.0.1:5432/hostmap";
        description = "a database url for the server";
      };
      repoUrl = mkOption {
        type = types.str;
        description = "url to the git remote repo, e.g. https://gitlab.dbc.dk/platform/deployments/-/commit";
      };

      columns = mkOption {
        type = types.listOf types.str;
        default = [ ];
        description = "columns to show from metadata";
      };
      groupingKey = mkOption {
        type = types.str;
        description = "the default grouping of the hosts on index page";
      };
      port = mkOption {
        type = types.port;
        default = 3000;
        description = "port for server to listen on";
      };
      apiKeyFile = mkOption {
        type = types.str;
        description = "path to file with api key";
      };
      timeZone = mkOption {
        type = types.str;
        description = "time zone you want to display info in. (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)";
        default = "UTC";
        example = "Europe/Copenhagen";
      };
    };
  };

  config = lib.mkMerge [

    (lib.mkIf (cfg.server.enable || cfg.scraper.enable || cfg.activationLogger.enable) {
      users.users.hostmap = {
        isSystemUser = true;
        group = "hostmap";
      };
      users.groups.hostmap = { };
    })

    (lib.mkIf cfg.scraper.enable {
      systemd.services.hostmap-scraper = {
        description = "scrapes servers and sends their log of activations to the hostmap server";
        wantedBy = [ "multi-user.target" ];
        after = [
          "network-online.target"
        ];
        wants = [
          "network-online.target"
        ];

        environment = {
          RUST_LOG = "info";
        };

        serviceConfig = {
          User = "hostmap";
          Group = "hostmap";
          Restart = "always";
          RestartSec = 30;
          ExecStart = ''
            ${pkgs.hostmap}/bin/hostmap scraper \
            --hosts-file ${pkgs.writeText "targetHosts.json" (builtins.toJSON cfg.scraper.targetHosts)} \
            --scrape-interval ${toString cfg.scraper.scrapeInterval} \
            --api-key-file ${cfg.scraper.apiKeyFile} \
            --url ${cfg.scraper.serverUrl} \
            --activation-logger-port "${toString cfg.scraper.activationLoggerPort}" \
            --concurrent-requests ${toString cfg.scraper.concurrentRequests}'';
        };
      };
    })
    (lib.mkIf cfg.server.enable {
      services.postgresql = {
        enable = true;
        ensureDatabases = [ "hostmap" ];
        ensureUsers = [
          {
            name = "hostmap";
            ensureDBOwnership = true;
          }
        ];
      };

      services.nginx = {
        enable = true;
        recommendedProxySettings = true;

        virtualHosts."${vhostName}-server" = lib.mkMerge [
          { default = lib.mkDefault true; }
          {
            locations."/" = {
              proxyPass = "http://127.0.0.1:${toString cfg.server.port}/";
            };
          }
        ];
      };

      networking.firewall.allowedTCPPorts = [ 80 ];

      systemd.services.hostmap-server = {
        description = "maps git rev to store path";
        wantedBy = [ "multi-user.target" ];
        after = [
          "postgresql.service"
          "network-online.target"
        ];
        wants = [
          "postgresql.service"
          "network-online.target"
        ];

        environment = {
          RUST_LOG = "info";
          TIME_ZONE_HOSTMAP = cfg.server.timeZone;
        };

        serviceConfig = {
          User = "hostmap";
          Group = "hostmap";
          Restart = "always";
          RestartSec = 2;
          ExecStart =
            let
              cols = lib.concatStringsSep "," cfg.server.columns;
              port = toString cfg.server.port;
            in
            ''
              ${pkgs.hostmap}/bin/hostmap server \
              --url 127.0.0.1 \
              --port ${port} \
              --database-url ${cfg.server.databaseUrl} \
              --repo-url ${cfg.server.repoUrl} \
              --grouping-key ${cfg.server.groupingKey} \
              --api-key-file ${cfg.server.apiKeyFile} \
              --columns "${cols}" '';
        };
      };
    })
    (lib.mkIf cfg.activationLogger.enable {
      # After default switch-to-configuration (1000)
      system.activatableSystemBuilderCommands = lib.mkOrder 1200 (
        let
          activationLog =
            with pkgs;
            writeShellScript "activation-log.sh" ''
              set -eu

              oldLogFile="/var/activationlog.csv"
              ${coreutils}/bin/mkdir -p "${activationLogFolder}"
              ${coreutils}/bin/chown root:root "${activationLogFolder}"
              ${coreutils}/bin/chmod 755 "${activationLogFolder}"

              # migrate old log file if it exists
              if ! ${coreutils}/bin/test -f "${activationLogFile}"; then
                if ${coreutils}/bin/test -f "$oldLogFile"; then
                  ${coreutils}/bin/cp "$oldLogFile" "${activationLogFile}"
                fi
              fi

              A_USER="$(${coreutils}/bin/logname 2>/dev/null || printf root)"
              A_SYSTEM="$(cd "$(${coreutils}/bin/dirname "''${BASH_SOURCE[0]}" )/.." && pwd )"
              A_NOW="$(${coreutils}/bin/date --rfc-3339=seconds)"
              A_ACTION="''${1:-unknown}"

              # Log to CSV
              ${coreutils}/bin/printf '%s;%s;%s;%s\n' "$A_NOW" "$A_USER" "$A_SYSTEM" "$A_ACTION" >> "${activationLogFile}"

              exec "$A_SYSTEM/bin/__switch-to-configuration" "$@"
            '';
        in
        ''
          SYMLINK_LOCATION="$out/bin/__switch-to-configuration"
          if [ -e $SYMLINK_LOCATION ]; then
            echo "activation-logger error:  $SYMLINK_LOCATION already exists"
            echo 'Do you have multiple things symlinking switch-to-configuration to __switch-to-configuration(two underscores)'
            exit 1
          fi
          mv $out/bin/switch-to-configuration $SYMLINK_LOCATION
          ln -s ${activationLog} $out/bin/switch-to-configuration
        ''
      );

      networking.firewall.allowedTCPPorts = [ cfg.activationLogger.port ];

      systemd.services.hostmap-activation-logger = {
        description = "Hostmap activation logger";
        wantedBy = [ "multi-user.target" ];
        after = [ "network-online.target" ];
        wants = [ "network-online.target" ];

        environment = {
          RUST_LOG = "info";
        };

        serviceConfig = {
          User = "hostmap";
          Group = "hostmap";
          ExecStart = ''
            ${pkgs.hostmap}/bin/hostmap activation-logger \
            --url-path ${activationLoggerUrl} \
            --activation-log-file ${activationLogFile} \
            --server-ip 0.0.0.0 \
            --port ${toString cfg.activationLogger.port}'';
          Restart = "always";
        };
      };
    })
  ];
}
