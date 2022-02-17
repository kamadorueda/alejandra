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
    version = "0.2.0+${builtins.substring 0 8 date}.${commit}";

    nixpkgsForHost = host:
      import inputs.nixpkgs {
        overlays = [
          (
            self: super: {
              alejandra = self.rustPlatform.buildRustPackage {
                pname = "alejandra";
                inherit version;
                src = ./.;

                cargoLock.lockFile = ./Cargo.lock;

                passthru.tests = {
                  version = nixpkgs.testVersion { package = super.alejandra; };
                };

                meta = {
                  description = "The Uncompromising Nix Code Formatter.";
                  homepage = "https://github.com/kamadorueda/alejandra";
                  license = self.lib.licenses.unlicense;
                  maintainers = [nixpkgs.lib.maintainers.kamadorueda];
                };
              };
            }
          )
        ];
        system = host;
      };

    nixpkgs."aarch64-darwin" = nixpkgsForHost "aarch64-darwin";
    nixpkgs."aarch64-linux" = nixpkgsForHost "aarch64-linux";
    nixpkgs."x86_64-darwin" = nixpkgsForHost "x86_64-darwin";
    nixpkgs."x86_64-linux" = nixpkgsForHost "x86_64-linux";
  in rec {
    checks."aarch64-darwin" = packages."aarch64-darwin";
    checks."aarch64-linux" = packages."aarch64-linux";
    checks."x86_64-darwin" = packages."x86_64-darwin";
    checks."x86_64-linux" = packages."x86_64-linux";

    defaultPackage."aarch64-darwin" = packages."aarch64-darwin"."alejandra-aarch64-apple-darwin";
    defaultPackage."aarch64-linux" = packages."aarch64-linux"."alejandra-aarch64-unknown-linux-musl";
    defaultPackage."x86_64-darwin" = packages."x86_64-darwin"."alejandra-x86_64-apple-darwin";
    defaultPackage."x86_64-linux" = packages."x86_64-linux"."alejandra-x86_64-unknown-linux-gnu";

    devShell."x86_64-linux" =
      with nixpkgs."x86_64-linux";
      mkShell {
        name = "alejandra";
        packages = [
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

    inherit nixpkgs;

    packages."aarch64-darwin"."alejandra-aarch64-apple-darwin" = nixpkgs."aarch64-darwin".alejandra;
    packages."aarch64-linux"."alejandra-aarch64-unknown-linux-musl" = nixpkgs."aarch64-linux".alejandra;
    packages."x86_64-darwin"."alejandra-x86_64-apple-darwin" = nixpkgs."x86_64-darwin".alejandra;
    packages."x86_64-linux"."alejandra-aarch64-unknown-linux-musl" = nixpkgs."x86_64-linux".pkgsCross.aarch64-multiplatform-musl.alejandra;
    packages."x86_64-linux"."alejandra-x86_64-unknown-linux-gnu" = nixpkgs."x86_64-linux".alejandra;
    packages."x86_64-linux"."alejandra-x86_64-unknown-linux-musl" = nixpkgs."x86_64-linux".pkgsCross.musl64.alejandra;

    packages."x86_64-linux"."alejandra-vscode-vsix" = nixpkgs."x86_64-linux".mkYarnPackage {
      name = "alejandra";
      src = ./integrations/vscode;
      packageJSON = ./integrations/vscode/package.json;
      yarnLock = ./integrations/vscode/yarn.lock;
      yarnNix = ./integrations/vscode/yarn.lock.nix;
    };
  };
}
