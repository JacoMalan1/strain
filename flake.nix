{
  description = "NixOS dev shell flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rustVersion = pkgs.rust-bin.stable.latest.default;

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustVersion;
        rustc = rustVersion;
      };

      rustBuild = rustPlatform.buildRustPackage {
        pname = "strain";
        version = "0.1.4";
        src = ./.;

        buildInputs = with pkgs; [
          gmp
          mpfr
          libmpc
        ];
      
        env = {
          CFLAGS = ''
            -I${pkgs.gmp.dev}/include 
            -L${pkgs.gmp}/lib 
            -I${pkgs.mpfr.dev}/include 
            -L${pkgs.mpfr}/lib
            -I${pkgs.libmpc}/include
            -L${pkgs.libmpc}/lib
          '';
        };

        cargoLock.lockFile = ./Cargo.lock;
      };
    in {
      defaultPackage = rustBuild;
      devShell = pkgs.mkShell {
        packages = with pkgs; [
          gmp
          mpfr
          libmpc
        ];

        env = {
          CFLAGS = ''
            -I${pkgs.gmp.dev}/include 
            -L${pkgs.gmp}/lib 
            -I${pkgs.mpfr.dev}/include 
            -L${pkgs.mpfr}/lib
            -I${pkgs.libmpc}/include
            -L${pkgs.libmpc}/lib
          '';
        };
      };
    });
}
