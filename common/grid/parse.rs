use super::*;

impl <'inp, Item, Pos> FromParser <'inp> for Grid <Vec <Item>, Pos, 2>
	where
		Item: Clone + Default + FromParser <'inp>,
		Pos: GridPosDisplay {

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
		use GridPosDisplayOrder::{ RightDown, RightUp, UpRight };
		let (grid_size, line_offset, tile_offset, first_idx) = match Pos::ORDER {
			RightDown => ([ height, width ], width.as_isize (), 1_isize, 0),
			RightUp => ([ height, width ], width.as_isize (), -1_isize, width.as_isize () * (height.as_isize () - 1)),
			UpRight => ([ width, height ], -1_isize, height.as_isize (), height.as_isize () - 1),
		};
		if ! Self::validate_dims (grid_origin, grid_size) { return Err (parser.err ()) }
		let mut grid_vec = vec! [ default (); width * height ];
		let mut line_idx = first_idx;
		for line in lines.iter () {
			let mut tile_idx = line_idx;
			for tile in line.iter ().cloned ()
					.chain (std::iter::repeat (Item::default ()))
					.take (width) {
				grid_vec [tile_idx.as_usize ()] = tile;
				tile_idx += tile_offset;
			}
			line_idx += line_offset;
		}
		Ok (Self::wrap (grid_vec.into_iter ().collect (), grid_origin, grid_size))
	}

}
