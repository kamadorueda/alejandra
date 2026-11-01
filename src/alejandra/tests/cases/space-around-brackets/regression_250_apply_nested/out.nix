{
  x =
    f (a:
      g (b: {
        c = a;
        d = b;
      }))
    y;
  z = map (x: x + 1) (filter (y: y > 2) [ 1 2 3 ]);
}
