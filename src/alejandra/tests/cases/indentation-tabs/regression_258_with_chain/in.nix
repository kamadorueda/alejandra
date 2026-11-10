{
  a =
    with pkgs;
    with lib;
    {
      x = 1;
    };
  b = with pkgs; with lib; [ x ];
  c = assert x; assert y; {
    z = 1;
  };
}
