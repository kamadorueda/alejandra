{
  a = {x}: let
    y = x;
    z = 1;
  in y;
  b = x: y: let
    z = x;
    w = y;
  in z;
  c = {x}: {y}: let
    z = x;
    w = y;
  in {inherit z w;};
  d = {
    e = {x}: let
      y = x;
      z = 1;
    in y;
    f = {
      g = x: let
        h = 1;
        i = 2;
      in h;
    };
  };
}
