//! Advent of Code 2016: Day 25: Clock Signal
//!
//! [https://adventofcode.com/2016/day/24](https://adventofcode.com/2016/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_2016_cpu as cpu;

puzzle_info! {
	name = "Clock Signal";
	year = 2016;
	day = 25;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
}

pub mod logic {

	use super::*;
	use cpu::Cpu;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <i32> {
		for start in 1_i32 ..= input.limit {
			let mut cpu = Cpu {
				instrs: Rc::new (input.instrs.clone ()),
				reg_a: start,
				limit: 900,
				.. default ()
			};
			let mut expect = 0_i32;
			let mut num = 0_u32;
			let mut states = HashSet::new ();
			while let Some (out) = cpu.exec () ? {
				if out != expect { break }
				expect = if expect == 0_i32 { 1_i32 } else { 0_i32 };
				num += 1;
				if num >= 2 && ! states.insert ((expect, cpu.next, cpu.reg_a, cpu.reg_b, cpu.reg_c, cpu.reg_d)) {
					return Ok (start);
				}
			}
		}
		Err ("No solution found".into ())
	}

}

pub mod model {

	use super::*;
	use cpu::Instr;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub instrs: Vec <Instr>,
		pub limit: i32,
	}

	impl Input {

		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let limit = parser::input_param (& mut input, "LIMIT=", i32::MAX) ?;
			let instrs: Vec <Instr> = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						let instr: Instr = parser.item () ?;
						if ! instr.is_v3 () { return Err (parser.err ()) }
						Ok (instr)
					}).map_parse_err (|col_idx|
						format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { instrs, limit })
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"cpy 10 b",
		"dec a",
		"dec b",
		"jnz b -2",
		"jnz a 4",
		"out 0",
		"out 1",
		"jnz 1 -2",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("10", puzzle.part_one (EXAMPLE));
	}

}
