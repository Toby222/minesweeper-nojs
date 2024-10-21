{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
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
              };
            in
            {
              inherit pkgs;
            }
          )
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
            # nativeBuildInputs = [ pkgs.rustPlatform ];
            # buildInputs = [ pkgs.rustPlatform ];
          };
        }
      );
      packages = forEachSupportedSystem (
        { pkgs }:
        {
          default = (
            let
              manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
              rustPlatform = pkgs.rustPlatform;
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
