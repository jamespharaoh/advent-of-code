use super::*;
use model::Grid;
use model::Tile;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub grid: Grid,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}

impl Input {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params);
			let tiles_temp: Vec <Vec <Tile>> = parser
				.delim_fn ("\n", |parser| Ok (parser.repeat (Parser::item).collect ()))
				.try_collect () ?;
			let rows = tiles_temp.len ();
			if ! (1 ..= 256).contains (& rows) {
				return Err ("Must have between 1 and 256 rows".into ());
			}
			let cols = tiles_temp.iter ().map (Vec::len).max ().unwrap ();
			if ! (1 ..= 256).contains (& cols) {
				return Err ("Must have between 1 and 256 cols".into ());
			}
			let tiles: Vec <Tile> =
				tiles_temp.iter ()
					.flat_map (|row_data| row_data.iter ().copied ()
						.chain (iter::repeat (Tile::Empty))
						.take (cols))
					.collect ();
			let grid = Grid::wrap (tiles, [0, 0], [rows, cols]);
			Ok (Self { grid, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}", self.grid.print (|tile| tile.as_char ())) ?;
		Ok (())
	}
}
