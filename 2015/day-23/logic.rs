//! Logic for solving the puzzles.

use super::*;

use input::Input;
use model::Instr;
use model::Reg;
use model::Val;

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
	let instrs_len = Val::from_usize (input.instrs.len ()) ?;
	while next < instrs_len {

		// abort when we reach max_loops

		if max_loops == 0 { return Err (EmulateError::MaxLoops) }
		max_loops -= 1;

		// detect infinite loops and abort

		if ! seen.insert ((reg_a, reg_b, next)) { return Err (EmulateError::InfiniteLoop) }

		// execute next instruction

		let instr = input.instrs [next.pan_usize ()];
		match instr {
			Instr::Hlf (Reg::A) => reg_a /= 2,
			Instr::Hlf (Reg::B) => reg_b /= 2,
			Instr::Tpl (Reg::A) => reg_a = chk! (reg_a * 3) ?,
			Instr::Tpl (Reg::B) => reg_b = chk! (reg_b * 3) ?,
			Instr::Inc (Reg::A) => reg_a = chk! (reg_a + 1_u32) ?,
			Instr::Inc (Reg::B) => reg_b = chk! (reg_b + 1_u32) ?,
			Instr::Jmp (offset) => { next = next.add_signed (offset) ?; continue },
			Instr::Jie (Reg::A, offset) => {
				if reg_a & 1 == 0 { next = next.add_signed (offset) ?; continue }
			},
			Instr::Jie (Reg::B, offset) => {
				if reg_b & 1 == 0 { next = next.add_signed (offset) ?; continue }
			},
			Instr::Jio (Reg::A, offset) => {
				if reg_a == 1 { next = next.add_signed (offset) ?; continue }
			},
			Instr::Jio (Reg::B, offset) => {
				if reg_b == 1 { next = next.add_signed (offset) ?; continue }
			},
		}

		// move to next instruction, jump instructions skip this bit

		chk! (next += 1_u32) ?;

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

impl From <Overflow> for EmulateError {
	fn from (_: Overflow) -> Self {
		Self::Overflow
	}
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
		use input::InputParams;
		use Instr::*;
		use Reg::*;
		fn emul (instrs: impl IntoIterator <Item = Instr>) -> Result <(Val, Val), EmulateError> {
			logic::emulate (
				& Input {
					instrs: instrs.into_iter ().collect (),
					params: InputParams::default (),
				},
				0,
				0,
				0,
				100)
		}
		// inc, hlf, tpl
		assert_eq_ok! ((2, 0), emul ([ Inc (A), Tpl (A), Inc (A), Hlf (A) ]));
		assert_eq_ok! ((0, 2), emul ([ Inc (B), Tpl (B), Inc (B), Hlf (B) ]));
		// jmp
		assert_eq_ok! ((1, 1), emul ([ Inc (A), Jmp (2), Tpl (A), Inc (B) ]));
		// jio
		assert_eq_ok! ((1, 1), emul ([ Inc (A), Jio (A, 2), Inc (B), Inc (B) ]));
		assert_eq_ok! ((0, 2), emul ([ Jio (A, 2), Inc (B), Inc (B) ]));
		assert_eq_ok! ((1, 1), emul ([ Inc (B), Jio (B, 2), Inc (A), Inc (A) ]));
		assert_eq_ok! ((2, 0), emul ([ Jio (B, 2), Inc (A), Inc (A) ]));
		// jie
		assert_eq_ok! ((1, 2), emul ([ Inc (A), Jie (A, 2), Inc (B), Inc (B) ]));
		assert_eq_ok! ((0, 1), emul ([ Jie (A, 2), Inc (B), Inc (B) ]));
		assert_eq_ok! ((2, 1), emul ([ Inc (B), Jie (B, 2), Inc (A), Inc (A) ]));
		assert_eq_ok! ((1, 0), emul ([ Jie (B, 2), Inc (A), Inc (A) ]));
		// errors
		assert_err! ("Infinite loop", emul ([ Jmp (0) ]));
		assert_err! ("Max loops reached", emul ([ Inc (A), Jmp (-1) ]));
		assert_err! ("Arithmetic overflow", emul ([ Inc (A), Tpl (A), Jmp (-1) ]));
	}

}
