use super::*;

impl <'inp, Storage, Pos> GridBuf <Storage, Pos, 2>
	where
		Pos: GridPosParse,
		Storage: Clone + FromIterator <Storage::Item> + GridStorage,
		Storage::Item: Clone + Default + FromParser <'inp> {

	#[ allow (clippy::missing_inline_in_public_items) ]
	pub fn parse_with_delim (
		parser: & mut Parser <'inp>,
		delim: impl ParseDelimiter,
	) -> ParseResult <Self> {
		let lines: Vec <Vec <Storage::Item>> = parser.delim_fn ("\n", |parser| {
			parser.any ().of (|parser| {
				let items: Vec <Storage::Item> = parser.delim_fn (delim, Parser::item).collect ();
				if items.is_empty () { return Err (parser.err ()) }
				Ok (items)
			}).done ()
		}).collect ();
		if lines.is_empty () { return Err (parser.err ()) }
		let num_lines = lines.len ();
		let num_lines_coord = Pos::Coord::from_usize (num_lines).map_err (|_err| parser.err ()) ?;
		let num_cols = lines.iter ().map (Vec::len).max ().unwrap_or (0);
		let num_cols_coord = Pos::Coord::from_usize (num_cols).map_err (|_err| parser.err ()) ?;
		let grid_origin = Pos::from_array ([Pos::Coord::ZERO; 2]);
		let grid_size = Pos::grid_parse_grid_size (num_lines_coord, num_cols_coord);
		let line_offset = Pos::grid_parse_row_offset (num_lines, num_cols);
		let tile_offset = Pos::grid_parse_col_offset (num_lines, num_cols);
		let first_idx = Pos::grid_parse_first_index (num_lines, num_cols).pan_isize ();
		if ! Pos::validate_dims (grid_origin, grid_size) { return Err (parser.err ()) }
		let mut grid_vec = vec! [ default (); num_lines * num_cols ];
		let mut line_idx = first_idx;
		for line in lines.iter () {
			let mut tile_idx = line_idx;
			for tile in line.iter ().cloned ()
					.chain (std::iter::repeat (default ()))
					.take (num_cols) {
				grid_vec [tile_idx.pan_usize ()] = tile;
				tile_idx += tile_offset;
			}
			line_idx += line_offset;
		}
		Ok (Self::wrap (grid_vec.into_iter ().collect (), grid_origin, grid_size))
	}

}

impl <'inp, Storage, Pos> FromParser <'inp> for GridBuf <Storage, Pos, 2>
	where
		Pos: GridPosParse,
		Storage: Clone + FromIterator <Storage::Item> + GridStorage,
		Storage::Item: Clone + Default + FromParser <'inp> {

	#[ allow (clippy::missing_inline_in_public_items) ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let lines: Vec <Vec <Storage::Item>> = parser.delim_fn ("\n", |parser| {
			parser.any ().of (|parser| {
				let items: Vec <Storage::Item> = parser.repeat (Parser::item).collect ();
				if items.is_empty () { return Err (parser.err ()) }
				Ok (items)
			}).done ()
		}).collect ();
		if lines.is_empty () { return Err (parser.err ()) }
		let num_lines = lines.len ();
		let num_lines_coord = Pos::Coord::from_usize (num_lines).map_err (|_err| parser.err ()) ?;
		let num_cols = lines.iter ().map (Vec::len).max ().unwrap_or (0);
		let num_cols_coord = Pos::Coord::from_usize (num_cols).map_err (|_err| parser.err ()) ?;
		let grid_origin = Pos::from_array ([Pos::Coord::ZERO; 2]);
		let grid_size = Pos::grid_parse_grid_size (num_lines_coord, num_cols_coord);
		let line_offset = Pos::grid_parse_row_offset (num_lines, num_cols);
		let tile_offset = Pos::grid_parse_col_offset (num_lines, num_cols);
		let first_idx = Pos::grid_parse_first_index (num_lines, num_cols).pan_isize ();
		if ! Pos::validate_dims (grid_origin, grid_size) { return Err (parser.err ()) }
		let mut grid_vec = vec! [ default (); num_lines * num_cols ];
		let mut line_idx = first_idx;
		for line in lines.iter () {
			let mut tile_idx = line_idx;
			for tile in line.iter ().cloned ()
					.chain (std::iter::repeat (default ()))
					.take (num_cols) {
				grid_vec [tile_idx.pan_usize ()] = tile;
				tile_idx += tile_offset;
			}
			line_idx += line_offset;
		}
		Ok (Self::wrap (grid_vec.into_iter ().collect (), grid_origin, grid_size))
	}

}

pub trait GridPosParse: GridPos <2> {
	fn grid_parse_grid_size (rows: Self::Coord, cols: Self::Coord) -> Self;
	fn grid_parse_row_offset (rows: usize, cols: usize) -> isize;
	fn grid_parse_col_offset (rows: usize, cols: usize) -> isize;
	fn grid_parse_first_index (rows: usize, cols: usize) -> usize;
}

impl <Pos: GridPosDisplayAuto> GridPosParse for Pos {

	#[ inline ]
	fn grid_parse_grid_size (rows: Self::Coord, cols: Self::Coord) -> Self {
		use GridPosDisplayType::{ DownRight, RightUp, UpRight, UpRightSlant };
		Self::from_array (match Self::DISPLAY_TYPE {
			DownRight | UpRight | UpRightSlant => [ rows, cols ],
			RightUp => [ cols, rows ],
		})
	}

	#[ inline ]
	fn grid_parse_row_offset (_rows: usize, cols: usize) -> isize {
		use GridPosDisplayType::{ DownRight, RightUp, UpRight, UpRightSlant };
		match Self::DISPLAY_TYPE {
			DownRight | UpRight | UpRightSlant => cols.pan_isize (),
			RightUp => -1_isize,
		}
	}

	#[ inline ]
	fn grid_parse_col_offset (rows: usize, _cols: usize) -> isize {
		use GridPosDisplayType::{ DownRight, RightUp, UpRight, UpRightSlant };
		match Self::DISPLAY_TYPE {
			DownRight => 1_isize,
			RightUp => rows.pan_isize (),
			UpRight | UpRightSlant => -1_isize,
		}
	}

	#[ inline ]
	fn grid_parse_first_index (rows: usize, cols: usize) -> usize {
		use GridPosDisplayType::{ DownRight, RightUp, UpRight, UpRightSlant };
		match Self::DISPLAY_TYPE {
			DownRight => 0_usize,
			RightUp => rows.pan_usize () - 1,
			UpRight | UpRightSlant => (rows.pan_usize () - 1) * cols.pan_usize (),
		}
	}

}
