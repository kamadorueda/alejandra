{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
    systems.url = "github:nix-systems/default";
  };

  outputs = inputs: let
    supportedSystems = import inputs.systems;
    forEachSystem = inputs.nixpkgs.lib.genAttrs supportedSystems;
    nixpkgsForSystem = system:
      import inputs.nixpkgs {
        inherit system;
        overlays = [
          (nixpkgs: _: rec {
            alejandra-frontend = {
              deploy = nixpkgs.writeShellApplication {
                name = "deploy";
                runtimeInputs = [nixpkgs.nodejs];
                text = ''
                  pnpm run build
                  pnpm exec wrangler deploy
                '';
              };

              deployPreview = nixpkgs.writeShellApplication {
                name = "deploy-preview";
                runtimeInputs = [nixpkgs.nodejs];
                text = ''
                  pnpm run build
                  pnpm exec wrangler versions upload --preview-alias dev
                '';
              };

              shell = nixpkgs.mkShell {
                name = "alejandra-frontend";
                packages = [
                  alejandra-frontend.deploy
                  alejandra-frontend.deployPreview
                  nixpkgs.nodejs
                  nixpkgs.nodePackages_latest.pnpm
                  nixpkgs.mprocs
                ];
              };
            };
          })
        ];
      };
    nixpkgs = forEachSystem nixpkgsForSystem;
  in {
    devShells = forEachSystem (system: {
      default = nixpkgs.${system}.alejandra-frontend.shell;
    });

    packages = forEachSystem (system: {
      deploy = nixpkgs.${system}.alejandra-frontend.deploy;
      "deploy-preview" = nixpkgs.${system}.alejandra-frontend.deployPreview;
    });
  };
}
