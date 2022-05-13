{
  description = "The Uncompromising Nix Code Formatter";

  inputs = {
    flakeCompat.url = github:edolstra/flake-compat;
    flakeCompat.flake = false;

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
  };

  outputs = inputs: let
    commit = inputs.self.shortRev or "dirty";
    date = inputs.self.lastModifiedDate or inputs.self.lastModified or "19700101";
    version = "1.3.0+${builtins.substring 0 8 date}.${commit}";

    nixpkgsForHost = host:
      import inputs.nixpkgs {
        overlays = [overlay];
        system = host;
      };

    nixpkgs."aarch64-darwin" = nixpkgsForHost "aarch64-darwin";
    nixpkgs."aarch64-linux" = nixpkgsForHost "aarch64-linux";
    nixpkgs."i686-linux" = nixpkgsForHost "i686-linux";
    nixpkgs."x86_64-darwin" = nixpkgsForHost "x86_64-darwin";
    nixpkgs."x86_64-linux" = nixpkgsForHost "x86_64-linux";

    overlay = final: prev: {
      alejandra = final.rustPlatform.buildRustPackage {
        pname = "alejandra";
        inherit version;
        src = final.stdenv.mkDerivation {
          name = "src";
          builder = builtins.toFile "builder.sh" ''
            source $stdenv/setup

            mkdir $out
            cp -rT --no-preserve=mode,ownership $src $out/src/
            cp $cargoLock $out/Cargo.lock
            cp $cargoToml $out/Cargo.toml
          '';
          cargoLock = ./Cargo.lock;
          cargoToml = ./Cargo.toml;
          src = ./src;
        };
        cargoLock.lockFile = ./Cargo.lock;

        passthru.tests = {
          version = final.testVersion {package = prev.alejandra;};
        };

        meta = {
          description = "The Uncompromising Nix Code Formatter.";
          homepage = "https://github.com/kamadorueda/alejandra";
          license = final.lib.licenses.unlicense;
          maintainers = [final.lib.maintainers.kamadorueda];
          platforms = final.lib.systems.doubles.all;
        };
      };
    };

    buildBinariesForHost = host: pkgs: let
      binaries = builtins.listToAttrs (
        builtins.map (pkg: {
          name = "alejandra-${pkg.stdenv.targetPlatform.config}";
          value = pkg;
        })
        pkgs
      );
    in
      binaries
      // {
        "alejandra-binaries" = nixpkgs.${host}.linkFarm "alejandra-binaries" (
          nixpkgs.${host}.lib.mapAttrsToList
          (name: binary: {
            inherit name;
            path = "${binary}/bin/alejandra";
          })
          binaries
        );
        "default" = builtins.elemAt pkgs 0;
      };
  in rec {
    checks."aarch64-darwin" = packages."aarch64-darwin";
    checks."aarch64-linux" = packages."aarch64-linux";
    checks."i686-linux" = packages."i686-linux";
    checks."x86_64-darwin" = packages."x86_64-darwin";
    checks."x86_64-linux" = packages."x86_64-linux";

    defaultPackage."aarch64-darwin" = packages."aarch64-darwin"."alejandra-aarch64-apple-darwin";
    defaultPackage."aarch64-linux" = packages."aarch64-linux"."alejandra-aarch64-unknown-linux-gnu";
    defaultPackage."i686-linux" = packages."i686-linux"."alejandra-i686-unknown-linux-gnu";
    defaultPackage."x86_64-darwin" = packages."x86_64-darwin"."alejandra-x86_64-apple-darwin";
    defaultPackage."x86_64-linux" = packages."x86_64-linux"."alejandra-x86_64-unknown-linux-gnu";

    devShell."x86_64-linux" = with nixpkgs."x86_64-linux";
      mkShell {
        name = "alejandra";
        packages = [
          cargo
          cargo-bloat
          cargo-license
          cargo-tarpaulin
          clippy
          jq
          linuxPackages_latest.perf
          nodejs
          nodePackages.prettier
          nodePackages.prettier-plugin-toml
          rustc
          shfmt
          treefmt
          yarn
          yarn2nix
        ];
      };

    inherit overlay;
    overlays.default = overlay;

    packages."aarch64-darwin" = with nixpkgs."aarch64-darwin";
      buildBinariesForHost "aarch64-darwin" [
        alejandra
      ];
    packages."aarch64-linux" = with nixpkgs."aarch64-linux";
      buildBinariesForHost "aarch64-linux" [
        alejandra
        pkgsStatic.alejandra
      ];
    packages."i686-linux" = with nixpkgs."i686-linux";
      buildBinariesForHost "i686-linux" [
        alejandra
      ];
    packages."x86_64-darwin" = with nixpkgs."x86_64-darwin";
      buildBinariesForHost "x86_64-darwin" [
        alejandra
      ];
    packages."x86_64-linux" = with nixpkgs."x86_64-linux";
      (buildBinariesForHost "x86_64-linux" [
        alejandra
        pkgsStatic.alejandra

        pkgsCross.aarch64-multiplatform.pkgsStatic.alejandra

        pkgsCross.armv7l-hf-multiplatform.pkgsStatic.alejandra

        pkgsCross.gnu32.pkgsStatic.alejandra

        pkgsCross.raspberryPi.pkgsStatic.alejandra
      ])
      // {
        "alejandra-vscode-vsix" = mkYarnPackage {
          name = "alejandra";
          src = ./integrations/vscode;
          packageJSON = ./integrations/vscode/package.json;
          yarnLock = ./integrations/vscode/yarn.lock;
          yarnNix = ./integrations/vscode/yarn.lock.nix;
        };
      };
  };
}
