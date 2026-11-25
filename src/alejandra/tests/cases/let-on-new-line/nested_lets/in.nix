let
  a = let
    b = 1;
    c = 2;
  in b + c;
  d = let e = 1; in e;
in let
  f = 1;
  g = 2;
in {
  inherit a d f g;
  h = let
    i = let
      j = 1;
      k = 2;
    in j;
    l = 2;
  in i + l;
}
