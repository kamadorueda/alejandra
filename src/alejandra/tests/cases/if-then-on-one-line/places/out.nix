let
  inherit (lib) mkIf;
  a =
    if x then
      1
    else
      2;
  b =
    f (
      if x then
        1
      else
        2
    ) (
      if y then
        3
      else
        4
    );
  c = [
    (
      if x then
        1
      else
        2
    )
  ];
  d = {
    e =
      if x then
        {f = 1;}
      else
        [2];
  };
  g = x:
    if x then
      1
    else
      2;
  h = {
    x,
    y ?
      if z then
        1
      else
        2,
  }:
    if x then
      y
    else
      2;
  i = "pre ${
    if x then
      "a"
    else
      "b"
  } post";
  j = ''
    pre ${
      if x then
        "a"
      else
        "b"
    } post
  '';
  k =
    (
      if x then
        1
      else
        2
    )
    + (
      if y then
        3
      else
        4
    );
  l = with pkgs;
    if x then
      a
    else
      b;
  m = assert x;
    if y then
      1
    else
      2;
  n =
    if x then
      1
    else
      2 + 3;
  o = lib.optionals (
    if x then
      true
    else
      false
  ) [a b];
  p =
    (
      if x then
        f
      else
        g
    )
    arg;
  q =
    if x then
      if y then
        1
      else
        2
    else if z then
      3
    else
      4;
in
  if a then
    b
  else
    c
