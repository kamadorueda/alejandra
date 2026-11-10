{
  a = attrsOf <| submodule {
    # ...
  };

  b = attrsOf (submodule {
    # ...
  });

  c = f <| g <| {
    y = 2;
  };

  d =
    attrsOf
    <| submodule {
      x = 1;
    };

  e = [
    (attrsOf <| submodule {
      x = 1;
      y = 2;
    })
  ];

  f =
    attrsOf
    <| # comment
    submodule {
      x = 1;
    };
}
