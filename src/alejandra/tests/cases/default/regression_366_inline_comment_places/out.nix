let
  a = /* sh */ ''
    echo ${b}
  '';
  b
  /* key */
  = /* sh */ "x";
  c =
    /* sh */
    "x"
    /* after */
    ;
  d =
    /* sh */
    "x" + "y";
  e = /* sh */ ''
    ${lib.concatStringsSep "\n" [
      "a"
      "b"
    ]}
  ''; # end
  f = /* a * b */ "x";
  g = /*sh*/ "x";
  h = /*  two  spaces  */ "x";
  i.j.k = /* sh */ ''
    x
  '';
  "l" = /* sh */ "x";
  ${m} = /* sh */ "x";
  n = {
    o = /* sh */ ''
      x
    '';
  };
  p = pkgs.writeShellScript "p" /* sh */ ''
    echo p
  '';
  q = [
    /* sh */
    ''
      x
    ''
  ];
in
  {
    r,
    /* c */
    s ?
    /* c */
    1,
  }: t:
  /* c */
  u:
    if
      /* c */
      r
    then
      /* c */
      s
    else
      /* c */
      t
