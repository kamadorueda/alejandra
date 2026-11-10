[
  ({
    fetchFromGitHub,
    lib,
    stdenv,

    cmake,
    epoxy,

    withColorScheme ? true,
    withKstyle ? false,
  }:
    null)
  ({
    a,
    b,
  }:
    null)
  ({
    a,
    b,

    c,
  }:
    null)
  ({
    a, # one

    b,
    # two
    c,

    # three
    d,
    ...
  }:
    null)
  ({a, ...}: null)
  ({a, ...}: null)
  ({
      a,

      b,
    } @ args:
      null)
  (args @ {
    a,

    b,
  }:
    null)
  ({
    a,
    /* c */
    b,
  }:
    null)
]
