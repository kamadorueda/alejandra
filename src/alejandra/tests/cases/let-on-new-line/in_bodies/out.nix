{
  attr_set =
    let
      a = 1;
      b = 2;
    in {inherit a b;};
  list =
    let
      a = 1;
      b = 2;
    in [a b];
  apply =
    let
      a = 1;
      b = 2;
    in
      f a b;
  if_else =
    let
      a = 1;
      b = 2;
    in
      if a
      then b
      else a;
  string =
    let
      a = 1;
      b = 2;
    in "${toString a}";
  multi_line_string =
    let
      a = 1;
      b = 2;
    in ''
      text ${toString a}
    '';
  paren =
    let
      a = 1;
      b = 2;
    in (a + b);
  lambda =
    let
      a = 1;
      b = 2;
    in
      x: a + x;
  bin_op =
    let
      a = 1;
      b = 2;
    in
      a + b;
  ident =
    let
      a = 1;
      b = 2;
    in
      a;
  with_ =
    let
      a = 1;
      b = 2;
    in
      with lib; a;
  empty =
    let
    in
      1;
}
