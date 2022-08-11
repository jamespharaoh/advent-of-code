//! Advent of Code 2016: Day 12: Leonardo's Monorail
//!
//! [https://adventofcode.com/2016/day/12](https://adventofcode.com/2016/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2016_cpu as cpu;

puzzle_info! {
	name = "Leonardo's Monorail";
	year = 2016;
	day = 12;
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
			instrs: Rc::new (input.instrs.clone ()),
			limit: input.ops_limit,
			.. Cpu::default ()
		};
		cpu.exec () ?;
		Ok (cpu.reg_a)
	}

	pub fn part_two (input: & Input) -> GenResult <i32> {
		let mut cpu = Cpu {
			instrs: Rc::new (input.instrs.clone ()),
			reg_c: 1,
			limit: input.ops_limit,
			.. Cpu::default ()
		};
		cpu.exec () ?;
		Ok (cpu.reg_a)
	}

}

pub mod model {

	use super::*;
	use cpu::Instr as Instr;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub instrs: Vec <Instr>,
		pub ops_limit: u32,
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let ops_limit = parser::input_param (& mut input, "OPS_LIMIT=", 100_000_000_u32) ?;
			#[ allow (clippy::redundant_closure_for_method_calls) ]
			let instrs = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						let instr: Instr = parser.item () ?;
						if ! instr.is_v1 () { return Err (parser.err ()) }
						Ok (instr)
					}).map_parse_err (|col_idx|
						format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { instrs, ops_limit })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"cpy 41 a",
		"inc a",
		"inc a",
		"dec a",
		"jnz a 2",
		"dec a",
		"dec c",
		"jnz c 3",
		"dec a",
		"dec a",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("42", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("40", puzzle.part_two (EXAMPLE));
	}

}
