{
  inputs = {
    flakeCompat.url = github:edolstra/flake-compat;
    flakeCompat.flake = false;
    flakeUtils.url = "github:numtide/flake-utils";
    nixpkgs.follows = "fenix/nixpkgs";
    # add https://app.cachix.org/cache/nix-community to your `nix.conf`
    fenix.url = "github:nix-community/fenix";
  };
  outputs =
    inputs:
    inputs.flakeUtils.lib.eachDefaultSystem
      (
        system:
        let
          toolchain = "stable";
          nixpkgs = import inputs.nixpkgs { inherit system; };
          cargoToml = builtins.fromTOML ( builtins.readFile ./Cargo.toml );
        in
        {
          checks = {
            defaultPackage = inputs.self.defaultPackage.${ system };
            inherit ( inputs.self.packages.${ system } ) nixpkgsFormatted;
          };
          defaultApp = { type = "app"; program = "${ inputs.self.defaultPackage.${ system } }/bin/alejandra"; };
          defaultPackage =
            (nixpkgs.makeRustPlatform {
              inherit (inputs.fenix.packages.${ system }.${ toolchain }) cargo rustc;
            }).buildRustPackage
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
                NIX_BUILD_CORES = 0;
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
                  inputs.fenix.packages.${ system }.rust-analyzer
                  (inputs.fenix.packages.${ system }.${ toolchain }.withComponents [
                    "cargo"
                    "clippy"
                    "rust-src"
                    "rustc"
                    "rustfmt"
                  ])
                ];
              };
          packages = {
            nixpkgsFormatted =
              nixpkgs.stdenv.mkDerivation
                {
                  name = "nixpkgs-formatted";
                  builder =
                    builtins.toFile
                      "builder.sh"
                      ''
                      source $stdenv/setup

                      cp -rT $nixpkgs $out
                      chmod -R +w $out

                      alejandra $out

                      git diff --no-index $nixpkgs $out > $diff || true
                      '';
                  buildInputs = [ inputs.self.defaultPackage.${ system } nixpkgs.git ];
                  nixpkgs = inputs.nixpkgs.sourceInfo.outPath;
                  NIX_BUILD_CORES = 0;
                  outputs = [ "diff" "out" ];
                };
          };
        }
      );
}
