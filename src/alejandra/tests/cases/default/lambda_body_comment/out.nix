let
  loadField = field: val:
    # This comment is indented with the body
    if field == val
    then "foo"
    else "bar";
  two = a: b:
    # first
    # second
    foo a b;
in {
  inherit loadField two;
  nested = x:
    # nested comment
    foo x;
}
