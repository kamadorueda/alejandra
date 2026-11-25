{
  lib,
  foo,
}:
assert foo != null;
with lib;
with foo;
let
  a = 1;
  b = 2;
in
  a
