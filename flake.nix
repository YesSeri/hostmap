{
  description = "Rust web project using Axum";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
      };

      buildInputs = [
        rustToolchain
        pkgs.pkg-config
        pkgs.openssl
        pkgs.postgresql_16 
        pkgs.sqlx-cli
		pkgs.rust-analyzer

      ];
    in {
	  devShells.default = pkgs.mkShell {
        packages = buildInputs;

        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

        shellHook = ''
PGFOLDER=$PWD/pg
mkdir -p $PGFOLDER
export PGDATA=$PGFOLDER/.pgdata
  export PGPORT=5433
export PGSOCKETS=$PGFOLDER/.pgsock

mkdir -p "$PGSOCKETS"

if [ ! -d "$PGDATA" ]; then
echo "Initializing PostgreSQL data dir"
initdb --username=devuser --no-locale > /dev/null
fi

echo "Starting PostgreSQL on port $PGPORT, sockets in $PGSOCKETS"
pg_ctl -D "$PGDATA" -o "-p $PGPORT -k $PGSOCKETS" -l postgres.log start

echo "To stop: pg_ctl -D \"$PGDATA\" stop"
'';

      };

      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "my-axum-app";
        version = "0.1.0";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = buildInputs;

        meta.mainProgram = "my-axum-app";
      };
    }
  );
}
