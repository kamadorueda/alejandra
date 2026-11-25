{
  something = let
    foo = "abc";
  in
    function foo;

  outputs = inputs @ {
    self,
    nixpkgs,
    home-manager,
    ...
  }: let
    # Code elided
    x = 1;
  in {
    # Code elided
    inherit x;
  };

  flake = inputs.flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import inputs.nixpkgs {
      inherit system;
      overlays = [inputs.devshell.overlay];
    };
  in {
    devShells.default = pkgs.mkShell {};
  });

  y = f (system: let
    pkgs = 1;
  in pkgs);

  z = if a then let
    x = 1;
    y = 2;
  in x else b;
}
