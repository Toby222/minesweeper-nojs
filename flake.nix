{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f ({
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ (import rust-overlay) ];
            };
          })
        );
    in
    {
      formatter = forEachSupportedSystem (
        { pkgs, ... }: (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.wrapper
      );
      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              pkg-config
              (rust-bin.nightly.latest.default.override
              {
                extensions = [ "rust-src" ];
              })
            ];
          };
        }
      );
      packages = forEachSupportedSystem (
        { pkgs }:
        {
          default = (
            let
              manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
              rustPlatform = pkgs.makeRustPlatform {
                cargo = pkgs.rust-bin.nightly.latest.minimal.override {
                  extensions = [ "rust-src" ];
                };
                rustc = pkgs.rust-bin.nightly.latest.minimal.override {
                  extensions = [ "rust-src" ];
                };
              };
            in
            rustPlatform.buildRustPackage {
              pname = manifest.name;
              version = manifest.version;
              cargoLock.lockFile = ./Cargo.lock;
              src = pkgs.lib.cleanSource ./.;
              env = {
                RUSTFLAGS = "-C target-cpu=native";
              };
            }
          );
        }
      );
    };
}
