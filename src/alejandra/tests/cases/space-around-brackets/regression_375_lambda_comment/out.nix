[
  (
    pkgs:
    # Overlay to be passed
    final:
    prev: let
      foo = 5;
    in
      foo
  )

  (
    # before
    a: b: c: d
  )

  (a: b: c:
    # after
    d)

  (
    a:
    # one
    b:
    c:
    # two
    d:
    e:
      f
  )

  (a:
    # inline
    b:
    c: d)

  (a: b:
    # inline
    c: d)

  (a:
    /* block */
    b:
    c: d)

  (a: b: c: d)

  (a:
    # c
    b:
    c: let
      x = 1;
    in
      x)

  (
    cueSchemaFile:
    # after the first argument
    {
      document ? null,
    }:
      document
  )

  (
    a:
    # c
    b: { c }:
    d: e
  )

  (
    a:
    # c
    args @ { b, ... }:
    c: d
  )
]
