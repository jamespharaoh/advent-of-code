//! Advent of Code 2021: Day 13: Transparent Origami
//!
//! [https://adventofcode.com/2021/day/13](https://adventofcode.com/2021/day/13)

use aoc_common::*;

puzzle_info! {
	name = "Transparent Origami";
	year = 2021;
	day = 13;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
	commands = [
		( name = "run"; method = tool::run; ),
	];
}

mod tool {

	use super::*;
	use model::Input;

	#[ derive (clap::Parser) ]
	pub struct RunArgs {

		#[ clap (long, value_parser, default_value = "inputs/day-13") ]
		input: String,

	}

	pub fn run (args: RunArgs) -> GenResult <()> {
		let input_string = fs::read_to_string (args.input) ?;
		let input_lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
		let input = Input::parse (& input_lines) ?;
		let mut dots = input.dots;
		for fold in input.folds.iter () {
			dots = logic::fold_dots (fold, & dots);
		}
		print! ("{}", logic::DrawDots (& dots));
		Ok (())
	}

}

mod logic {

	use super::*;
	use model::Axis;
	use model::Fold;
	use model::Input;
	use model::Pos;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let input = Input::parse (lines) ?;
		let dots = fold_dots (& input.folds [0], & input.dots);
		Ok (dots.len () as u64)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <String> {
		let input = Input::parse (lines) ?;
		let mut dots = input.dots;
		for fold in input.folds.iter () {
			dots = fold_dots (fold, & dots);
		}
		Ok (read_dots (& dots) ?)
	}

	pub fn fold_dots (fold: & Fold, dots: & HashSet <Pos>) -> HashSet <Pos> {
		let mut new_dots = HashSet::new ();
		for mut dot in dots.iter ().cloned () {
			match fold.axis {
				Axis::X => if dot.x > fold.val { dot.x = fold.val - (dot.x - fold.val) },
				Axis::Y => if dot.y > fold.val { dot.y = fold.val - (dot.y - fold.val) },
			}
			new_dots.insert (dot);
		}
		new_dots
	}

	pub fn read_dots (dots: & HashSet <Pos>) -> GenResult <String> {
		let mut result = String::new ();
		for offset in (0 .. ).step_by (5) {
			let mut encoded: u32 = 0;
			for row in 0 .. 6 {
				for col in 0 .. 5 {
					encoded <<= 1;
					if dots.contains (& Pos { x: offset + col, y: row }) {
						encoded |= 1;
					}
				}
			}
			result.push (match encoded {
				0x19297a52 => 'A',
				0x392e4a5c => 'B',
				0x1928424c => 'C',
				0x3d0e421e => 'E',
				0x19285a4e => 'G',
				0x3d0e4210 => 'F',
				0x0c210a4c => 'J',
				0x252f4a52 => 'H',
				0x254c5292 => 'K',
				0x2108421e => 'L',
				0x39297210 => 'P',
				0x39297292 => 'R',
				0x25294a4c => 'U',
				0x3c22221e => 'Z',
				0x00000000 => break,
				_ => Err (format! ("Unrecognised character: {:#08x} in position {}", encoded,
					result.len () + 1)) ?,
			});
		}
		Ok (result)
	}

	pub struct DrawDots <'a> (pub & 'a HashSet <Pos>);

	impl <'a> fmt::Display for DrawDots <'a> {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			let dots = {
				let mut dots_temp: Vec <Pos> = self.0.iter ().cloned ().collect ();
				dots_temp.sort_by_key (|pos| (pos.y, pos.x));
				dots_temp
			};
			let mut dots_iter = dots.into_iter ();
			let mut row: i64 = -1;
			let mut col: i64 = -1;
			while let Some (dot) = dots_iter.next () {
				while row < dot.y {
					write! (formatter, "\n") ?;
					col = -1;
					row += 1;
				}
				while col < dot.x {
					write! (formatter, "  ") ?;
					col += 1;
				}
				write! (formatter, "##") ?;
				col += 1;
			}
			write! (formatter, "\n\n") ?;
			Ok (())
		}
	}

}

mod model {

	use super::*;

	#[ derive (Debug) ]
	pub struct Input {
		pub dots: HashSet <Pos>,
		pub folds: Vec <Fold>,
	}

	impl Input {
		pub fn parse (lines: & [& str]) -> GenResult <Input> {
			let mut lines_iter = lines.iter ();
			let mut dots = HashSet::new ();
			while let Some (line) = lines_iter.next () {
				if line.is_empty () { break }
				let line_err = || format! ("Invalid input: {}", line);
				let line_parts: Vec <& str> = line.split (",").collect ();
				if line_parts.len () != 2 { Err (line_err ()) ? }
				dots.insert (Pos {
					x: line_parts [0].parse ().map_err (|_| line_err ()) ?,
					y: line_parts [1].parse ().map_err (|_| line_err ()) ?,
				});
			}
			let mut folds = Vec::new ();
			while let Some (line) = lines_iter.next () {
				let line_err = || format! ("Invalid input: {}", line);
				if line.starts_with ("fold along x=") {
					folds.push (Fold { axis: Axis::X, val: line [13 ..].parse () ? });
				} else if line.starts_with ("fold along y=") {
					folds.push (Fold { axis: Axis::Y, val: line [13 ..].parse () ? });
				} else {
					Err (line_err ()) ?
				}
			}
			Ok (Input { dots, folds })
		}
	}

	#[ derive (Debug) ]
	pub struct Fold {
		pub axis: Axis,
		pub val: i64,
	}

	#[ derive (Clone, Copy, Debug) ]
	pub enum Axis { X, Y }

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Pos { pub x: i64, pub y: i64 }

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"6,10",
		"0,14",
		"9,10",
		"0,3",
		"10,4",
		"4,11",
		"6,0",
		"6,12",
		"4,1",
		"0,13",
		"10,12",
		"3,4",
		"3,0",
		"8,4",
		"1,10",
		"2,14",
		"8,10",
		"9,0",
		"",
		"fold along y=7",
		"fold along x=5",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (17, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

}
