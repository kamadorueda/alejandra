{
  a = pkgs.writeShellScript "a" /* sh */ ''
    echo a
  '';
  b = writeText "b" /* json */ "{}";
  c = f /* sh */ ''
    echo c
  '';
  d = f /* sh */ "x" "y";
  e = f /* sh */ ''
    x
  '' {inherit y;};
  g =
    f
    /* a */
    /* b */
    "x";
  h =
    f
    /* sh */
    ''
      echo h
    '';
  i =
    f
    /* not a string */
    x;
  j =
    f
    /*
    several lines
    */
    "x";
  k = f "k" /* sh */ ''
    ${lib.concatMapStrings (x: ''
        echo ${x}
      '')
      xs}
  '';
  l = [
    (writeShellScript "l" /* sh */ ''
      echo l
    '')
  ];
  m =
    f /* sh */ "x"
    /* after */
    ;
  n =
    f # sh
    "x";
  o = lib.mkOption {
    default = f /* sh */ "x";
  };
  p = map (x: f /* sh */ "x ${x}") xs;
  q =
    f /* sh */ ''
      x
    ''
    + "y";
  r = writeShellScript "r" /** sh */ ''
    echo r
  '';
}
