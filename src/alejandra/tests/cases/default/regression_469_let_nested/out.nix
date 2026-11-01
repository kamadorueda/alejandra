let
  a.b = {c = 1;};
  x = a.b.c;
  y = {z.w = 1;}.z.w;
in
  x.y
