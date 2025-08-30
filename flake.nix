{
  description = "hostoverview";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    ci.url = "git+https://gitlab.dbc.dk/platform/bump-o-matic.git";
    ci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, ci }:
  let
    pname = "hostoverview";
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ ci.overlay ];
    };

    localDevScript = pkgs.writeShellScriptBin "hostoverview" ''
      systems=''${SYSTEMS:-testdata}
      deployments=''${DEPLOYMENTS:-../deployments}
 
      export HO_SYSTEMS_PATH=$systems
      export HO_DEPLOYMENTS_PATH=$deployments
      php -S localhost:8888
    '';

    bump = pkgs.writeShellScriptBin "bump" ''
      composer update
    '';
  in {

    devShell.${system} = with pkgs; mkShell {
      buildInputs = [
        bump-o-matic
        bump
        localDevScript        
        php
        phpPackages.composer
        nix
        nix-diff
      ];
    };
  };
}
