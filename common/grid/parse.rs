use super::*;

impl <'inp, Item, Pos> FromParser <'inp> for Grid <Vec <Item>, Pos, 2>
	where
		Item: Clone + Default + FromParser <'inp>,
		Pos: GridPos <2> {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let lines: Vec <Vec <Item>> = parser.delim_fn ("\n", |parser| {
			parser.any ().of (|parser| {
				let items: Vec <Item> = parser.repeat (Parser::item).collect ();
				if items.is_empty () { return Err (parser.err ()) }
				Ok (items)
			}).done ()
		}).collect ();
		if lines.is_empty () { return Err (parser.err ()) }
		let height = lines.len ();
		let width = lines.iter ().map (Vec::len).max ().unwrap_or (0);
		let grid_origin = [ 0, 0 ];
		let grid_size = [ height, width ];
		if ! Self::validate_dims (grid_origin, grid_size) { return Err (parser.err ()) }
		Ok (Self::wrap (
			lines.iter ()
				.flat_map (|line| line.iter ().cloned ()
					.chain (std::iter::repeat (Item::default ()))
					.take (width))
				.collect (),
			[ 0, 0 ],
			[ height, width ],
		))
	}

}
