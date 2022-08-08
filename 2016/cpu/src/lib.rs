//! Advent of Code 2016: CPU
//!
//! CPU used in the following puzzles:
//! - [https://adventofcode.com/2016/day/12](https://adventofcode.com/2016/day/12)
//! - [https://adventofcode.com/2016/day/23](https://adventofcode.com/2016/day/23)
//! - [https://adventofcode.com/2016/day/25](https://adventofcode.com/2016/day/25)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use parser::FromParser;
use parser::ParseResult;
use parser::Parser;

#[ derive (Default) ]
pub struct Cpu {
	pub instrs: Vec <Instr>,
	pub reg_a: i32,
	pub reg_b: i32,
	pub reg_c: i32,
	pub reg_d: i32,
	pub limit: u32,
	pub next: u32,
}

impl Cpu {

	const fn load (& self, arg: Arg) -> i32 {
		match arg {
			Arg::RegA => self.reg_a,
			Arg::RegB => self.reg_b,
			Arg::RegC => self.reg_c,
			Arg::RegD => self.reg_d,
			Arg::Imm (val) => val,
		}
	}

	fn store (& mut self, arg: Arg, val: i32) {
		match arg {
			Arg::RegA => self.reg_a = val,
			Arg::RegB => self.reg_b = val,
			Arg::RegC => self.reg_c = val,
			Arg::RegD => self.reg_d = val,
			Arg::Imm (_) => (),
		}
	}

	pub fn exec (& mut self) -> GenResult <()> {
		while self.next.as_usize () < self.instrs.len () {
			if self.limit == 0 { return Err ("Reached ops limit".into ()) }
			self.limit -= 1;
			#[ allow (clippy::match_on_vec_items) ]
			match self.instrs [self.next.as_usize ()] {
				Instr::Cpy (src, dst) => self.store (dst, self.load (src)),
				Instr::Inc (reg) => self.store (reg, i32::add_2 (self.load (reg), 1_i32) ?),
				Instr::Dec (reg) => self.store (reg, i32::sub_2 (self.load (reg), 1_i32) ?),
				Instr::Jnz (src, dst) => { self.jnz (src, dst) ?; continue }
				Instr::Tgl (dst) => {
					let idx = u32::add_signed (self.next, self.load (dst)) ?;
					if let Some (instr) = self.instrs.get_mut (idx.as_usize ()) {
						* instr = match * instr {
							Instr::Cpy (src, dst) => Instr::Jnz (src, dst),
							Instr::Jnz (src, dst) => Instr::Cpy (src, dst),
							Instr::Inc (arg) => Instr::Dec (arg),
							Instr::Dec (arg) | Instr::Tgl (arg) => Instr::Inc (arg),
						};
					}
				},
			}
			self.next += 1;
		}
		Ok (())
	}

	fn jnz (& mut self, src: Arg, dst: Arg) -> GenResult <()> {

		// ignore if src is zero

		if self.load (src) == 0_i32 {
			self.next += 1;
			return Ok (());
		}

		// check if this can be optimized

		if self.next >= 2 {
			match <[Instr; 3]>::try_from (
				& self.instrs [self.next.as_usize () - 2 .. self.next.as_usize () + 1],
			).unwrap () {
				[
					Instr::Dec (arg),
					Instr::Inc (dst),
					Instr::Jnz (arg_0, Arg::Imm (-2_i32)),
				] | [
					Instr::Inc (dst),
					Instr::Dec (arg),
					Instr::Jnz (arg_0, Arg::Imm (-2_i32)),
				] if arg == arg_0 && dst != arg => {
					self.store (dst, i32::add_2 (self.load (dst), self.load (arg)) ?);
					self.store (arg, 0);
					return Ok (());
				},
				_ => (),
			}
		}

		if self.next >= 5 {
			match <[Instr; 6]>::try_from (
				& self.instrs [self.next.as_usize () - 5 .. self.next.as_usize () + 1],
			).unwrap () {
				[
					Instr::Cpy (arg, tmp),
					Instr::Inc (dst),
					Instr::Dec (tmp_0),
					Instr::Jnz (tmp_1, Arg::Imm (-2_i32)),
					Instr::Dec (ctr),
					Instr::Jnz (ctr_0, Arg::Imm (-5_i32)),
				] if tmp == tmp_0 && tmp == tmp_1 && ctr == ctr_0 && arg != tmp && arg != dst
						&& arg != ctr || tmp != dst && tmp != ctr && dst != ctr => {
					self.store (dst, i32::add_2 (
						self.load (dst),
						i32::mul_2 (self.load (arg), self.load (ctr)) ?,
					) ?);
					self.store (tmp, 0);
					self.store (ctr, 0);
					return Ok (());
				},
				_ => (),
			}
		}

		// if not then jump as described

		self.next = u32::add_signed (self.next, self.load (dst)) ?;
		Ok (())

	}

}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Instr {
	Cpy (Arg, Arg),
	Inc (Arg),
	Dec (Arg),
	Jnz (Arg, Arg),
	Tgl (Arg),
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Arg { RegA, RegB, RegC, RegD, Imm (i32) }

impl Instr {

	#[ must_use ]
	pub const fn is_v1 (& self) -> bool {
		use Instr::{ Cpy, Dec, Inc, Jnz };
		matches! (* self, Cpy (_, _) | Inc (_) | Dec (_) | Jnz (_, _))
	}

	#[ must_use ]
	pub const fn is_v2 (& self) -> bool {
		use Instr::{ Cpy, Dec, Inc, Jnz, Tgl };
		matches! (* self, Cpy (_, _) | Inc (_) | Dec (_) | Jnz (_, _) | Tgl (_))
	}

}

impl FromParser for Instr {
	fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| {
				let arg_0 = parser.expect ("cpy ") ?.confirm ().item () ?;
				let arg_1 = parser.expect (" ") ?.item () ?;
				parser.end () ?;
				Ok (Self::Cpy (arg_0, arg_1))
			})
			.of (|parser| {
				let arg = parser.expect ("inc ") ?.confirm ().item () ?;
				parser.end () ?;
				Ok (Self::Inc (arg))
			})
			.of (|parser| {
				let arg = parser.expect ("dec ") ?.confirm ().item () ?;
				parser.end () ?;
				Ok (Self::Dec (arg))
			})
			.of (|parser| {
				let arg_0 = parser.expect ("jnz ") ?.confirm ().item () ?;
				let arg_1 = parser.expect (" ") ?.confirm ().item () ?;
				parser.end () ?;
				Ok (Self::Jnz (arg_0, arg_1))
			})
			.of (|parser| {
				let arg = parser.expect ("tgl ") ?.confirm ().item () ?;
				parser.end () ?;
				Ok (Self::Tgl (arg))
			})
			.done ()
	}
}

impl FromParser for Arg {
	fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| { parser.expect ("a") ?; Ok (Self::RegA) })
			.of (|parser| { parser.expect ("b") ?; Ok (Self::RegB) })
			.of (|parser| { parser.expect ("c") ?; Ok (Self::RegC) })
			.of (|parser| { parser.expect ("d") ?; Ok (Self::RegD) })
			.of (|parser| { let val = parser.int () ?; Ok (Self::Imm (val)) })
			.done ()
	}
}
