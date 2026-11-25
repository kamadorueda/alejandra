{
  a =
    if c
    then
      let
        x = 1;
        y = 2;
      in
        x
    else b;
  b =
    if c
    then a
    else
      let
        x = 1;
        y = 2;
      in
        x;
  c =
    if c
    then
      let
        x = 1;
        y = 2;
      in
        x
    else
      let
        z = 1;
        w = 2;
      in
        z;
  d =
    if c
    then let x = 1; in x
    else let y = 2; in y;
  e =
    if c
    then 1
    else if d
    then
      let
        x = 1;
        y = 2;
      in
        x
    else 3;
  f =
    if
      let
        x = 1;
        y = 2;
      in
        x
    then 1
    else 2;
  g = x:
    if c
    then
      let
        a = 1;
        b = 2;
      in
        a
    else b;
}
