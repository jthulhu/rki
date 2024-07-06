{
  description = "redac";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ];
        };
        build-dependencies = with pkgs; [
          gtk4.dev
          dav1d.dev
          libavif
          pkg-config
          meson
          ninja
          nasm
          cmake
        ];
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" ];
        };
        naerskLib = naersk.lib.${system}.override {
          cargo = rust;
          rustc = rust;
        };
      in {
        defaultPackage = naerskLib.buildPackage {
          pname = "redac";
          root = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ gtk4 ];
        };
        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage.${system};
        };
        devShell = with pkgs; mkShell {
          packages = [
            rust
            cargo
            cargo-edit
            rustfmt
            rustPackages.clippy
            rust-analyzer
          ] ++ build-dependencies;
        };
      });
}
