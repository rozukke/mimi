{
  description = "A dev environment for Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  nixConfig = {
    bash-prompt-prefix = "(rust-shell) ";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Specify Rust version here
        _rustToolchain = pkgs.rust-bin.stable.latest.default;
        _rustPlatform = pkgs.makeRustPlatform {
          rustc = _rustToolchain;
          cargo = _rustToolchain;
        };

        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;

      in {
        packages = {
          mimi = _rustPlatform.buildRustPackage {
            pname = manifest.name;
            inherit (manifest) version;

            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            buildInputs = with pkgs; [
              python38
            ];

            preCheck = ''
              export RUST_BACKTRACE=1 
            '';
          };

          # Default target for nix commands
          default = self.packages.${system}.mimi;

        };

        devShells.default = pkgs.mkShell {

          buildInputs = with pkgs; [
            _rustToolchain
            python39
          ];

          env = {
            RUST_SRC_PATH = "${_rustToolchain}/lib/rustlib/src/rust/library";
          };

        };
      }
    );
}