use super::*;

use model::Grid;
use model::Node;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub nodes: Grid,
	pub params: InputParams,
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub iters_one: u32 = ("ITERS_ONE=", 10_000, 1_u32 .. ),
		pub iters_two: u32 = ("ITERS_TWO=", 10_000_000, 1_u32 .. ),
	}
}

impl Input {
	pub fn parse (input: & [& str]) -> GenResult <Self> {
		Parser::wrap_lines (input, |parser| {
			parse! (parser, params);
			let nodes_temp: Vec <Vec <Node>> = parser
				.delim_fn ("\n", |parser| Ok (parser.repeat (Parser::item).collect ()))
				.collect ();
			let num_rows = nodes_temp.len ();
			if num_rows < 1 { return Err ("Must have at least one row".into ()) }
			if num_rows & 1 == 0 { return Err ("Must have an odd number of rows".into ()) }
			let num_cols = nodes_temp.iter ().map (Vec::len).max ().unwrap ();
			if num_cols < 1 { return Err ("Must have at least one col".into ()) }
			if num_cols & 1 == 0 { return Err ("Must have an odd number of cols".into ()) }
			let nodes_vec = nodes_temp.iter ()
				.flat_map (|row| row.iter ().copied ()
					.chain (iter::repeat (Node::Clean))
					.take (num_cols))
				.collect ();
			let grid_start = Pos::new (- (num_rows / 2).pan_i16 (), - (num_cols / 2).pan_i16 ());
			let grid_end = Pos::new ((num_rows / 2 + 1).pan_i16 (), (num_cols / 2 + 1).pan_i16 ());
			let nodes = Grid::wrap_range (nodes_vec, grid_start, grid_end) ?;
			Ok (Self { nodes, params })
		})
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}", self.nodes.print (Node::as_str)) ?;
		Ok (())
	}
}
