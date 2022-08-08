//! Advent of Code 2016: Day 22: Grid Computing
//!
//! [https://adventofcode.com/2016/day/22](https://adventofcode.com/2016/day/22)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Grid Computing";
	year = 2016;
	day = 22;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Pos;

	pub fn part_one (input: & Input) -> GenResult <usize> {
		Ok (
			input.nodes.iter ()
				.filter (|node_a| node_a.used > 0)
				.map (|node_a| input.nodes.iter ()
					.filter (|node_b| node_a.pos != node_b.pos)
					.filter (|node_b| node_a.used <= node_b.avail)
					.count ())
				.sum ()
		)
	}

	pub fn part_two (input: & Input) -> GenResult <usize> {

		#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		struct State { data: Pos, empty: Pos }

		type Walls = grid::Grid <Vec <bool>, Pos>;

		let empty_start =
			input.nodes.iter ()
				.filter (|node| node.used == 0)
				.map (|node| node.pos)
				.exactly_one ()
				.map_err (|_err| "Must have exactly one empty node") ?;

		let lowest = input.nodes.iter ().filter (|node| node.used > 0).map (|node| node.used).min ().unwrap ();

		let grid_size = [input.width.as_usize (), input.height.as_usize ()];
		let mut walls = Walls::new_vec ([0, 0], grid_size);
		for node in input.nodes.iter () {
			walls.set (node.pos, node.used > lowest * 2);
		}

		let state = State { data: input.start, empty: empty_start };

		let mut seen: HashSet <State> = HashSet::new ();
		seen.insert (state);

		let mut todo: VecDeque <(usize, State)> = VecDeque::new ();
		todo.push_back ((0, state));

		while let Some ((dist, state)) = todo.pop_front () {
			if state.data == input.end { return Ok (dist) }
			for from_pos in state.empty.adjacent_4 () {
				if walls.get (from_pos).unwrap_or (true) { continue }
				let new_state = State {
					data: if from_pos == state.data { state.empty } else { state.data },
					empty: from_pos,
				};
				if seen.insert (new_state) {
					todo.push_back ((dist + 1, new_state));
				}
			}
		}

		Err ("No solution found".into ())

	}

}

pub mod model {

	use super::*;
	use parser::*;

	pub type Coord = u8;
	pub type Size = u16;
	pub type Pos = pos::PosXY <Coord>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub nodes: Vec <Node>,
		pub width: Coord,
		pub height: Coord,
		pub start: Pos,
		pub end: Pos,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub struct Node {
		pub pos: Pos,
		pub size: Size,
		pub used: Size,
		pub avail: Size,
	}

	impl Input {

		pub fn parse (input: & [& str]) -> GenResult <Self> {
			if input.len () < 2
					|| input [0] != "root@ebhq-gridcenter# df -h"
					|| input [1] != "Filesystem              Size  Used  Avail  Use%" {
				return Err ("Invalid input".into ());
			}
			let nodes: Vec <Node> = input.iter ()
				.enumerate ()
				.skip (2)
				.map (|(line_idx, line)| {
					#[ allow (clippy::redundant_closure_for_method_calls) ]
					Parser::wrap (line, |parser| parser.item ())
						.map_parse_err (|col_idx|
							format! ("Invalid input: line {}: col {}: {}",
								line_idx + 1, col_idx + 1, line))
				})
				.collect::<GenResult <_>> () ?;
			if nodes.is_empty () { return Err ("Must have at least one node".into ()) }
			let width = nodes.iter ().map (|& node| node.pos.x + 1).max ().unwrap ();
			let height = nodes.iter ().map (|& node| node.pos.y + 1).max ().unwrap ();
			let start = Pos { x: width - 1, y: 0 };
			let end = Pos { x: 0, y: 0 };
			Ok (Self { nodes, width, height, start, end })
		}

	}

	impl FromParser for Node {
		fn from_parser (parser: & mut Parser) -> ParseResult <Self> {

			let x = parser.expect ("/dev/grid/node-x") ?.int () ?;
			if x == Coord::MAX { return Err (parser.err ()) }

			let y = parser.expect ("-y") ?.int () ?;
			if y == Coord::MAX { return Err (parser.err ()) }

			let pos = Pos { x, y };

			let size: Size = parser.skip_whitespace ().int () ?;

			let used = parser.expect ("T") ?.skip_whitespace ().int () ?;
			if used > size { return Err (parser.err ()) }

			let avail = parser.expect ("T") ?.skip_whitespace ().int () ?;
			if avail > size { return Err (parser.err ()) }
			if Size::add_2 (used, avail) ? != size { return Err (parser.err ()) }

			let use_pc: u8 = parser.expect ("T") ?.skip_whitespace ().int () ?;
			let expect_pc = Size::div_2 (Size::mul_2 (used, 100) ?, size) ?;
			if use_pc.as_u16 () != expect_pc { return Err (parser.err ()) }

			parser.expect ("%") ?.end () ?;

			Ok (Self { pos, size, used, avail })

		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"root@ebhq-gridcenter# df -h",
		"Filesystem              Size  Used  Avail  Use%",
		"/dev/grid/node-x0-y0     93T   68T    25T   73%",
		"/dev/grid/node-x0-y1     91T   69T    22T   75%",
		"/dev/grid/node-x0-y2     92T   68T    24T   73%",
		"/dev/grid/node-x0-y3     92T   73T    19T   79%",
		"/dev/grid/node-x0-y4     89T   69T    20T   77%",
		"/dev/grid/node-x1-y0     89T   65T    24T   73%",
		"/dev/grid/node-x1-y1     89T   71T    18T   79%",
		"/dev/grid/node-x1-y2     88T   73T    15T   82%",
		"/dev/grid/node-x1-y3     93T   68T    25T   73%",
		"/dev/grid/node-x1-y4     91T   68T    23T   74%",
		"/dev/grid/node-x2-y0     94T   73T    21T   77%",
		"/dev/grid/node-x2-y1     93T   67T    26T   72%",
		"/dev/grid/node-x2-y2     87T   69T    18T   79%",
		"/dev/grid/node-x2-y3     86T   66T    20T   76%",
		"/dev/grid/node-x2-y4     94T   68T    26T   72%",
		"/dev/grid/node-x3-y0     94T   65T    29T   69%",
		"/dev/grid/node-x3-y1     87T   71T    16T   81%",
		"/dev/grid/node-x3-y2     91T   0T     91T   0%",
		"/dev/grid/node-x3-y3     94T   69T    25T   73%",
		"/dev/grid/node-x3-y4     90T   73T    17T   81%",
		"/dev/grid/node-x4-y0     86T   69T    17T   80%",
		"/dev/grid/node-x4-y1     92T   72T    20T   78%",
		"/dev/grid/node-x4-y2     94T   64T    30T   68%",
		"/dev/grid/node-x4-y3     85T   64T    21T   75%",
		"/dev/grid/node-x4-y4     92T   68T    24T   73%",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("24", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("18", puzzle.part_two (EXAMPLE));
	}

}
