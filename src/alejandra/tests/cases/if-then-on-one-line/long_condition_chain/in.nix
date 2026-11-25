if
  lib.hasPrefix "aarch64" system
  && lib.versionAtLeast version "1.2.3"
  && builtins.elem name [ "one" "two" "three" "four" "five" "six" "seven" "eight" ]
then
  first
else if
  other
  || another
then second
else if short then third
else
  fourth
