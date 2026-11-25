{
  a =
    let
      # c1
      x = 1; # c2
      # c3
    in
      # c4
      # c5
      x;
  b =
    /* c0 */
    let
      x = 1;
      y = 2;
    in
      x;
  c =
    let
      x = 1;
      y = 2;
    in
      /* c6 */
      {y = x;};
  d = x:
    /* cc */
    let
      x = 1;
      y = 2;
    in
      x;
  e =
    # k
    let
      x = 1;
      y = 2;
    in
      x;
  f =
    if c
    then # t
      let
        x = 1;
        y = 2;
      in
        x
    else
      let
        y = 2;
        z = 3;
      in
        y;
  g =
    with lib; # w
    let
      x = 1;
      y = 2;
    in
      x;
  h =
    let
      # leading
      x = 1;

      # between
      y = 2;
      # trailing
    in
      # before body
      x;
  i =
    let
      x = 1;
      y = 2;
    in
      x; # after
  # before j
  j =
    let
      x = 1;
      y = 2;
    in
      x;
  k = f (
    /* inner */
    let
      x = 1;
      y = 2;
    in
      x
  );
  l = f (
    let
      x = 1;
      y = 2;
    in
      x
    /* tail */
  );
  m = f ( # line
    let
      x = 1;
      y = 2;
    in
      x
  );
}
