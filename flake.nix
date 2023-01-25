{
  description = "diva_db";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }: let
    emptyOverlay = final: prev: {};
    diva_db-drv = pkgs:
      pkgs.rustPlatform.buildRustPackage {
        pname = "diva_db";
        version = "v0.1.0";

        src = ./.;

        buildFeatures = [ "serde" "pyo3" ];
        doCheck = false;

        cargoLock = {
          # Why I yes, I would like not writing the hash of my Cargo.lock very much.
          lockFile = ./Cargo.lock;
          outputHashes = {
            "nom_ext-0.1.0" = "sha256-SndCtQsBQYZoDwkHqQbWrEoUH0Z2TRgFGEkq3ppr+OQ=";
          };
        };
      };
    diva_db-python-drv = pkgs: isNew: pythonPackages:
      pythonPackages.buildPythonPackage rec {
        pname = "diva_db";
        version = "v0.1.0";

        src = ./.;

        cargoDeps = pkgs.rustPlatform.importCargoLock {
          # Why I yes, I would like not writing the hash of my Cargo.lock very much.
          lockFile = ./Cargo.lock;
          outputHashes = {
            "nom_ext-0.1.0" = "sha256-SndCtQsBQYZoDwkHqQbWrEoUH0Z2TRgFGEkq3ppr+OQ=";
          };
        };

        format = "pyproject";

        # HACK: maturinBuildHook is dumb and doesn't read pyproject.toml for some reason
        maturinBuildFlags = if isNew then ["--all-features"] else [''--cargo-extra-args="--all-features"''];
        nativeBuildInputs = with pkgs.rustPlatform; [cargoSetupHook maturinBuildHook];

        # needed for maturin
        propagatedBuildInputs = with pythonPackages; [cffi];
      };
    pythonOverride = prev: isNew: (prevArgs: {
      packageOverrides = let
        ourOverlay = new: old: {
          diva_db = diva_db-python-drv prev isNew old;
        };
      in
        prev.lib.composeExtensions
        prevArgs.packageOverrides or emptyOverlay
        ourOverlay;
    });
  in
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
      in rec {
        packages = rec {
          diva_db = diva_db-drv pkgs;
          diva_db-python = diva_db-python-drv pkgs true pkgs.python3Packages;
          default = diva_db;
        };
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = ["rust-src" "cargo" "rustc"];
              targets = [ "x86_64-pc-windows-gnu" ];
            })
            gcc
          ];

          RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default.override {
            extensions = ["rust-src"];
          }}/lib/rustlib/src/rust/library";
          buildInputs = with pkgs; [
            maturin
            rust-analyzer
            pkgsCross.mingwW64.stdenv.cc
            # pkgs.pkgsCross.mingwW64.windows.pthreads
            (pkgs.python3.withPackages (p:
              with p; [
                cffi
              ]))
          ];
        };
        devShells.python = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            (pkgs.python3.withPackages (p:
              with p; [
                packages.diva_db-python
              ]))
          ];
        };
      }
    )
    // {
      overlays.default = final: prev: rec {
        diva_db = diva_db-drv prev;
        python3 = prev.python3.override (pythonOverride prev true);
        python310 = prev.python310.override (pythonOverride prev true);
        python39 = prev.python39.override (pythonOverride prev false);
      };
    };
}
