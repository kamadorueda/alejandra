assert ltoSupport -> stdenv.isDarwin -> throw "LTO is broken on Darwin (see PR#19312).";
let
  flag = tf: x: [
    (
      if tf
      then "--enable-${x}"
      else "--disable-${x}"
    )
  ];
  other = 1;
in
  flag
