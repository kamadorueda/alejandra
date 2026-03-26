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
          (nixpkgs: _: {
            alejandra-frontend = nixpkgs.mkShell {
              name = "alejandra-frontend";
              packages = [
                nixpkgs.nodejs
                nixpkgs.nodePackages_latest.pnpm
                nixpkgs.mprocs
              ];
            };
          })
        ];
      };
    nixpkgs = forEachSystem nixpkgsForSystem;
  in {
    devShells = forEachSystem (system: {
      default = nixpkgs.${system}.alejandra-frontend;
    });
  };
}
