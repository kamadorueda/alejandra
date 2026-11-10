{
  a =
    with pkgs;
    with lib;
    {
      x = 1;
    };
  b = with pkgs; with lib; [ x y ];
  c = with pkgs; with lib; with stdenv; f x;
  d = with pkgs; {
    x = 1;
  };
  e = with pkgs;
    with lib;
      [
        x
      ];
  f = with a; with b; with c; with d; with e; with f; with g; with h; with i; with j; [ x ];
}

