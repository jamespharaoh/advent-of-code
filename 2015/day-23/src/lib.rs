//! Advent of Code 2015: Day 23: Opening the Turing Lock
//!
//! [https://adventofcode.com/2015/day/23](https://adventofcode.com/2015/day/23)

use aoc_common::*;

puzzle_info! {
	name = "Opening the Turing Lock";
	year = 2015;
	day = 23;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Input;
	use model::Instr;
	use model::Reg;
	use model::Val;
	use nums::Int;

	pub fn part_one (input: Input) -> GenResult <Val> {
		let (_, reg_b) = emulate (input, 0, 0, 0, 0x10000) ?;
		Ok (reg_b)
	}

	pub fn part_two (input: Input) -> GenResult <Val> {
		let (_, reg_b) = emulate (input, 1, 0, 0, 0x10000) ?;
		Ok (reg_b)
	}

	fn emulate (
		input: Input,
		mut reg_a: Val,
		mut reg_b: Val,
		mut next: Val,
		mut max_loops: usize,
	) -> Result <(Val, Val), EmulateError> {

		// main loop

		let mut seen = HashSet::new ();
		while next < input.instrs.len () as Val {

			// abort when we reach max_loops

			if max_loops == 0 { return Err (EmulateError::MaxLoops) }
			max_loops -= 1;

			// detect infinite loops and abort

			if ! seen.insert ((reg_a, reg_b, next)) { return Err (EmulateError::InfiniteLoop) }

			// execute next instruction

			let instr = input.instrs [next as usize];
			match instr {
				Instr::Hlf (Reg::A) => reg_a /= 2,
				Instr::Hlf (Reg::B) => reg_b /= 2,
				Instr::Tpl (Reg::A) =>
					reg_a = Val::mul_2 (reg_a, 3).map_err (|_| EmulateError::Overflow) ?,
				Instr::Tpl (Reg::B) =>
					reg_b = Val::mul_2 (reg_b, 3).map_err (|_| EmulateError::Overflow) ?,
				Instr::Inc (Reg::A) =>
					reg_a = Val::add_2 (reg_a, 1).map_err (|_| EmulateError::Overflow) ?,
				Instr::Inc (Reg::B) =>
					reg_b = Val::add_2 (reg_b, 1).map_err (|_| EmulateError::Overflow) ?,
				Instr::Jmp (offset) => {
					next = Val::add_signed (next, offset).map_err (|_| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jie (Reg::A, offset) => if reg_a & 1 == 0 {
					next = Val::add_signed (next, offset).map_err (|_| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jie (Reg::B, offset) => if reg_b & 1 == 0 {
					next = Val::add_signed (next, offset).map_err (|_| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jio (Reg::A, offset) => if reg_a == 1 {
					next = Val::add_signed (next, offset).map_err (|_| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jio (Reg::B, offset) => if reg_b == 1 {
					next = Val::add_signed (next, offset).map_err (|_| EmulateError::Overflow) ?;
					continue;
				},
			}

			// move to next instruction, jump instructions skip this bit

			next += 1;

		}

		// return the two registers

		Ok ((reg_a, reg_b))

	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	enum EmulateError {
		InfiniteLoop,
		Overflow,
		MaxLoops,
	}

	impl Error for EmulateError {
	}

	impl Display for EmulateError {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			match self {
				EmulateError::InfiniteLoop => write! (formatter, "Infinite loop") ?,
				EmulateError::Overflow => write! (formatter, "Arithmetic overflow") ?,
				EmulateError::MaxLoops => write! (formatter, "Max loops reached") ?,
			}
			Ok (())
		}
	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;
	use parser::*;

	pub type Val = u32;
	pub type Offset = i32;

	#[ derive (Clone, Debug) ]
	pub struct Input {
		pub instrs: Vec <Instr>,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Instr {
		Hlf (Reg),
		Tpl (Reg),
		Inc (Reg),
		Jmp (Offset),
		Jie (Reg, Offset),
		Jio (Reg, Offset),
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Reg { A, B }

	impl Instr {
		fn parse_real (parser: & mut Parser) -> ParseResult <Instr> {
			parser
				.set_ignore_whitespace (true)
				.set_word_pred (|ch| ch.is_ascii_alphanumeric ());
			parser.any ()
				.of (|parser| {
					parser.expect_word ("hlf") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					parser.end () ?;
					Ok (Instr::Hlf (reg))
				})
				.of (|parser| {
					parser.expect_word ("tpl") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					parser.end () ?;
					Ok (Instr::Tpl (reg))
				})
				.of (|parser| {
					parser.expect_word ("inc") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					parser.end () ?;
					Ok (Instr::Inc (reg))
				})
				.of (|parser| {
					parser.expect_word ("jmp") ?.confirm ();
					let offset = parser.int () ?;
					parser.end () ?;
					Ok (Instr::Jmp (offset))
				})
				.of (|parser| {
					parser.expect_word ("jio") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					let offset = parser.expect (",") ?.int () ?;
					parser.end () ?;
					Ok (Instr::Jio (reg, offset))
				})
				.of (|parser| {
					parser.expect_word ("jie") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					let offset = parser.expect (",") ?.int () ?;
					parser.end () ?;
					Ok (Instr::Jie (reg, offset))
				})
				.done ()
		}
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Input> {
			let instrs =
				input.iter ().enumerate ()
					.map (|(line_idx, line)|
						Parser::wrap (line, Instr::parse_real)
							.map_parse_err (|col_idx|
								format! ("Invalid input: line {}: col {}: {}",
									line_idx + 1, col_idx + 1, line)))
					.collect::<GenResult <_>> () ?;
			Ok (Input { instrs })
		}
	}

	impl Reg {
		fn parse_real (parser: & mut Parser) -> ParseResult <Reg> {
			parser.any ()
				.of (|parser| { parser.expect_word ("a") ?; Ok (Reg::A) })
				.of (|parser| { parser.expect_word ("b") ?; Ok (Reg::B) })
				.done ()
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"jio a, +5",
		"inc a",
		"tpl a",
		"tpl a",
		"jmp +3",
		"inc a",
		"tpl a",
		"jio a, +4",
		"hlf a",
		"inc b",
		"jmp -3",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2", puzzle.part_two (EXAMPLE));
	}

}
