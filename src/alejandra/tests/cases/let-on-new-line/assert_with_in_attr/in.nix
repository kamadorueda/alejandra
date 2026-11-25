{
  a = assert x; let
    a = 1;
    b = 2;
  in a;
  b = with lib; let
    a = 1;
    b = 2;
  in [a b];
  c = assert x; with lib; let
    a = 1;
    b = 2;
  in a;
  d = {pkgs}: with pkgs; let
    a = 1;
    b = 2;
  in a;
  e = with lib; let a = 1; in a;
}
