{ self, nixpkgs, crane, system ? "x86_64-linux" }:

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

  mkCommon = { pkgs, ... }: {
    services.openssh.enable = true;

    users.users.root.initialPassword = "root";
    services.getty.autologinUser = "root";

    environment.systemPackages = with pkgs; [ curl jq vim git ];

    system.stateVersion = "25.11";
  };

  mkNixos = modules:
    lib.nixosSystem {
      inherit system pkgs;
      specialArgs = { inherit self; };

      modules = [
        mkCommon
        (import (nixpkgs + "/nixos/modules/virtualisation/qemu-vm.nix"))
      ] ++ modules;
    };

  apiKey = builtins.readFile ./test-api-key.txt;

in
{
  hostmap-server = mkNixos [
    self.nixosModules.hostmap
    ({ pkgs, ... }: {
      networking.hostName = "hostmap-server";

      virtualisation.forwardPorts = [
        { from = "host"; host.port = 8080; guest.port = 80; }
      ];

      virtualisation.vmVariant.virtualisation.cores = 2;
      virtualisation.vmVariant.virtualisation.memorySize = 2048;

      services.hostmap.server.enable = true;
      services.hostmap.server.repoUrl = "https://example.invalid/commit";
      services.hostmap.server.groupingKey = "environment";
      services.hostmap.server.columns = [ "environment" "host_group_name" ];
      services.hostmap.server.timeZone = "Europe/Copenhagen";
	  services.hostmap.server.databaseUrl = "postgresql:///hostmap?user=hostmap&host=/run/postgresql";


      services.hostmap.server.apiKeyFile = toString (pkgs.writeText "hostmap-api-key.txt" apiKey);

    })
  ];

  hostmap-host1 = mkNixos [
    self.nixosModules.hostmap
    ({ ... }: {
      networking.hostName = "hostmap-host1";

      virtualisation.forwardPorts = [
        { from = "host"; host.port = 9001; guest.port = 9001; }
      ];

      virtualisation.vmVariant.virtualisation.cores = 1;
      virtualisation.vmVariant.virtualisation.memorySize = 1024;

      services.hostmap.activationLogger.enable = true;
      services.hostmap.activationLogger.port = 9001;
    })
  ];

  hostmap-host2 = mkNixos [
    self.nixosModules.hostmap
    ({ ... }: {
      networking.hostName = "hostmap-host2";

      virtualisation.forwardPorts = [
        { from = "host"; host.port = 9002; guest.port = 9001; }
      ];

      virtualisation.vmVariant.virtualisation.cores = 1;
      virtualisation.vmVariant.virtualisation.memorySize = 1024;

      services.hostmap.activationLogger.enable = true;
      services.hostmap.activationLogger.port = 9001;
    })
  ];
}

