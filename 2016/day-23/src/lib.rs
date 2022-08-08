//! Advent of Code 2016: Day 23: Safe Cracking
//!
//! [https://adventofcode.com/2016/day/23](https://adventofcode.com/2016/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2016_cpu as cpu;

puzzle_info! {
	name = "Safe Cracking";
	year = 2016;
	day = 23;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use cpu::Cpu;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <i32> {
		let mut cpu = Cpu {
			instrs: input.instrs.clone (),
			reg_a: 7,
			limit: 1000,
			.. default ()
		};
		cpu.exec () ?;
		Ok (cpu.reg_a)
	}

	pub fn part_two (input: & Input) -> GenResult <i32> {
		let mut cpu = Cpu {
			instrs: input.instrs.clone (),
			reg_a: 12,
			limit: 1000,
			.. default ()
		};
		cpu.exec () ?;
		Ok (cpu.reg_a)
	}

}

pub mod model {

	use super::*;
	use cpu::Instr;
	use parser::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub instrs: Vec <Instr>,
	}

	impl Input {

		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let instrs: Vec <Instr> = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						let instr: Instr = parser.item () ?;
						if ! instr.is_v2 () { return Err (parser.err ()) }
						Ok (instr)
					}).map_parse_err (|col_idx|
						format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { instrs })
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"cpy 2 a",
		"tgl a",
		"tgl a",
		"tgl a",
		"cpy 1 a",
		"dec a",
		"dec a",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_one (EXAMPLE));
	}

}
