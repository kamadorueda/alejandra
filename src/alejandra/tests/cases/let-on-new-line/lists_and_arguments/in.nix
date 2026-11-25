{
  list = [
    (let
      a = 1;
      b = 2;
    in a)
    (let a = 1; in a)
    ({x}: let
      y = 1;
      z = 2;
    in x)
  ];
  apply = f (let
    a = 1;
    b = 2;
  in a) 2;
  apply2 = f 1 (let
    a = 1;
    b = 2;
  in {inherit a b;});
  lambda_arg = map (x: let
    y = x;
    z = 2;
  in y + z) list;
  select = (let
    a = {b = 1;};
    c = 2;
  in a).b;
  string = "${let
    a = 1;
    b = 2;
  in a}";
  default = {a ? let
    x = 1;
    y = 2;
  in x}: a;
}
