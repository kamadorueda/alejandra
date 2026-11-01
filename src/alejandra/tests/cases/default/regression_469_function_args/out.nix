{
  a = f x.y z.w;
  b = f (x.y) (z.w);
  c = x.y x.y;
  d = [a.b c.d];
  e = a.b + c.d;
  f =
    if a.b
    then c.d
    else e.f;
}
