{
  description = "Incinerator Sentinel: centinela zero-trust para servicios criticos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems
          (system: f (import nixpkgs { inherit system; }));
    in {
      packages = forAllSystems (pkgs: rec {
        incinerator-sentinel = pkgs.rustPlatform.buildRustPackage {
          pname = "incinerator-sentinel";
          version = "0.1.0";
          src = ../.;
          cargoLock.lockFile = ../Cargo.lock;
          buildFeatures = [ "hardened" "volatile-audit" ];
          doCheck = true;
        };

        default = incinerator-sentinel;
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [ cargo rustc rustfmt clippy ];
        };
      });

      checks = forAllSystems (pkgs: {
        inherit (self.packages.${pkgs.system}) incinerator-sentinel;
      });
    };
}
