{
  a = /* sh */ ''
    echo ${b}
  '';
  c = { d = /* lua */ "x"; e = [ /* one */ 1 ]; };
  f = x /* c */ : y;
}
