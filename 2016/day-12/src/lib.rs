//! Advent of Code 2016: Day 12: Leonardo's Monorail
//!
//! [https://adventofcode.com/2016/day/12](https://adventofcode.com/2016/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

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
	use model::Input;
	use model::Instr;
	use model::Reg;

	pub fn part_one (input: & Input) -> GenResult <i32> {
		let mut regs = Regs {
			l: input.ops_limit,
			.. Regs::default ()
		};
		regs.exec (& input.instrs) ?;
		Ok (regs.a)
	}

	pub fn part_two (input: & Input) -> GenResult <i32> {
		let mut regs = Regs {
			c: 1,
			l: input.ops_limit,
			.. Regs::default ()
		};
		regs.exec (& input.instrs) ?;
		Ok (regs.a)
	}

	#[ derive (Default) ]
	struct Regs {
		a: i32,
		b: i32,
		c: i32,
		d: i32,
		l: u32,
		p: u32,
	}

	impl Regs {

		const fn load (& self, reg: Reg) -> i32 {
			match reg {
				Reg::A => self.a,
				Reg::B => self.b,
				Reg::C => self.c,
				Reg::D => self.d,
			}
		}

		fn store (& mut self, reg: Reg, val: i32) {
			match reg {
				Reg::A => self.a = val,
				Reg::B => self.b = val,
				Reg::C => self.c = val,
				Reg::D => self.d = val,
			}
		}

		fn exec (& mut self, instrs: & [Instr]) -> GenResult <()> {
			while self.p.as_usize () < instrs.len () {
				if self.l == 0 { return Err ("Reached ops limit".into ()) }
				self.l -= 1;
				match instrs [self.p.as_usize ()] {
					Instr::CpyInt (src, dst) => self.store (dst, src),
					Instr::CpyReg (src, dst) => self.store (dst, self.load (src)),
					Instr::Inc (reg) => self.store (reg, self.load (reg) + 1),
					Instr::Dec (reg) => self.store (reg, self.load (reg) - 1),
					Instr::Jnz (src, dst) => {
						if self.load (src) != 0_i32 {
							self.p = u32::add_signed (self.p, dst) ?;
							continue;
						}
					},
					Instr::Jmp (dst) => {
						self.p = u32::add_signed (self.p, dst) ?;
						continue;
					},
				}
				self.p += 1;
			}
			Ok (())
		}

	}

}

pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub instrs: Vec <Instr>,
		pub ops_limit: u32,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Instr {
		CpyInt (i32, Reg),
		CpyReg (Reg, Reg),
		Inc (Reg),
		Dec (Reg),
		Jnz (Reg, i32),
		Jmp (i32),
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Reg { A, B, C, D }

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let ops_limit = parser::input_param (& mut input, "OPS_LIMIT=", 100_000_000) ?;
			#[ allow (clippy::redundant_closure_for_method_calls) ]
			let instrs = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| parser.item ())
						.map_parse_err (|col_idx| format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { instrs, ops_limit })
		}
	}

	impl FromParser for Instr {
		fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let src = parser.expect ("cpy ") ?.int () ?;
					let dst = parser.expect (" ") ?.confirm ().item () ?;
					parser.end () ?;
					Ok (Self::CpyInt (src, dst))
				})
				.of (|parser| {
					let src = parser.expect ("cpy ") ?.item () ?;
					let dst = parser.expect (" ") ?.confirm ().item () ?;
					parser.end () ?;
					Ok (Self::CpyReg (src, dst))
				})
				.of (|parser| {
					let reg = parser.expect ("inc ") ?.confirm ().item () ?;
					parser.end () ?;
					Ok (Self::Inc (reg))
				})
				.of (|parser| {
					let reg = parser.expect ("dec ") ?.confirm ().item () ?;
					parser.end () ?;
					Ok (Self::Dec (reg))
				})
				.of (|parser| {
					let src = parser.expect ("jnz ") ?.item () ?;
					let dst = parser.expect (" ") ?.confirm ().int () ?;
					parser.end () ?;
					Ok (Self::Jnz (src, dst))
				})
				.of (|parser| {
					let src: i32 = parser.expect ("jnz ") ?.int () ?;
					if src == 0_i32 { return Err (parser.err ()) }
					let dst = parser.expect (" ") ?.confirm ().int () ?;
					parser.end () ?;
					Ok (Self::Jmp (dst))
				})
				.done ()
		}
	}

	impl FromParser for Reg {
		fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| { parser.expect ("a") ?.confirm (); Ok (Self::A) })
				.of (|parser| { parser.expect ("b") ?.confirm (); Ok (Self::B) })
				.of (|parser| { parser.expect ("c") ?.confirm (); Ok (Self::C) })
				.of (|parser| { parser.expect ("d") ?.confirm (); Ok (Self::D) })
				.done ()
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
