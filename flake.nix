{
  description = "The Uncompromising Nix Code Formatter";

  inputs = {
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    flakeCompat.url = github:edolstra/flake-compat;
    flakeCompat.flake = false;

    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = inputs: let
    build = host: target: let
      nixpkgs = import inputs.nixpkgs {
        system = host;
        crossSystem = builtins.getAttr target {
          "aarch64-apple-darwin" = inputs.nixpkgs.lib.systems.examples.aarch64-darwin;
          "aarch64-unknown-linux-musl".config = "aarch64-unknown-linux-musl";
          "x86_64-apple-darwin" = null;
          "x86_64-unknown-linux-gnu" = null;
          "x86_64-unknown-linux-musl".config = "x86_64-unknown-linux-musl";
        };
      };

      fenix = inputs.fenix.packages.${host};
      rustPlatform = nixpkgs.makeRustPlatform {
        cargo = fenix.latest.cargo;
        rustc = fenix.combine [
          fenix.latest.rustc
          fenix.targets.${target}.latest.rust-std
        ];
      };
    in
      rec {
        bin = rustPlatform.buildRustPackage {
          pname = "alejandra";
          version =
            let
              commit = inputs.self.shortRev or "dirty";
              date = inputs.self.lastModifiedDate or inputs.self.lastModified or "19700101";
            in
              "0.1.0+${builtins.substring 0 8 date}.${commit}";
          src = ./.;
          inherit target;
          cargoLock.lockFile = ./Cargo.lock;
          meta = {
            description = "The Uncompromising Nix Code Formatter.";
            homepage = "https://github.com/kamadorueda/alejandra";
            license = nixpkgs.lib.licenses.unlicense;
            maintainers = [ nixpkgs.lib.maintainers.kamadorueda ];
          };
        };
        integrations.vscode-env = nixpkgs.mkYarnPackage {
          name = "alejandra";
          src = ./integrations/vscode;
          packageJSON = ./integrations/vscode/package.json;
          yarnLock = ./integrations/vscode/yarn.lock;
          yarnNix = ./integrations/vscode/yarn.lock.nix;
        };
        integrations.vscode = nixpkgs.stdenv.mkDerivation {
          name = "alejandra";
          src = ./integrations/vscode;
          env = integrations.vscode-env;
          buildInputs = [ nixpkgs.yarn ];
          builder = builtins.toFile "builder.sh" ''
            source $stdenv/setup

            cd $src
            $env/libexec/alejandra/node_modules/.bin/vsce package --out $out
          '';
        };
        shell = nixpkgs.mkShell {
          name = "alejandra";
          packages = [
            fenix.latest.toolchain
            nixpkgs.cargo-tarpaulin
            nixpkgs.jq
            nixpkgs.nodejs
            nixpkgs.nodePackages.prettier
            nixpkgs.nodePackages.prettier-plugin-toml
            nixpkgs.shfmt
            nixpkgs.treefmt
            nixpkgs.yarn
            nixpkgs.yarn2nix
          ];
        };
      };
  in
    rec {
      checks."aarch64-darwin" = packages."aarch64-darwin";
      checks."x86_64-darwin" = packages."x86_64-darwin";
      checks."aarch64-linux" = packages."aarch64-linux";
      checks."x86_64-linux" = packages."x86_64-linux";

      defaultPackage."aarch64-darwin" =
        packages."aarch64-darwin"."aarch64-apple-darwin";

      defaultPackage."aarch64-linux" =
        packages."aarch64-linux"."aarch64-unknown-linux-musl";

      defaultPackage."x86_64-darwin" =
        packages."x86_64-darwin"."x86_64-apple-darwin";

      defaultPackage."x86_64-linux" =
        packages."x86_64-linux"."x86_64-unknown-linux-gnu";

      devShell."x86_64-linux" =
        (build "x86_64-linux" "x86_64-unknown-linux-gnu").shell;

      packages."aarch64-darwin"."aarch64-apple-darwin" =
        (build "aarch64-darwin" "aarch64-apple-darwin").bin;

      packages."aarch64-linux"."aarch64-unknown-linux-musl" =
        (build "aarch64-linux" "aarch64-unknown-linux-musl").bin;

      packages."x86_64-darwin"."x86_64-apple-darwin" =
        (build "x86_64-darwin" "x86_64-apple-darwin").bin;

      packages."x86_64-linux"."integrations-vscode" =
        (build "x86_64-linux" "x86_64-unknown-linux-gnu").integrations.vscode;

      packages."x86_64-linux"."aarch64-unknown-linux-musl" =
        (build "x86_64-linux" "aarch64-unknown-linux-musl").bin;
      packages."x86_64-linux"."x86_64-unknown-linux-gnu" =
        (build "x86_64-linux" "x86_64-unknown-linux-gnu").bin;
      packages."x86_64-linux"."x86_64-unknown-linux-musl" =
        (build "x86_64-linux" "x86_64-unknown-linux-musl").bin;
    };
}
