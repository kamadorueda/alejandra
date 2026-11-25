{
  a = let x = 1; in x;
  b = let x = 1; in x + 1;
  c = x: let y = x; in y;
  d = if c then let x = 1; in x else 2;
  e = f (let x = 1; in x);
  f = [(let x = 1; in x)];
  g = with lib; let x = 1; in x;
  h = let
    x = 1;
  in x;
  i = let in 1;
  j = let x = 1; in let y = 2; in x + y;
}
