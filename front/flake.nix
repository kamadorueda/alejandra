{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = inputs: let
    system = "x86_64-linux";

    fenix = inputs.fenix.packages.${system};
    nixpkgs = import inputs.nixpkgs { inherit system; };
  in
    {
      devShell.${system} = nixpkgs.mkShell {
        name = "alejandra";
        packages = [
          (
            fenix.combine [
              fenix.latest.rustc
              fenix.latest.toolchain
              fenix.targets."wasm32-unknown-unknown".latest.rust-std
            ]
          )
          nixpkgs.binaryen
          nixpkgs.pkg-config
          nixpkgs.openssl
          nixpkgs.yarn
          nixpkgs.wasm-pack
        ];
      };
    };
}
