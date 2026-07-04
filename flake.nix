{
  description = "hostmap shows git revision to nix store path link and more";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

    crane = {
      url = "github:ipetkov/crane";
    };

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    {
      self,
      crane,
      nixpkgs,
      pre-commit-hooks,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      craneLib = crane.mkLib pkgs;
      lib = nixpkgs.lib;
      hostmapPkg = craneLib.buildPackage {
        pname = "hostmap";
        version = "0.1.0";

        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter =
            let
              drvnix = path: _type: builtins.match ".*/drv.nix" path != null;
              migrations = path: _type: builtins.match ".*/migrations/.*\\.up\\.sql" path != null;
              sqlxMetadata = path: _type: builtins.match ".*/\\.sqlx/.*\\.json" path != null;
              templates = path: _type: builtins.match ".*/templates/.*\\.tera" path != null;
            in
            path: type:
            craneLib.filterCargoSources path type
            || drvnix path type
            || migrations path type
            || templates path type
            || sqlxMetadata path type;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
          makeWrapper
        ];
        buildInputs = with pkgs; [ openssl ];

        env.SQLX_OFFLINE = "true";

        postInstall = ''
          mkdir -p $out/share/hostmap/templates
          cp -R --no-preserve=mode,ownership ./templates/* $out/share/hostmap/templates
          chmod -R u+w $out/share/hostmap/templates
          wrapProgram $out/bin/hostmap \
            --set HOSTMAP_TEMPLATES_DIR $out/share/hostmap/templates
        '';
      };

    in
    {
      packages.${system} = {
        hostmap = hostmapPkg;
        default = hostmapPkg;
      };

      overlay = final: prev: {
        hostmap = self.packages.${final.system}.hostmap;
      };

      devShells.${system}.default =
        let
          preCommitHook = self.checks.${system}.pre-commit-check.shellHook;
        in
        pkgs.mkShell {
          packages = with pkgs; [
            pkg-config
            openssl
          ];
          buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;
          inputsFrom = [ self.packages.${system}.hostmap ];
          nativeBuildInputs = with pkgs; [
            sqlx-cli
            gdb
            cargo
            rustc
            nix
            clippy
            rustfmt
            rust-analyzer
            postgresql_16
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          OPENSSL_NO_VENDOR = "1";
          OPENSSL_DIR = pkgs.openssl.dev;
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          PKG_CONFIG_PATH = nixpkgs.lib.makeSearchPath "lib/pkgconfig" [ pkgs.openssl.dev ];

          shellHook = ''
            export PG=$PWD/.dev_postgres
            export PGDATA=$PG/data
            export PGPORT=5432
            export PGHOST=localhost
            export PGUSER=$USER
            export PGDATABASE=hostmap
            export DATABASE_URL=postgres://$PGUSER@$PGHOST:$PGPORT/$PGDATABASE

            pg_start() {
              mkdir -p "$PG"

              if [ ! -d "$PGDATA" ]; then
                initdb -D "$PGDATA" --auth=trust
                echo "unix_socket_directories = '$PGDATA'" >> "$PGDATA/postgresql.conf"
                echo "port = $PGPORT" >> "$PGDATA/postgresql.conf"
              fi

              if ! pg_ctl -D "$PGDATA" status >/dev/null 2>&1; then
                pg_ctl -D "$PGDATA" -l "$PG/postgres.log" start
              fi

              createdb "$PGDATABASE" 2>/dev/null || true
            }

            pg_stop() {
              pg_ctl -D "$PGDATA" stop
            }

            pg_reset() {
              pg_stop 2>/dev/null || true
              rm -rf "$PG"
              pg_start
            }

            pg_start

          '';
        };

      checks.${system} = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            rustfmt.enable = true;
            nixfmt-rfc-style.enable = true;
            sqlx-prepare-migrations = {
              enable = true;
              entry = "./sqlx-prepare-migrations.sh";
              pass_filenames = false;
              always_run = true;
            };
          };
        };
      };
      nixosModules.hostmap = import ./module.nix;
    };
}
