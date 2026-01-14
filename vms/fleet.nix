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
  loggerPort = 9001;

  pkgs = import nixpkgs {
    inherit system;
    overlays = [
      self.overlay
      crane-overlay
    ];
  };

  mkActivationLogger = {
    services.hostmap.activationLogger = {
      enable = true;
      port = loggerPort;
    };
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
          KbdInteractiveAuthentication = true;
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
      let
        host1_url = "127.0.0.2";
        host2_url = "127.0.0.3";
      in
      {
        networking.hostName = "hostmap-server";
        services.nginx.enable = true;

        # need to do this, because they actually serve on different ports, because they all run on localhost,
        # but activation logger port must be same over all hosts.
        services.nginx.virtualHosts."hostmap" = {
          listen = [
            {
              addr = "0.0.0.0";
              port = 80;
            }
          ];
          locations."/" = {
            proxyPass = "http://127.0.0.1:3000";
          };
        };

        services.nginx.virtualHosts."host1-proxy" = {
          listen = [
            {
              addr = host1_url;
              port = loggerPort;
            }
          ];
          locations."/" = {
            proxyPass = "http://10.0.2.2:${toString loggerPort}";
          };
        };

        services.nginx.virtualHosts."host2-proxy" = {
          listen = [
            {
              addr = host2_url;
              port = loggerPort;
            }
          ];
          locations."/" = {
            proxyPass = "http://10.0.2.2:${toString (loggerPort + 1)}";
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

        services.hostmap.server = {
          enable = true;
          repoUrl = "https://example.invalid/commit";
          groupingKey = "host_group_name";
          databaseUrl = "postgresql:///hostmap?user=hostmap&host=/run/postgresql";
          apiKeyFile = toString (pkgs.writeText "hostmap-api-key.txt" apiKey);
          columns = [
            "host_group_name"
          ];
        };

        services.hostmap.scraper = {
          enable = true;
          serverUrl = "http://127.0.0.1:3000";
          activationLoggerPort = loggerPort;
          apiKeyFile = toString (pkgs.writeText "hostmap-api-key.txt" apiKey);
          targetHosts = [
            {
              hostname = "hostmap-host1";
              host_url = host1_url;
              metadata = {
                environment = "test";
                host_group_name = "hg-1";
              };
            }
            {
              hostname = "hostmap-host2";
              host_url = host2_url;
              metadata = {
                environment = "test";
                host_group_name = "hg-1";
              };
            }
          ];
        };
      }
    )
  ];

  hostmap-host1 = mkNixos [
    self.nixosModules.hostmap
    mkActivationLogger
    (
      { ... }:
      {
        networking.hostName = "hostmap-host1";

        virtualisation.forwardPorts = [
          {
            from = "host";
            host.port = loggerPort;
            guest.port = loggerPort;
          }
          {
            from = "host";
            host.port = 2222;
            guest.port = 22;
          }
        ];

        virtualisation.vmVariant.virtualisation.cores = 1;
        virtualisation.vmVariant.virtualisation.memorySize = 1024;
      }
    )
  ];

  hostmap-host2 = mkNixos [
    self.nixosModules.hostmap
    mkActivationLogger
    (
      { ... }:
      {
        networking.hostName = "hostmap-host2";

        virtualisation.forwardPorts = [
          {
            from = "host";
            host.port = loggerPort + 1;
            guest.port = loggerPort;
          }
          {
            from = "host";
            host.port = 2223;
            guest.port = 22;
          }
        ];

        virtualisation.vmVariant.virtualisation.cores = 1;
        virtualisation.vmVariant.virtualisation.memorySize = 1024;

      }
    )
  ];
}
