let
  a = with pkgs; with lib; {
    x = 1;
  };
  b = assert x; assert y; {
    z = 1;
  };
  c =
    assert x;
    assert y;
    z;
  d = with pkgs; assert x; [
    y
  ];
  e = with pkgs; # comment
    with lib; x;
  f = with pkgs; f {
    x = 1;
  };
in
  with a;
  with b; {
    inherit c;
  }
