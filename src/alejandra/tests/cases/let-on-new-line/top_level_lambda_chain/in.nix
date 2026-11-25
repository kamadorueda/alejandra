{lib, ...}: pkgs: system: let
  x = 1;
  y = 2;
in {
  inherit x y;
}
