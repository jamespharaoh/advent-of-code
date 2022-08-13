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
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		let rows_data_temp = Parser::wrap_lines_auto (
			input.iter ().copied ().enumerate (),
			|parser| {
				let mut tiles = ArrayVec::<Tile, 256>::new ();
				while parser.peek ().is_some () {
					if tiles.is_full () { return Err (parser.err ()) }
					tiles.push (parser.item () ?);
				}
				Ok (tiles)
			}) ?;
		let rows = rows_data_temp.len ();
		if ! (1 ..= 256).contains (& rows) {
			return Err ("Must have between 1 and 256 rows".into ());
		}
		let cols = rows_data_temp.iter ().map (ArrayVec::len).max ().unwrap ();
		if ! (1 ..= 256).contains (& cols) {
			return Err ("Must have between 1 and 256 cols".into ());
		}
		let tiles: Vec <Tile> =
			rows_data_temp.iter ()
				.flat_map (|row_data| row_data.iter ().copied ()
					.chain (iter::repeat (Tile::Empty))
					.take (cols))
				.collect ();
		let grid = Grid::wrap (tiles, [0, 0], [rows, cols]);
		Ok (Self { grid, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}", self.grid.print (|tile| tile.as_char ())) ?;
		Ok (())
	}
}
