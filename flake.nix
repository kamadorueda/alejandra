{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    flakeCompat.url = github:edolstra/flake-compat;
    flakeCompat.flake = false;
    flakeUtils.url = "github:numtide/flake-utils";
    treefmt.url = "github:numtide/treefmt";
    treefmt.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };
  outputs =
    inputs:
    inputs.flakeUtils.lib.eachDefaultSystem
      (
        system:
        let
          nixpkgs = import inputs.nixpkgs { inherit system; };
          cargoToml = builtins.fromTOML ( builtins.readFile ./Cargo.toml );
          treefmt = inputs.treefmt.defaultPackage.${ system };
          fenix = inputs.fenix.packages.${ system };
          fenixPlatform = nixpkgs.makeRustPlatform { inherit ( fenix.latest ) cargo rustc; };
        in
        {
          checks = { defaultPackage = inputs.self.defaultPackage.${ system }; };
          defaultApp = {
            type = "app";
            program = "${ inputs.self.defaultPackage.${ system } }/bin/alejandra";
          };
          defaultPackage =
            fenixPlatform.buildRustPackage
              {
                pname = cargoToml.package.name;
                version =
                  let
                    commit = inputs.self.shortRev or "dirty";
                    date = inputs.self.lastModifiedDate or inputs.self.lastModified or "19700101";
                  in
                  "${ builtins.substring 0 8 date }_${ commit }";
                src = inputs.self.sourceInfo;
                cargoLock.lockFile = ./Cargo.lock;
                meta = {
                  description = cargoToml.package.description;
                  homepage = "https://github.com/kamadorueda/alejandra";
                  license = nixpkgs.lib.licenses.unlicense;
                  maintainers = [ nixpkgs.lib.maintainers.kamadorueda ];
                };
              };
          devShell =
            nixpkgs.mkShell
              {
                packages = [
                  fenix.rust-analyzer
                  fenix.latest.cargo
                  fenix.latest.clippy
                  fenix.latest.rust-src
                  fenix.latest.rustc
                  fenix.latest.rustfmt
                  treefmt
                  nixpkgs.shfmt
                  nixpkgs.nodePackages.prettier
                ];
                shellHook = "cargo build";
              };
        }
      );
}
