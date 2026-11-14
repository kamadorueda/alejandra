{a}: [
  {}
  {a = 1;}
  {  a = 1;  }
  {a = 1; b = 2;}
  {
    a = 1;
    b = 2;
  }
  rec {x = 1;}
  rec {  x = 1;  }
  ({b}: {inherit a b;})
  ({  b  }: {  inherit a b;  })
  ({b, c}: {inherit a b c;})
  ({  b, c  }: {  inherit a b c;  })
 ]
