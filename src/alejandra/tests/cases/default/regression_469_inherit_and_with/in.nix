{
  inherit (a . b . c) d e;
  inherit (x) y;
  f = with a . b; c . d;
  g = (a . b) . c;
}
