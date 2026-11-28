{
	a = {
		b = {
			c = {
				inherit (mkFruit {
					a = "apple";
					b = "banana";
					c = "cherry";
					d = "date";
					e = "elderberry";
				}) feijoa grape honeydew;
				inherit (f x {
					a = "apple";
					b = "banana";
					c = "cherry";
					d = "date";
					e = "elderberry";
				}) feijoa grape honeydew;
				inherit (callPackage ./fruit.nix {
					a = "apple";
					b = "banana";
					c = "cherry";
					d = "date";
					e = "elderberry";
				}) feijoa grape;
				inherit (builtins.concatLists ["apple" "banana" "cherry" "date" "elderberry" "fig" "grape" "honeydew"]) a b;
				inherit ({
					a = "apple";
					b = "banana";
					c = "cherry";
					d = "date";
					e = "elderberry";
				}) a b;
			};
		};
	};

	let-in = let
		inherit (mkFruit {
			a = "apple";
			b = "banana";
			c = "cherry";
			d = "date";
			e = "elderberry";
		}) feijoa grape honeydew;
	in
		feijoa;

	simple = f (mkFruit {
		a = "apple";
		b = "banana";
		c = "cherry";
		d = "date";
		e = "elderberry";
	});
}
