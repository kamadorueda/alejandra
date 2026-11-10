{
  a = /* sh */ ''
    echo "hello"
  '';
  b = /* lua */ "vim.cmd('q')";
  c = /*python*/ ''
    print("hello")
  '';
  d =
    /* sh */ ''
    echo "hello"
  '';
  e = /* a */ /* b */ "x";
  f = /* not a string */ 1;
  g = /*
    several
    lines
  */ "x";
  h = /** doc */ "x";
  /* before */ i = 1;
  j = [ /* one */ 1 ];
}
