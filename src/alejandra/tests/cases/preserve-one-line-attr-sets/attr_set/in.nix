[
  {}
  {a = 1;}
  {a = 1; b = 2;}
  { a = 1;   b = 2; c = 3; }
  rec {a = 1; b = a;}
  {inherit a b; inherit (c) d; e = 1;}
  {a = {b = 1; c = 2;}; d = [1 2];}
  {
    a = 1; b = 2;
  }
  {a = 1; /* comment */ b = 2;}
  {a = 1; # comment
   b = 2;}
  {a = 1; b = {
    c = 2;
  };}
  {
    x = {a = 1; b = 2;};
    y = let z = {a = 1; b = 2;}; in z;
  }
]
