[
  (if # c1
  a then b else c)
  (if a # c2
  then b else c)
  (if a then # c3
  b else c)
  (if a then b # c4
  else c)
  (if a then b else # c5
  c)
  (if /* c6 */ a then b else c)
  (if a /* c7 */ then b else c)
  (if a then /* c8 */ b else c)
  (if a then b /* c9 */ else c)
  (if a then b else /* c10 */ c)
  (if a then b else if c then d else # c11
  e)
  (if a then b else if c # c12
  then d else e)
  (if a then b
  # c13
  else c)
  (if a
  # c14
  then b else c)
  (if a then
  # c15
  b else c)
  (if a then b else
  # c16
  c)
  (if a then b else c # c17
  )
  (if a then b else c) # c18
]
