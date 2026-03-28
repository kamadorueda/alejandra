let
	inner = ''
    inner line 1
    inner line 2
  '';
in {
	outer = ''
    before ${inner}
    after
  '';
}
