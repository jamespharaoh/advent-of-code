//! Advent of Code 2016: Day 24: Air Duct Spelunking
//!
//! [https://adventofcode.com/2016/day/24](https://adventofcode.com/2016/day/24)
//!
//! # Input
//!
//! Map with `#` for wall tiles, `.` for open tiles, and digits `0` to `9` for points of interest.
//! Starting point is always `0`.
//!
//! # Part one
//!
//! Work out the shortest route starting at `0` and passing through all points of interest.
//!
//! # Part two
//!
//! Same as part one, while also returning to `0` afterwards.
//!
//! # Algorithm
//!
//! This works in two steps:
//!
//! - Work out the shortest route from every point of interest to every other. This uses a simple
//!   breadth first path-finding algorithm, starting from each of the points and recording the
//!   shortest distance to each of the others.
//!
//! - Use [`search::PrioritySearch`] to find the shortest path connecting them. As an optimisation,
//!   we sort the routes used as search nodes, except for the last item, since we don't care about
//!   the order, only the distance and the point we are setting off from for the next leg.

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
		let (nums, dists) = calc_distances (input) ?;
		calc_shortest (& nums, & dists, round_trip)
	}

	fn calc_shortest (
		nums: & [(u8, Pos)],
		dists: & [(u8, u8, u32)],
		round_trip: bool,
	) -> GenResult <u32> {

		let mut search = PrioritySearch::with_hash_map (
			|mut route: ArrayVec <u8, 11>, dist, mut adder: PrioritySearchAdder <_, _, _>| {
				let here = route.last ().copied ().unwrap ();
				route.sort ();
				for & (from, to, next_dist) in dists.iter () {
					if from != here { continue }
					if route.len () == nums.len () && to != 0 { continue }
					if route.len () < nums.len () && route.contains (& to) { continue }
					if route.len () < nums.len () + usize::from (round_trip) {
						let mut new_route = route.clone ();
						new_route.push (to);
						adder.add (new_route.clone (), dist + next_dist);
					}
				}
				(route, dist)
			});

		search
			.push (array_vec! [ 0_u8 ], 0)
			.filter (|& (ref route, _)| route.len () ==
				if round_trip { nums.len () + 1 } else { nums.len () })
			.map (|(_, dist)| dist)
			.next ()
			.ok_or_else (|| "No solution found".into ())

	}

	fn calc_distances (
		input: & Input,
	) -> GenResult <(Vec <(u8, Pos)>, Vec <(u8, u8, u32)>)> {

		// find list of numbers in grid

		let nums: Vec <(u8, Pos)> =
			input.tiles.iter ()
				.filter_map (|(pos, tile)| match tile {
					Tile::Num (num) => Some ((num, pos)),
					Tile::Wall | Tile::Open => None,
				})
				.sorted ()
				.collect ();

		// check for duplicates

		if nums.iter ().map (|& (num, _)| num).sorted ().dedup ().count () != nums.len () {
			return Err ("Duplicated nums".into ());
		}

		// check start exists

		if ! nums.iter_vals ().any (|(num, _)| num == 0_u8) {
			return Err ("No starting position".into ());
		}

		// assume we already visited all walls, reusing this makes things quicker

		let seen_template = grid::Grid::<Vec <bool>, Pos>::wrap (
			input.tiles.values ().map (|tile| matches! (tile, Tile::Wall)).collect (),
			input.tiles.native_origin (),
			input.tiles.native_size ());

		// work out distances

		let mut dists: Vec <(u8, u8, u32)> = Vec::new ();
		'OUTER: for (start_idx, & (start_num, start_pos)) in nums.iter ().enumerate () {

			// track places visited so far

			let mut seen = seen_template.clone ();
			seen.set (start_pos, true);

			// track next places to visit

			let mut todo: VecDeque <(u32, Pos)> = VecDeque::new ();
			todo.push_back ((0, start_pos));

			// work out number of distance to find, allows some short-circuiting

			let mut num_to_find = nums.len () - 1 - start_idx;
			if num_to_find == 0 { continue }

			// iterate 'todo' places

			while let Some ((dist, pos)) = todo.pop_front () {

				// iterate adjacent positions

				for adj_pos in pos.adjacent_4 () {

					// track seen tiles and short-cirtcuit

					if seen.get (adj_pos).unwrap_or (true) { continue }
					seen.set (adj_pos, true);

					// add adjacent position to `todo`

					let adj_tile = input.tiles.get (adj_pos).unwrap ();
					todo.push_back ((dist + 1, adj_pos));

					// check if we reached a point-of-interest

					if let Tile::Num (num) = adj_tile {

						// only record it if it is greater than us, to prevent counting twice and
						// allow short-circuiting

						if num < start_num { continue }

						// record distance, both ways

						dists.push ((start_num, num, dist + 1));
						dists.push ((num, start_num, dist + 1));

						// abort path-finding if we found all the points greater than start

						num_to_find -= 1;
						if num_to_find == 0 { continue 'OUTER }

					}

				}

			}

			// due to short-circuiting, this loop should never complete if all the points are
			// connected

			return Err ("No solution found1".into ());

		}

		// return

		Ok ((nums, dists))

	}

}

pub mod model {

	use super::*;

	pub type Coord = u16;
	pub type TilesGrid = grid::Grid <Vec <Tile>, Pos>;
	pub type Pos = pos::PosRowCol <u16>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub tiles: TilesGrid,
		pub width: Coord,
		pub height: Coord,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Tile { Wall, Open, Num (u8) }

	impl Input {

		pub fn parse (input: & [& str]) -> GenResult <Self> {

			// work out and validate width and height

			let height = Coord::try_from (input.len ()).unwrap ();
			let width = Coord::try_from (input [0].chars ().count ()).unwrap ();
			if width == 0 || height == 0
					|| input.iter ().any (|line| line.chars ().count () != width.as_usize ()) {
				return Err ("Invalid input".into ());
			}

			// parse chars into tiles and collect

			let tiles_vec: Vec <Tile> = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						let items: Vec <_> =
								iter::from_fn (|| Some (parser.item ()))
							.take (width.as_usize ())
							.collect::<ParseResult <_>> () ?;
						parser.end () ?;
						Ok (items)
					}).map_parse_err (|_, col_idx|
						format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1, line)))
				.flatten_ok ()
				.collect::<GenResult <_>> () ?;

			// wrap collected tiles into 'Grid'

			let tiles =
				TilesGrid::wrap (
					tiles_vec,
					[0, 0],
					[ height.as_usize (), width.as_usize () ]);

			// construct and return

			Ok (Self { tiles, width, height })

		}

	}

	impl <'inp> FromParser <'inp> for Tile {
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
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
