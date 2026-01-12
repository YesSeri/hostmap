{
  self,
  nixpkgs,
  crane,
  system ? "x86_64-linux",
}:

let
  lib = nixpkgs.lib;

  crane-overlay = final: prev: {
    craneLib = crane.mkLib prev;
  };

  pkgs = import nixpkgs {
    inherit system;
    overlays = [
      self.overlay
      crane-overlay
    ];
  };

  mkCommon =
    { pkgs, ... }:
    {
      systemd.oomd.enable = false;
      services.openssh = {
        enable = true;
        openFirewall = true;
        settings = {
          PermitRootLogin = "yes";
          PasswordAuthentication = true;
          KbdInteractiveAuthentication = true; # helps on some setups
          UsePAM = true;
        };
      };

      users.users.root.initialPassword = "root";
      services.getty.autologinUser = lib.mkForce null;

      environment.systemPackages = with pkgs; [
        curl
        jq
        vim
        git
      ];

      system.stateVersion = "25.11";
    };

  mkNixos =
    modules:
    lib.nixosSystem {
      inherit system pkgs;
      specialArgs = { inherit self; };

      modules = [
        mkCommon
        (import (nixpkgs + "/nixos/modules/virtualisation/qemu-vm.nix"))
      ]
      ++ modules;
    };

  apiKey = builtins.readFile ./test-api-key.txt;

in
{
  hostmap-server = mkNixos [
    self.nixosModules.hostmap
    (
      { pkgs, ... }:
      {
        networking.hostName = "hostmap-server";
        services.nginx.enable = true;

        services.nginx.virtualHosts."host1-proxy" = {
          listen = [
            {
              addr = "127.0.0.2";
              port = 9001;
            }
          ];
          locations."/" = {
            proxyPass = "http://10.0.2.2:9001";
          };
        };

        services.nginx.virtualHosts."host2-proxy" = {
          listen = [
            {
              addr = "127.0.0.3";
              port = 9001;
            }
          ];
          locations."/" = {
            proxyPass = "http://10.0.2.2:9002";
          };
        };

        virtualisation.forwardPorts = [
          {
            from = "host";
            host.port = 8080;
            guest.port = 80;
          }
          {
            from = "host";
            host.port = 2221;
            guest.port = 22;
          }
        ];

        virtualisation.vmVariant.virtualisation.cores = 2;
        virtualisation.vmVariant.virtualisation.memorySize = 2048;

        services.hostmap.server.enable = true;
        services.hostmap.server.repoUrl = "https://example.invalid/commit";
        services.hostmap.server.groupingKey = "environment";
        #         services.hostmap.server.columns = [
        #           "environment"
        #           "host_group_name"
        #         ];
        # services.hostmap.server.timeZone = "Europe/Copenhagen";
        services.hostmap.server.databaseUrl = "postgresql:///hostmap?user=hostmap&host=/run/postgresql";

        services.hostmap.server.apiKeyFile = toString (pkgs.writeText "hostmap-api-key.txt" apiKey);
        services.hostmap.scraper.enable = true;
        services.hostmap.scraper.serverUrl = "http://127.0.0.1:3000";
        services.hostmap.scraper.activationLoggerPort = 9001;
        services.hostmap.scraper.apiKeyFile = toString (pkgs.writeText "hostmap-api-key.txt" apiKey);

        services.hostmap.scraper.targetHosts = [
          {
            hostname = "hostmap-host1";
            host_url = "127.0.0.2";
            metadata = {
              environment = "test";
            };
          }
          {
            hostname = "hostmap-host2";
            host_url = "127.0.0.3";
            metadata = {
              environment = "test";
            };
          }
        ];

      }
    )
  ];

  hostmap-host1 = mkNixos [
    self.nixosModules.hostmap
    (
      { ... }:
      {
        networking.hostName = "hostmap-host1";

        virtualisation.forwardPorts = [
          {
            from = "host";
            host.port = 9001;
            guest.port = 9001;
          }
          {
            from = "host";
            host.port = 2222;
            guest.port = 22;
          }

        ];

        virtualisation.vmVariant.virtualisation.cores = 1;
        virtualisation.vmVariant.virtualisation.memorySize = 1024;

        services.hostmap.activationLogger.enable = true;
        services.hostmap.activationLogger.port = 9001;
      }
    )
  ];

  hostmap-host2 = mkNixos [
    self.nixosModules.hostmap
    (
      { ... }:
      {
        networking.hostName = "hostmap-host2";

        virtualisation.forwardPorts = [
          {
            from = "host";
            host.port = 9002;
            guest.port = 9001;
          }
          {
            from = "host";
            host.port = 2223;
            guest.port = 22;
          }
        ];

        virtualisation.vmVariant.virtualisation.cores = 1;
        virtualisation.vmVariant.virtualisation.memorySize = 1024;

        services.hostmap.activationLogger.enable = true;
        services.hostmap.activationLogger.port = 9001;
      }
    )
  ];
}
