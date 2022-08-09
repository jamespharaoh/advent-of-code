//! Advent of Code 2016: Day 24: Air Duct Spelunking
//!
//! [https://adventofcode.com/2016/day/24](https://adventofcode.com/2016/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid as grid;
use aoc_pos as pos;
use aoc_search as search;

puzzle_info! {
	name = "Air Duct Spelunking";
	year = 2016;
	day = 24;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Pos;
	use model::Tile;
	use search::PrioritySearch;
	use search::PrioritySearchAdder;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		calc_result (input, false)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		calc_result (input, true)
	}

	fn calc_result (input: & Input, round_trip: bool) -> GenResult <u32> {

		// find list of numbers in grid

		let nums: Vec <(u8, Pos)> =
			input.tiles.iter ()
				.filter_map (|(pos, tile)| match tile {
					Tile::Num (num) => Some ((num, pos)),
					Tile::Wall | Tile::Open => None,
				})
				.collect ();

		// check for duplicates

		if nums.iter ().map (|& (num, _)| num).sorted ().dedup ().count () != nums.len () {
			return Err ("Duplicated nums".into ());
		}

		// convert to map

		let nums: HashMap <u8, Pos> = nums.into_iter ().collect ();

		// check start exists

		if ! nums.contains_key (& 0_u8) {
			return Err ("No starting position".into ());
		}

		// work out distances

		let mut dists: HashMap <(u8, u8), u32> = HashMap::new ();
		for (& start_num, & start_pos) in nums.iter () {
			let mut seen: HashSet <Pos> = HashSet::new ();
			seen.insert (start_pos);
			let mut todo: VecDeque <(u32, Pos)> = VecDeque::new ();
			todo.push_back ((0, start_pos));
			while let Some ((dist, pos)) = todo.pop_front () {
				for adj_pos in pos.adjacent_4 () {
					let adj_tile = some_or! (input.tiles.get (adj_pos), continue);
					if adj_tile == Tile::Wall { continue }
					if ! seen.insert (adj_pos) { continue }
					todo.push_back ((dist + 1, adj_pos));
					if let Tile::Num (num) = adj_tile {
						dists.insert ((start_num, num), dist + 1);
					}
				}
			}
		}

		// check everything is connected

		if dists.len () != nums.len () * (nums.len () - 1) {
			return Err ("No solution found1".into ());
		}

		// setup priority search

		let mut search = PrioritySearch::with_hash_map (
			|route: Vec <u8>, dist, mut adder: PrioritySearchAdder <_, _, _>| {
				for (& (from, to), & next_dist) in dists.iter () {
					if route.last () != Some (& from) { continue }
					if route.len () < nums.len () && route.contains (& to) { continue }
					if route.len () == nums.len () && to != 0 { continue }
					if route.len () > nums.len () { continue }
					let mut new_route = route.clone ();
					new_route.push (to);
					adder.add (new_route.clone (), dist + next_dist);
				}
				(route, dist)
			});

		// find best route

		search
			.push (vec! [ 0_u8 ], 0)
			.filter (|& (ref route, _)| route.len () ==
				if round_trip { nums.len () + 1 } else { nums.len () })
			.map (|(_, dist)| dist)
			.next ()
			.ok_or_else (|| "No solution found".into ())

	}

}

pub mod model {

	use super::*;
	use parser::*;

	pub type Coord = u16;
	pub type Grid = grid::Grid <Vec <Tile>, Pos>;
	pub type Pos = pos::PosRowCol <u16>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub tiles: Grid,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Tile { Wall, Open, Num (u8) }

	impl Input {

		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let height = input.len ();
			let width = input [0].chars ().count ();
			if width == 0 || height == 0
					|| input.iter ().any (|line| line.chars ().count () != width) {
				return Err ("Invalid input".into ());
			}
			let tiles_vec: Vec <Tile> = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						let items: Vec <_> =
								iter::from_fn (|| Some (parser.item ()))
							.take (width)
							.collect::<ParseResult <_>> () ?;
						parser.end () ?;
						Ok (items)
					}).map_parse_err (|col_idx|
						format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1, line)))
				.flatten_ok ()
				.collect::<GenResult <_>> () ?;
			let tiles = Grid::wrap (tiles_vec, [0, 0], [ height, width ]);
			Ok (Self { tiles })
		}

	}

	impl FromParser for Tile {
		fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
			Ok (match parser.expect_next () ? {
				'#' => Self::Wall,
				'.' => Self::Open,
				ch if ('0' ..= '9').contains (& ch) =>
					Self::Num (ch.to_digit (10).ok_or_else (|| parser.err ()) ?.as_u8 ()),
				_ => return Err (parser.err ()),
			})
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"###########",
		"#0.1.....2#",
		"#.#######.#",
		"#4.......3#",
		"###########",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("14", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("20", puzzle.part_two (EXAMPLE));
	}

}
