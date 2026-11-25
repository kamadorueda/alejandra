{lib}:
let
  a =
    if cond
    then let
      x = 1;
      y = 2;
    in
      x + y
    else if other
    then let x = 1; in x
    else let
      z = 3;
      w = 4;
    in {
      inherit z w;
    };
  b = if c then {a = 1; b = 2;} else [1 2 3];
in
  a
