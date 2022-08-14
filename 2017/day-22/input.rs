use super::*;
use model::Grid;
use model::Node;

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
	pub fn parse (mut input: & [& str]) -> GenResult <Self> {
		let params = InputParams::parse (& mut input) ?;
		let nodes_temp = Parser::wrap_lines_auto (
			input.iter ().copied ().enumerate (),
			|parser| {
				let mut nodes = ArrayVec::<Node, 32>::new ();
				while parser.peek ().is_some () {
					if nodes.is_full () { return Err (parser.err ()) }
					nodes.push (parser.item () ?);
				}
				Ok (nodes)
			}) ?;
		let num_rows = nodes_temp.len ();
		if num_rows < 1 { return Err ("Must have at least one row".into ()) }
		if num_rows & 1 == 0 { return Err ("Must have an odd number of rows".into ()) }
		let num_cols = nodes_temp.iter ().map (ArrayVec::len).max ().unwrap ();
		if num_cols < 1 { return Err ("Must have at least one col".into ()) }
		if num_cols & 1 == 0 { return Err ("Must have an odd number of cols".into ()) }
		let nodes_vec = nodes_temp.iter ()
			.flat_map (|row| row.iter ().copied ()
				.chain (iter::repeat (Node::Clean))
				.take (num_cols))
			.collect ();
		let grid_origin = [ (num_rows / 2).as_isize (), (num_cols / 2).as_isize ()];
		let grid_size = [ num_rows, num_cols ];
		let nodes = Grid::wrap (nodes_vec, grid_origin, grid_size);
		Ok (Self { nodes, params })
	}
}

impl Display for Input {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.params, formatter) ?;
		write! (formatter, "{}", self.nodes.print (|node| node.as_str ())) ?;
		Ok (())
	}
}
