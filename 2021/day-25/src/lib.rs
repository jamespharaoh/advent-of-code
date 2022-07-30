//! Advent of Code 2021: Day 25: Sea Cucumber
//!
//! [https://adventofcode.com/2021/day/25](https://adventofcode.com/2021/day/25)

use aoc_common::*;

puzzle_info! {
	name = "Sea Cucumber";
	year = 2021;
	day = 25;
	part_one = |lines| logic::part_one (lines);
}

mod logic {

	use super::*;
	use model::Pos;
	use model::Region;
	use model::Seafloor;
	use nums::IntConv;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		let seafloor = Seafloor::parse (lines) ?;
		let mut count = 0;
		let mut prev_seafloor = seafloor;
		loop {
			let next_seafloor = move_both (& prev_seafloor);
			count += 1;
			if prev_seafloor == next_seafloor { break }
			prev_seafloor = next_seafloor;
		}
		Ok (count)
	}

	pub fn move_both (seafloor: & Seafloor) -> Seafloor {
		let size = seafloor.size ();
		let iter_row = |y|
			iter::once (seafloor.get (Pos { y, x: size.x - 1 }))
				.chain (seafloor.values ()
					.skip (size.x.as_usize () * y.as_usize ())
					.take (size.x.as_usize ()))
				.chain (iter::once (seafloor.get (Pos { y, x: 0 })))
				.collect::<Vec <Region>> ();
		let data =
			iter::once (iter_row (size.y - 1))
				.chain ((0 .. size.y).map (iter_row))
				.chain (iter::once (iter_row (0)))
				.scan ((Rc::new (Vec::new ()), Rc::new (Vec::new ())),
					move |rows, row| {
						let row_0 = Rc::clone (& rows.0);
						let row_1 = Rc::clone (& rows.1);
						let row_2 = Rc::new (row);
						* rows = (Rc::clone (& row_1), Rc::clone (& row_2));
						if row_0.len () == 0 || row_1.len () == 0 {
							return Some (Either::Left (iter::empty ()));
						}
						Some (Either::Right (
							(0 .. size.x.as_usize ()).map (move |idx|
								calc_one_region (
									row_0 [idx .. idx + 3].try_into ().unwrap (),
									row_1 [idx .. idx + 3].try_into ().unwrap (),
									row_2 [idx .. idx + 3].try_into ().unwrap (),
								)
							)
						))
					})
				.flatten ()
				.collect::<Vec <_>> ();
		Seafloor::new (size, data)
	}

	pub fn calc_one_region (
		above: [Region; 3],
		level: [Region; 3],
		below: [Region; 3],
	) -> Region {
		use Region::{ Empty as X, East as E, South as S };
		match (above, level, below) {
			([_, _, _], [E, X, _], [_, _, _]) => E,
			([_, S, _], [_, E, X], [_, _, _]) => S,
			([_, _, _], [_, E, X], [_, _, _]) => X,
			([_, S, _], [_, X, _], [_, _, _]) => S,
			([_, _, _], [_, S, _], [_, E, X]) => X,
			([_, _, _], [_, S, _], [E, X, _]) => S,
			([_, _, _], [_, S, _], [_, X, _]) => X,
			([_, _, _], [_, h, _], [_, _, _]) => h,
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		fn test_sequence <StepFn: Fn (& Seafloor) -> Seafloor> (
			step_fn: StepFn,
			seafloor_strs: & [& [& str]],
		) -> GenResult <()> {
			let mut seafloor = Seafloor::parse (seafloor_strs [0]) ?;
			let count = |seafloor: & Seafloor, region|
				seafloor.iter ()
					.filter (|& (_, other_region)| region == other_region)
					.count ();
			let num_east = count (& seafloor, Region::East);
			let num_south = count (& seafloor, Region::South);
			for expect_str in seafloor_strs.iter ().skip (1) {
				let expect = Seafloor::parse (expect_str) ?;
				seafloor = step_fn (& seafloor);
				assert_eq! (expect, seafloor);
				assert_eq! (num_east, count (& seafloor, Region::East));
				assert_eq! (num_south, count (& seafloor, Region::South));
			}
			Ok (())
		}

		#[ test ]
		fn test_complex () -> GenResult <()> {
			test_sequence (move_both, & [
				& [ "v...>>.vv>", ".vv>>.vv..", ">>.>v>...v", ">>v>>.>.v.", "v>v.vv.v..",
					">.>>..v...", ".vv..>.>v.", "v.v..>>v.v", "....v..v.>" ],
				& [ "....>.>v.>", "v.v>.>v.v.", ">v>>..>v..", ">>v>v>.>.v", ".>v.v...v.",
					"v>>.>vvv..", "..v...>>..", "vv...>>vv.", ">.v.v..v.v" ],
			])
		}

		#[ test ]
		fn test_east () -> GenResult <()> {
			test_sequence (move_both, & [
				& ["...>>>>>..."], & ["...>>>>.>.."], & ["...>>>.>.>."], & ["...>>.>.>.>"],
			])
		}

		#[ test ]
		fn test_both () -> GenResult <()> {
			test_sequence (move_both, & [
				& ["..........", ".>v....v..", ".......>..", ".........."],
				& ["..........", ".>........", "..v....v>.", ".........."],
				& ["..........", "..>.......", ".........>", "..v....v.."],
			])
		}

	}

}

