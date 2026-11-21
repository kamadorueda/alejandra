[
  {
    inherit (mkFruit {
      a = "apple";
      b = "banana";
      c = "cherry";
      d = "date";
      e = "elderberry";
    }) feijoa grape honeydew;
  }

  {
    inherit (mkFruit {
      a = "apple";
      b = "banana";
    }) feijoa grape honeydew;
  }

  {inherit (builtins.concatLists [(mkFruit "apple") (mkFruit "banana") (mkFruit "cherry") (mkFruit "date") (mkFruit "elderberry")]) a b;}

  {
    inherit (builtins.concatLists [
      "apple"
      "banana"
    ]) a b;
  }

  {
    inherit (f x {
      a = "apple";
      b = "banana";
      c = "cherry";
      d = "date";
      e = "elderberry";
    }) feijoa grape honeydew;
  }

  {
    inherit ({
      a = "apple";
      b = "banana";
      c = "cherry";
      d = "date";
      e = "elderberry";
    }) a b;
  }

  {inherit (["apple" "banana"]) a b;}

  {inherit (pkgs) lib stdenv;}

  {
    inherit
      (f {
          a = "apple";
          b = "banana";
          c = "cherry";
          d = "date";
          e = "elderberry";
        }
        x)
      feijoa
      grape
      ;
  }

  {inherit (a.b.c) lib;}

  {
    inherit
      (mkFruit {
        a = "apple";
        b = "banana";
        c = "cherry";
        d = "date";
        e = "elderberry";
      })
      /* names */
      feijoa
      grape
      honeydew
      ;
  }

  {
    inherit
      (mkFruit {
        a = "apple";
        b = "banana";
        c = "cherry";
        d = "date";
        e = "elderberry";
        /* inside */
      })
      feijoa
      grape
      honeydew
      ;
  }

  {
    inherit
      /* first */
      (mkFruit {
        a = "apple";
        b = "banana";
        c = "cherry";
        d = "date";
        e = "elderberry";
      })
      feijoa
      grape
      honeydew
      ;
  }

  {
    inherit
      (mkFruit {
        a = "apple";
        b = "banana";
        c = "cherry";
        d = "date";
        e = "elderberry";
      })
      feijoa
      grape
      honeydew
      # trailing
      ;
  }

  {
    inherit
      (mkFruit {
        a = "apple";
        b = "banana";
        c = "cherry";
        d = "date";
        e = "elderberry";
      })
      feijoaAAAAAAAAAAAAAAAA
      graperrrrrrrrrrrrrrrrrrr
      honeydewwwwwwwwwwwwwwwwwwww
      kiwiiiiiiiiiiiiiiiiii
      ;
  }

  {
    inherit
      (mkFruit {
        a = "apple";
        b = "banana";
        c = "cherry";
        d = "date";
        e = "elderberry";
      })
      feijoaAAAAAAAAAAAAAAAA
      graperrrrrrrrrrrrrrrrrrrr
      honeydewwwwwwwwwwwwwwwwwwwwwwwwwwww
      ;
  }

  {
    inherit
      (mkFruit {
        a = "apple";
        b = "banana";
        c = "cherry";
        d = "date";
        e = "elderberry";
      })
      "quoted"
      ${dynamic}
      plain
      ;
  }

  {
    inherit (mkFruit {
      a = "apple";
      b = "banana";
      c = "cherry";
      d = "date";
      e = "elderberry";
    }) feijoa grape;
  }

  {
    outer = {
      inner = {
        deep = let
          inherit (mkFruit {
            a = "apple";
            b = "banana";
            c = "cherry";
            d = "date";
            e = "elderberry";
          }) feijoa grape honeydew;
        in
          feijoa;
      };
    };
  }

  {
    inherit
      (let
        x = {
          a = "apple";
          b = "banana";
          c = "cherry";
          d = "date";
          e = "elderberry";
        };
      in
        x)
      a
      ;
  }

  {
    inherit (callPackage ./fruit.nix {
      a = "apple";
      b = "banana";
      c = "cherry";
      d = "date";
      e = "elderberry";
    }) feijoa grape;
  }

  {
    inherit
      (''
        multi line
        string
      '')
      a
      b
      ;
  }
]
