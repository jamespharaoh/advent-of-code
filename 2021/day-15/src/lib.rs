//! Advent of Code 2021: Day 15: Chiton
//!
//! [https://adventofcode.com/2021/day/15](https://adventofcode.com/2021/day/15)
//!
//! This is a relatively simple path finder. We use [`PrioritySearch`](search::PrioritySearch) to
//! prioritise the routes with the lowest risk, and keep a cache of the best ones so far to each
//! point so we can short-circuit appropriately.

use aoc_common::*;
use aoc_grid as grid;
use aoc_pos as pos;
use aoc_search as search;

puzzle_info! {
	name = "Chiton";
	year = 2021;
	day = 15;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Cave;
	use model::Grid;
	use model::Pos;
	use nums::IntConv;
	use search::PrioritySearch;
	use search::PrioritySearchAdder;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let cave = Cave::parse (lines) ?;
		Ok (calc_result (& cave))
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let mut cave = Cave::parse (lines) ?;
		let risks = {
			let cave_risks = & cave.risks;
			(0 .. 5).flat_map (move |rep_y| (0 ..= cave.end.y).flat_map (move |y|
				(0 .. 5).flat_map (move |rep_x| (0 ..= cave.end.x).map (move |x| {
					let orig_risk = cave_risks.get (Pos { y, x }).unwrap ();
					(orig_risk + rep_y + rep_x - 1) % 9 + 1
				})),
			)).collect ()
		};
		let new_size = [cave.risks.native_size () [0] * 5, cave.risks.native_size () [1] * 5];
		cave.risks = Grid::wrap (risks, [0, 0], new_size);
		cave.end = Pos { y: (cave.end.y + 1) * 5 - 1, x: (cave.end.x + 1) * 5 - 1 };
		Ok (calc_result (& cave))
	}

	pub fn calc_result (cave: & Cave) -> u64 {
		let mut search = PrioritySearch::with_grid (
			[0, 0],
			cave.risks.native_size (),
			|pos: Pos, path_risk, mut adder: PrioritySearchAdder <Pos, u64, _>| {
				for adj_pos in pos.adjacent_4 () {
					if let Some (adj_risk) = cave.risks.get (adj_pos) {
						let adj_path_risk = path_risk + adj_risk.as_u64 ();
						adder.add (adj_pos, adj_path_risk);
					}
				}
				(pos, path_risk)
			},
		);
		search.push (cave.start, 0);
		search
			.filter (|& (pos, _)| pos == cave.end)
			.map (|(_, score)| score)
			.next ()
			.unwrap ()
	}

}

mod model {

	use super::*;
	use nums::IntConv;

	pub type Grid <Val> = grid::Grid <Vec <Val>, Pos, 2>;
	pub type Pos = pos::PosYX <i16>;

	pub struct Cave {
		pub risks: Grid <u8>,
		pub start: Pos,
		pub end: Pos,
	}

	impl Cave {
		pub fn parse (lines: & [& str]) -> GenResult <Self> {
			let mut risks = Vec::new ();
			for (line_idx, line) in lines.iter ().enumerate () {
				let line_err = || format! ("Invalid input on line {}: {}", line_idx + 1, line);
				for letter in line.chars () {
					risks.push (letter.to_digit (10).ok_or_else (line_err) ?.as_u8 ());
				}
			}
			let risks = Grid::wrap (risks, [0, 0], [lines.len (), lines [0].len ()]);
			let start = Pos { x: 0, y: 0 };
			let end = Pos {
				x: risks.native_size () [1].as_i16 () - 1,
				y: risks.native_size () [0].as_i16 () - 1,
			};
			Ok (Self { risks, start, end })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"1163751742", "1381373672", "2136511328", "3694931569", "7463417111",
		"1319128137", "1359912421", "3125421639", "1293138521", "2311944581",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (40, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (315, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
