[
  (if x then { a = 1; b = 2; } else { c = 3; })
  (if x then [ 1 2 3 ] else [ ])
  (if x then ''
    multi
    line
  '' else ''
    other
  '')
  (if x then "short" else "also short")
  (if x then let a = 1; in a else let b = 2; in b)
  (if x then y: y + 1 else y: y - 1)
  (if x then with pkgs; hello else null)
  (if x then f a b c else g { x = 1; })
  (if x then a // { b = 1; } else a)
  (if x then ./path else /abs/path)
]
