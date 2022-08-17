//! Advent of Code 2015: Day 23: Opening the Turing Lock
//!
//! [https://adventofcode.com/2015/day/23](https://adventofcode.com/2015/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Opening the Turing Lock";
	year = 2015;
	day = 23;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
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
	use nums::IntConv;

	pub fn part_one (input: & Input) -> GenResult <Val> {
		let (_, reg_b) = emulate (input, 0, 0, 0, 0x10000) ?;
		Ok (reg_b)
	}

	pub fn part_two (input: & Input) -> GenResult <Val> {
		let (_, reg_b) = emulate (input, 1, 0, 0, 0x10000) ?;
		Ok (reg_b)
	}

	fn emulate (
		input: & Input,
		mut reg_a: Val,
		mut reg_b: Val,
		mut next: Val,
		mut max_loops: usize,
	) -> Result <(Val, Val), EmulateError> {

		// main loop

		let mut seen = HashSet::new ();
		while next < Val::from_usize (input.instrs.len ())
			.map_err (|_err| EmulateError::Overflow) ? {

			// abort when we reach max_loops

			if max_loops == 0 { return Err (EmulateError::MaxLoops) }
			max_loops -= 1;

			// detect infinite loops and abort

			if ! seen.insert ((reg_a, reg_b, next)) { return Err (EmulateError::InfiniteLoop) }

			// execute next instruction

			let instr = input.instrs [next.as_usize ()];
			match instr {
				Instr::Hlf (Reg::A) => reg_a /= 2,
				Instr::Hlf (Reg::B) => reg_b /= 2,
				Instr::Tpl (Reg::A) =>
					reg_a = Val::mul_2 (reg_a, 3)
						.map_err (|_err| EmulateError::Overflow) ?,
				Instr::Tpl (Reg::B) =>
					reg_b = Val::mul_2 (reg_b, 3)
						.map_err (|_err| EmulateError::Overflow) ?,
				Instr::Inc (Reg::A) =>
					reg_a = Val::add_2 (reg_a, 1)
						.map_err (|_err| EmulateError::Overflow) ?,
				Instr::Inc (Reg::B) =>
					reg_b = Val::add_2 (reg_b, 1)
						.map_err (|_err| EmulateError::Overflow) ?,
				Instr::Jmp (offset) => {
					next = Val::add_signed (next, offset)
						.map_err (|_err| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jie (Reg::A, offset) => if reg_a & 1 == 0 {
					next = Val::add_signed (next, offset)
						.map_err (|_err| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jie (Reg::B, offset) => if reg_b & 1 == 0 {
					next = Val::add_signed (next, offset)
						.map_err (|_err| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jio (Reg::A, offset) => if reg_a == 1 {
					next = Val::add_signed (next, offset)
						.map_err (|_err| EmulateError::Overflow) ?;
					continue;
				},
				Instr::Jio (Reg::B, offset) => if reg_b == 1 {
					next = Val::add_signed (next, offset)
						.map_err (|_err| EmulateError::Overflow) ?;
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
			match * self {
				Self::InfiniteLoop => write! (formatter, "Infinite loop") ?,
				Self::Overflow => write! (formatter, "Arithmetic overflow") ?,
				Self::MaxLoops => write! (formatter, "Max loops reached") ?,
			}
			Ok (())
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn emulate () {
			use Instr::*;
			use Reg::*;
			// inc, hlf, tpl
			assert_eq_ok! ((2, 0), logic::emulate (
				& Input { instrs: vec! [ Inc (A), Tpl (A), Inc (A), Hlf (A) ] }, 0, 0, 0, 10));
			assert_eq_ok! ((0, 2), logic::emulate (
				& Input { instrs: vec! [ Inc (B), Tpl (B), Inc (B), Hlf (B) ] }, 0, 0, 0, 10));
			// jmp
			assert_eq_ok! ((1, 1), logic::emulate (
				& Input { instrs: vec! [ Inc (A), Jmp (2), Tpl (A), Inc (B) ] }, 0, 0, 0, 10));
			// jio
			assert_eq_ok! ((1, 1), logic::emulate (
				& Input { instrs: vec! [ Inc (A), Jio (A, 2), Inc (B), Inc (B) ] }, 0, 0, 0, 10));
			assert_eq_ok! ((0, 2), logic::emulate (
				& Input { instrs: vec! [ Jio (A, 2), Inc (B), Inc (B) ] }, 0, 0, 0, 10));
			assert_eq_ok! ((1, 1), logic::emulate (
				& Input { instrs: vec! [ Inc (B), Jio (B, 2), Inc (A), Inc (A) ] }, 0, 0, 0, 10));
			assert_eq_ok! ((2, 0), logic::emulate (
				& Input { instrs: vec! [ Jio (B, 2), Inc (A), Inc (A) ] }, 0, 0, 0, 10));
			// jie
			assert_eq_ok! ((1, 2), logic::emulate (
				& Input { instrs: vec! [ Inc (A), Jie (A, 2), Inc (B), Inc (B) ] }, 0, 0, 0, 10));
			assert_eq_ok! ((0, 1), logic::emulate (
				& Input { instrs: vec! [ Jie (A, 2), Inc (B), Inc (B) ] }, 0, 0, 0, 10));
			assert_eq_ok! ((2, 1), logic::emulate (
				& Input { instrs: vec! [ Inc (B), Jie (B, 2), Inc (A), Inc (A) ] }, 0, 0, 0, 10));
			assert_eq_ok! ((1, 0), logic::emulate (
				& Input { instrs: vec! [ Jie (B, 2), Inc (A), Inc (A) ] }, 0, 0, 0, 10));
			// errors
			assert_err! ("Infinite loop", logic::emulate (
				& Input { instrs: vec! [ Jmp (0) ] }, 0, 0, 0, 10));
			assert_err! ("Max loops reached", logic::emulate (
				& Input { instrs: vec! [ Inc (A), Jmp (-1) ] }, 0, 0, 0, 10));
			assert_err! ("Arithmetic overflow", logic::emulate (
				& Input { instrs: vec! [ Inc (A), Tpl (A), Jmp (-1) ] }, 0, 0, 0, 100));
		}

	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;

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
		pub fn parse (input: & str) -> GenResult <Self> {
			Parser::wrap (input, Self::parse_real)
				.map_parse_err (|_, col_idx|
					format! ("Invalid input: col {}: {}", col_idx + 1, input))
		}
		fn parse_real (parser: & mut Parser) -> ParseResult <Self> {
			parser
				.set_ignore_whitespace (true)
				.set_word_pred (|ch| ch.is_ascii_alphanumeric ());
			parser.any ()
				.of (|parser| {
					parser.expect_word ("hlf") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					parser.end () ?;
					Ok (Self::Hlf (reg))
				})
				.of (|parser| {
					parser.expect_word ("tpl") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					parser.end () ?;
					Ok (Self::Tpl (reg))
				})
				.of (|parser| {
					parser.expect_word ("inc") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					parser.end () ?;
					Ok (Self::Inc (reg))
				})
				.of (|parser| {
					parser.expect_word ("jmp") ?.confirm ();
					let offset = parser.int () ?;
					parser.end () ?;
					Ok (Self::Jmp (offset))
				})
				.of (|parser| {
					parser.expect_word ("jio") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					let offset = parser.expect (",") ?.int () ?;
					parser.end () ?;
					Ok (Self::Jio (reg, offset))
				})
				.of (|parser| {
					parser.expect_word ("jie") ?.confirm ();
					let reg = Reg::parse_real (parser) ?;
					let offset = parser.expect (",") ?.int () ?;
					parser.end () ?;
					Ok (Self::Jie (reg, offset))
				})
				.done ()
		}
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let instrs =
				input.iter ().enumerate ()
					.map (|(line_idx, line)|
						Parser::wrap (line, Instr::parse_real)
							.map_parse_err (|_, col_idx|
								format! ("Invalid input: line {}: col {}: {}",
									line_idx + 1, col_idx + 1, line)))
					.collect::<GenResult <_>> () ?;
			Ok (Self { instrs })
		}
	}

	impl Reg {
		fn parse_real (parser: & mut Parser) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| { parser.expect_word ("a") ?; Ok (Self::A) })
				.of (|parser| { parser.expect_word ("b") ?; Ok (Self::B) })
				.done ()
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn input_parse () {
			assert_err! ("Invalid input: line 2: col 8: jio a, +-5",
				Input::parse (& [ "inc a", "jio a, +-5" ]));
		}

		#[ test ]
		fn instr_parse () {
			use { Instr::*, Reg::* };
			assert_eq_ok! (Hlf (A), Instr::parse ("hlf a"));
			assert_eq_ok! (Tpl (B), Instr::parse ("tpl b"));
			assert_eq_ok! (Inc (A), Instr::parse ("inc a"));
			assert_eq_ok! (Jmp (-1), Instr::parse ("jmp -1"));
			assert_eq_ok! (Jio (B, 0), Instr::parse ("jio b, 0"));
			assert_eq_ok! (Jie (A, 1), Instr::parse ("jie a, +1"));
			assert_err! ("Invalid input: col 5: inc abc", Instr::parse ("inc abc"));
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
