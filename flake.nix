{
  description = "hostmap shows git revision to nix store path link and more";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      crane,
      nixpkgs,
      treefmt-nix,
      pre-commit-hooks,
    }:
    let
      pname = "hostmap";
      system = "x86_64-linux";
      testOutput = "hello";
      crane-overlay = final: prev: {
        # crane's lib is not exposed as an overlay in its flake (should be added
        # upstream ideally) so this interface might be brittle, but avoids
        # accidentally passing a detached nixpkgs from its flake (or its follows)
        # on to consumers.
        craneLib = crane.mkLib prev;
      };
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          self.overlay
          crane-overlay
        ];
      };
      lib = nixpkgs.lib;

      treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;

      outputPackages = {
        "${pname}" = [ ];
      };
    in
    {
      packages.${system} = lib.mapAttrs (n: _: pkgs.${n}) outputPackages;
      defaultPackage.${system} = self.packages.${system}.hostmap;
      overlay =
        final: prev:
        let
          cratePackage =
            name: features:
            (final.craneLib.buildPackage {
              src =
                with final;
                lib.cleanSourceWith {
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
              nativeBuildInputs = with final; [
                pkg-config
                makeWrapper
              ];
              buildInputs = with final; [ openssl ];
              env.SQLX_OFFLINE = "true";
              cargoExtraArgs = final.lib.concatMapStringsSep " " (f: "--features=${f}") features;
              postInstall = ''
                mkdir -p $out/share/hostmap/templates
                cp -R --no-preserve=mode,ownership ./templates/* $out/share/hostmap/templates
                chmod -R u+w $out/share/hostmap/templates
                wrapProgram $out/bin/hostmap \
                  --set HOSTMAP_TEMPLATES_DIR $out/share/hostmap/templates
              '';

            });
        in
        lib.mapAttrs cratePackage outputPackages;

      devShells.${system} = {
        default =
          let
            preCommitHook = self.checks.${system}.pre-commit-check.shellHook;
          in
          with pkgs;
          mkShell {
            packages = [
              pkg-config
              openssl
            ];
            buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;
            inputsFrom = [ self.defaultPackage.${system} ];
            nativeBuildInputs = [
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
            OPENSSL_DIR = openssl.dev;
            OPENSSL_LIB_DIR = "${openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
            PKG_CONFIG_PATH = nixpkgs.lib.makeSearchPath "lib/pkgconfig" [ openssl.dev ];

            shellHook = ''
                                          ${preCommitHook} 
                            	      
                                          export HOSTMAP_TEMPLATES_DIR='./templates'
                            	          export RUST_LOG='info,hostmap=debug'

                                          export PG=$PWD/.dev_postgres
                                          export PGDATA=$PG/data
                                          export PGPORT=5432
                                          export PGHOST=localhost
                                          export PGUSER=$USER
                                          export PGPASSWORD=postgres
                                          #export PGDATABASE=hostmap-dev
                                          export PGDATABASE=hostmap_restore
                                          export DATABASE_URL=postgres://$PGUSER:$PGPASSWORD@$PGHOST:$PGPORT/$PGDATABASE
                                          alias pg_start="pg_ctl -D $PGDATA -l $PG/postgres.log start"
                                          alias pg_stop="pg_ctl -D $PGDATA stop"
                                          pg_initial_setup() {
                                            pg_stop;
                                            rm -rf $PG;
                                            initdb -D $PGDATA &&
                                            echo "unix_socket_directories = '$PGDATA'" >> $PGDATA/postgresql.conf && pg_start && createdb
                                          }
              			    pg_ctl -D .dev_postgres/data/ status &> /dev/null && echo "Server already running" || pg_ctl -D $PGDATA -l $PG/postgres.log start
            '';
          };
      };

      formatter.${system} = treefmtEval.config.build.wrapper;

      checks."x86_64-linux" = {
        formatting = treefmtEval.config.build.check self;
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            rustfmt.enable = true;
            nixfmt-rfc-style.enable = true;
            sqlx-prepare-migrations = {
              enable = true;
              entry = ''./sqlx-prepare-migrations.sh'';
              pass_filenames = false;
              always_run = true;
            };
          };
        };
      };
      nixosModules.hostmap = import ./module.nix;
    };
}