mod model {

	use super::*;
	use bitvec::BitVecNative;
	use grid::Grid;
	use nums::IntConv;
	use pos::PosYX;

	pub type Pos = PosYX <Coord>;
	pub type Coord = u16;
	pub type GridInner = Vec <Region>;

	#[ derive (Clone, Eq, PartialEq) ]
	pub struct Seafloor {
		grid: Grid <GridInner, Pos>,
		size: Pos,
	}

	impl Seafloor {
		pub fn new (size: Pos, regions: GridInner) -> Seafloor {
			let grid_size = [size.y.as_usize (), size.x.as_usize ()];
			let grid = Grid::wrap (regions, [0, 0], grid_size);
			Seafloor { grid, size }
		}
		pub fn size (& self) -> Pos { self.size }
		pub fn parse (lines: & [& str]) -> GenResult <Seafloor> {
			if lines.is_empty () { Err (format! ("Invalid input")) ? }
			let size = Pos { y: lines.len ().as_u16 (), x: lines [0].chars ().count ().as_u16 () };
			let data = lines.iter ().enumerate ()
				.flat_map (|(line_idx, line)| {
					let line_err = move || format! ("Invalid input: {}: {}", line_idx, line);
					line.chars ().map::<Result <_, String>, _> (move |letter|
						Ok (match letter {
							'.' => Region::Empty,
							'>' => Region::East,
							'v' => Region::South,
							_ => Err (line_err ()) ?,
						}),
					)
				}).collect::<Result <_, _>> () ?;
			let grid_size = [size.y.as_usize (), size.x.as_usize ()];
			let grid = Grid::wrap (data, [0, 0], grid_size);
			Ok (Seafloor { grid, size })
		}
		pub fn get (& self, pos: Pos) -> Region {
			self.grid.get (pos).unwrap ()
		}
		pub fn iter (& self) -> impl Iterator <Item = (Pos, Region)> + '_ {
			self.grid.iter ()
		}
		pub fn values (& self) -> impl Iterator <Item = Region> + '_ {
			self.grid.values ()
		}
	}

	impl fmt::Debug for Seafloor {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "Seafloor {{ {:?}, ", self.size) ?;
			for (pos, region) in self.iter () {
				if pos.x == 0 && pos.y > 0 { write! (formatter, ", ") ?; }
				if pos.x == 0 { write! (formatter, "\"") ?; }
				write! (formatter, "{}", region) ?;
				if pos.x + 1 == self.size.x { write! (formatter, "\"") ?; }
			}
			write! (formatter, " }}") ?;
			Ok (())
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Region {
		Empty,
		East,
		South,
	}

	impl BitVecNative for Region {
		const BITS: u32 = 2;
		fn encode (self) -> usize {
			match self {
				Region::Empty => 0x0,
				Region::East => 0x1,
				Region::South => 0x2,
			}
		}
		fn decode (encoded: usize) -> Region {
			match encoded {
				0 => Region::Empty,
				1 => Region::East,
				2 => Region::South,
				_ => panic! ("Invalid encoded value for Region: {:#x}", encoded),
			}
		}
	}

	impl Display for Region {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{}", match * self {
				Region::Empty => '.',
				Region::East => '>',
				Region::South => 'v',
			})
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_0: & [& str] = & [
		"v...>>.vv>", ".vv>>.vv..", ">>.>v>...v", ">>v>>.>.v.", "v>v.vv.v..", ">.>>..v...",
		".vv..>.>v.", "v.v..>>v.v", "....v..v.>",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (58, logic::part_one (EXAMPLE_0) ?);
		Ok (())
	}

}
