{
	v =
		if a
		then 1
		else if b
		then 2
		else x: y: x + y;
	w =
		if a
		then x: x
		else y: let z = y; in z;
}
