{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = inputs @ {self, nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.hello;
  };
}
