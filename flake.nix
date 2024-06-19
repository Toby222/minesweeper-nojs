{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
      fenix,
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
          f (
            let
              pkgs = import nixpkgs {
                inherit system;
                overlays = [ fenix.overlays.default ];
              };
              fenix' = (import fenix { inherit system pkgs; });
              toolchain = fenix'.fromToolchainFile {
                file = ./rust-toolchain.toml;
                sha256 = "sha256-Ngiz76YP4HTY75GGdH2P+APE/DEIx2R/Dn+BwwOyzZU=";
              };
            in
            {
              inherit pkgs toolchain;
            }
          )
        );
    in
    {
      formatter = forEachSupportedSystem (
        { pkgs, ... }: (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.wrapper
      );
      devShells = forEachSupportedSystem (
        { pkgs, toolchain }:
        {
          default = pkgs.mkShell {
            nativeBuildInputs = [ toolchain ];
            buildInputs = [ toolchain ];
          };
        }
      );
      packages = forEachSupportedSystem (
        { pkgs, toolchain }:
        {
          default = (
            let
              manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
              rustPlatform = pkgs.makeRustPlatform {
                cargo = toolchain;
                rustc = toolchain;
              };
            in
            rustPlatform.buildRustPackage {
              pname = manifest.name;
              version = manifest.version;
              cargoLock.lockFile = ./Cargo.lock;
              src = pkgs.lib.cleanSource ./.;
            }
          );
        }
      );
    };
}
