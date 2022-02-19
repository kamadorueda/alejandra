{
  description = "The Uncompromising Nix Code Formatter";

  inputs = {
    flakeCompat.url = github:edolstra/flake-compat;
    flakeCompat.flake = false;

    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs: let
    commit = inputs.self.shortRev or "dirty";
    date = inputs.self.lastModifiedDate or inputs.self.lastModified or "19700101";
    version = "0.3.0+${builtins.substring 0 8 date}.${commit}";

    nixpkgsForHost = host:
      import inputs.nixpkgs {
        overlays = [
          (
            self: super: {
              alejandra = self.rustPlatform.buildRustPackage {
                pname = "alejandra";
                inherit version;
                src = self.stdenv.mkDerivation {
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
                  version = self.testVersion {package = super.alejandra;};
                };

                meta = {
                  description = "The Uncompromising Nix Code Formatter.";
                  homepage = "https://github.com/kamadorueda/alejandra";
                  license = self.lib.licenses.unlicense;
                  maintainers = [self.lib.maintainers.kamadorueda];
                  platforms = self.lib.systems.doubles.all;
                };
              };
            }
          )
        ];
        system = host;
      };
  in rec {
    checks."aarch64-darwin" = packages."aarch64-darwin";
    checks."aarch64-linux" = packages."aarch64-linux";
    checks."x86_64-darwin" = packages."x86_64-darwin";
    checks."x86_64-linux" = packages."x86_64-linux";

    defaultPackage."aarch64-darwin" = packages."aarch64-darwin"."alejandra-aarch64-apple-darwin";
    defaultPackage."aarch64-linux" = packages."aarch64-linux"."alejandra-aarch64-unknown-linux-musl";
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
          jq
          nodejs
          nodePackages.prettier
          nodePackages.prettier-plugin-toml
          shfmt
          treefmt
          yarn
          yarn2nix
        ];
      };

    nixpkgs."aarch64-darwin" = nixpkgsForHost "aarch64-darwin";
    nixpkgs."aarch64-linux" = nixpkgsForHost "aarch64-linux";
    nixpkgs."x86_64-darwin" = nixpkgsForHost "x86_64-darwin";
    nixpkgs."x86_64-linux" = nixpkgsForHost "x86_64-linux";

    packages."aarch64-darwin" = with nixpkgs."aarch64-darwin"; {
      "alejandra-aarch64-apple-darwin" = alejandra;
    };
    packages."aarch64-linux" = with nixpkgs."aarch64-linux"; {
      "alejandra-aarch64-unknown-linux-musl" = alejandra;
    };
    packages."x86_64-darwin" = with nixpkgs."x86_64-darwin"; {
      "alejandra-x86_64-apple-darwin" = alejandra;
    };
    packages."x86_64-linux" = with nixpkgs."x86_64-linux"; let
      binaries = with pkgsCross; (
        builtins.listToAttrs
        (
          builtins.map
          (
            pkg: {
              name = "alejandra-${pkg.stdenv.targetPlatform.config}";
              value = pkg;
            }
          )
          [
            aarch64-multiplatform.alejandra
            aarch64-multiplatform.pkgsStatic.alejandra

            alejandra
            pkgsStatic.alejandra

            armv7l-hf-multiplatform.alejandra
            armv7l-hf-multiplatform.pkgsStatic.alejandra

            gnu32.alejandra
            gnu32.pkgsStatic.alejandra

            raspberryPi.alejandra
            raspberryPi.pkgsStatic.alejandra

            #  ben-nanonote.alejandra
            #  fuloongminipc.alejandra
          ]
        )
      );
    in
      binaries
      // {
        "alejandra-binaries" = linkFarm "alejandra-binaries" (
          lib.mapAttrsToList
          (
            name: path: {
              inherit name;
              path = "${path}/bin/alejandra";
            }
          )
          binaries
        );
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
