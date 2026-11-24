{
  a = { lib }:
    assert lib != null;
    let
      x = 1;
      y = 2;
    in
      x + y;
  b = { lib }:
    with lib;
    let
      x = 1;
    in
      x;
  chain1 = { lib }:
    with lib;
    with builtins;
    let
      x = 1;
      y = 2;
    in
      x + y;
  chain2 = { lib }:
    assert lib != null;
    with lib;
    let
      x = 1;
      y = 2;
    in
      x + y;
  chain3 = { lib }:
    assert lib != null;
    assert lib != 1;
    let
      x = 1;
      y = 2;
    in
      x + y;
  lambda = { lib }: let
    x = 1;
    y = 2;
  in
    x + y;
  inLet = let
    f = { lib }:
      with lib;
      let
        x = 1;
        y = 2;
      in
        x + y;
    g =
      assert true;
      let
        x = 1;
        y = 2;
      in
        x + y;
  in
    f;
  commentAfterSemi = { lib }:
    with lib; # comment
    let
      x = 1;
      y = 2;
    in
      x + y;
  commentBeforeLet = { lib }:
    with lib;
    # comment
    let
      x = 1;
      y = 2;
    in
      x + y;
  single = { lib }: with lib; let x = 1; in x;
  singleAssert = assert true; let x = 1; in x;
  nested = { lib }:
    with lib;
    let
      inner =
        with lib;
        let
          x = 1;
          y = 2;
        in
          x + y;
    in
      inner;
  inList = [
    (with lib;
      let
        x = 1;
        y = 2;
      in
        x + y)
  ];
  type =
    with types;
    let
      a = 1;
    in
      either a b;
  g =
    assert true;
    let
      x = 1;
    in
      x;
  chainKey =
    with a;
    with b;
    let
      x = 1;
    in
      x;
  parenArg = f (with lib;
    let
      x = 1;
      y = 2;
    in
      x + y);
  parenChain = f (with a;
    with b; {
      a = 1;
    });
  parenApply =
    f (with lib;
      let
        x = 1;
      in
        x)
    arg;
  listItem = [
    (with lib;
      let
        x = 1;
      in
        x)
    1
  ];
  lambdaBody = x:
    with lib;
    let
      y = 1;
      z = 2;
    in
      y + z;
  lambdaInKey = f: x:
    assert x;
    let
      y = 1;
      z = 2;
    in
      y + z;
  inBody = let
    a = 1;
  in
    with lib;
    let
      y = 1;
      z = 2;
    in
      y + z;
  thenWith =
    if c
    then
      with lib;
      let
        y = 1;
        z = 2;
      in
        y + z
    else 2;
  elseWith =
    if c
    then 1
    else
      with lib;
      let
        y = 1;
        z = 2;
      in
        y + z;
  argument = f x (with lib;
    let
      y = 1;
      z = 2;
    in
      y + z);
  withInAttrSet = {
    a =
      with lib;
      let
        y = 1;
        z = 2;
      in
        y + z;
    b = [
      (assert x;
        let
          y = 1;
          z = 2;
        in
          y + z)
    ];
  };
  thenLet =
    if true
    then let
      x = 1;
      y = 2;
    in
      x + y
    else 1;
}
