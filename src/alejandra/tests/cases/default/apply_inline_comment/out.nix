{
  apply =
    this # is a
    test;
  applyMore =
    foo # x
    # y
    bar
    baz;
  assertion = assert true; #
    1;
  withScope = with x; # w
    y;
}
