//! Quick algorithm found by reverse engineering the input programme.
//!
//! # Quick start
//!
//! Call [`iterator`] to create an iterator over the valid solutions:
//!
//! ```
//! # use aoc_2021_day_24::*;
//! # let steps = [quick::Step { random: false, check: 0, increment: 0 }; 14];
//! for solution in quick::iterator (& steps, true) {
//!   println! ("{:?}", solution);
//! }
//! ```
//!
//! # Description
//!
//! Looking carefully at the provided programme, it seems to be the same 18 instructions repeated,
//! once for every input character. There are only three that vary. Here's the template:
//!
//! ```text
//!  1  inp w
//!  2  mul x 0
//!  3  add x z
//!  4  mod x 26
//!  5  div z (1 or 26)
//!  6  add x (-16 to 13)
//!  7  eql x w
//!  8  eql x 0
//!  9  mul y 0
//! 10  add y 25
//! 11  mul y x
//! 12  add y 1
//! 13  mul z y
//! 14  mul y 0
//! 15  mul y w
//! 16  add y (3 to 15)
//! 17  mul y x
//! 18  add z y
//! ```
//!
//! So, the registers seem to be used as follows:
//!
//!
//! * `w` --- The current input value. This is loaded in the first instruction and not modified
//! again.
//! * `x` --- Temporary value, the value from the previous step is discarded. This is set to `0` if
//! the input value matches a calculation based on `z` and a parameter which varies for each step,
//! or to `1` otherwise.
//! * `y` --- Another temporary value, this is used to calculate values based on `x`, `w` and a
//! parameter, and then used to update `z`. The value is carefully calculated to not update `z` if
//! `x` is set to `0`, by setting it to `1` for a multiply or `0` for an add.
//! * `z` --- An accumulator, which is kept between steps. This starts off at `0` and gets
//! conditionally multiplied by 26, divided by 26, and added to.
//!
//! We can compile the differences in each step into a table of parameters. Since a divide by one
//! is an identity operation, we reduce the first difference into a boolean, tracking whether we
//! will divide of not. We will name the other two parameters `check` and `increment`, based on
//! how they are used. So using the programme I was given as an example, we end up with the
//! following:
//!
//! | Parameter   | 1    | 2    | 3    | 4    | 5    | 6     | 7    | 8     | 9     | 10   | 11   | 12   | 13   | 14    |
//! |-------------|:----:|:----:|:----:|:----:|:----:|:-----:|:----:|:-----:|:-----:|:----:|:----:|:----:|:----:|:-----:|
//! | `divide`    |      |      |      |      |      | yes   |      | yes   | yes   |      | yes  | yes  | yes  | yes   |
//! | `check`     | `10` | `12` | `10` | `12` | `11` | `-16` | `10` | `-11` | `-13` | `13` | `-8` | `-1` | `-4` | `-14` |
//! | `increment` | `12` | `7`  | `8`  | `8`  | `15` | `12`  | `8`  | `13`  | `3`   | `13` | `3`  | `9`  | `4`  | `13`  |
//!
//! And we can descibe the algorithm for each step as code. Here we use `acc` to refer to the
//! shared state stored in register `z`, and we expect the parameters for each step to be passed in
//! a struct:
//!
//! ```
//! struct StepParams {
//!     divide: bool,
//!     check: i64,
//!     increment: i64,
//! }
//! fn apply_step (params: & StepParams, acc: & mut i64, input: i64) {
//!     if input == * acc % 26 + params.check { // instructions 2-4,6-8
//!         if params.divide { * acc /= 26; }   // instruction 5
//!     } else {
//!         if ! params.divide { * acc *= 26; } // instructions 9-13
//!         * acc += input + params.increment;  // instructions 14-18
//!     }
//! }
//! ```
//!
//! We can either divide `acc` by 26, multiply it by 26 and add something, or just add something.
//! Furthermore, in seven of our fourteen steps we always multiply, so to get this value down to
//! zero we are going to have to make sure we divide on all seven of the other steps. The extra
//! addition from increment will have to be removed as a rounding error from the division - this
//! presumably is intended to reduce the total number of valid inputs.
//!
//! Since we are deciding whether to divide based on a value derived from `acc` and `check`, we
//! can simply calculate the correct value based on the previous digits. This reduces our problem
//! from searching 9¹⁴ (twenty-three trillion) combinations, down to only 9⁷ (five million).
//! What's more, we will often be able to shortcut the search if there is no matching next digit.
//!
//! This leads to the algorithm implemented here. The steps are encoded in the [`Step`] struct,
//! and the `steps_for` function reads an input programme and identifies the correct parameters
//! for each step. I inverted the `divide` parameter around and called it [`random`][Step::random]
//! - this indicates that we should choose a random value for this input value, or in practice try
//! all possible values. The `iterator` function returns an iterator over all of the valid inputs,
//! actually 14 nested iterators plus a final call to `map` at the end to check the final sum, and
//! to convert to match the function signature.
//!
//! The actual algorithm is implemented in the [`NextNumIter`] struct. This is an iterator
//! adapter, which takes an iterator over partial solutions and provides the possible answers with
//! the next digit included. If this is a `random` step, then this expands every `n`-digit entry
//! into nine `n+1`-digit entries, one for each valid digit. Otherwise, it will either provide a
//! single entry with the appropriate next digit, or no entries if the correct next digit is not
//! in the valid range.
//!
//! # Performance
//!
//! This shows the time for finding the answers for the puzzle, and for finding all valid codes,
//! plus the peak memory usage, for each build version.
//!
//! | Version | Puzzle    | All        | Memory   |
//! |:-------:|:---------:|:----------:|:--------:|
//! | Debug   | 0.56 secs | 2.23 secs  | 2.88 MiB |
//! | Release | 0.10 secs | 0.39 secs  | 2.20 MiB |

use super::*;
use machine::Instr;
use machine::Reg;
use machine::RegOrInt;
use model::Input;

#[ derive (Clone, Copy, Debug) ]
pub struct Step {
	pub random: bool,
	pub check: i64,
	pub increment: i64,
}

impl Step {

	#[ inline ]
	#[ must_use ]
	pub const fn new (random: bool, check: i64, increment: i64) -> Self {
		Self { random, check, increment }
	}

	#[ inline ]
	#[ must_use ]
	pub fn solve (self, progress: i64) -> Option <u8> {
		let next = progress % 26 + self.check;
		(1 ..= 9).contains (& next).then (|| next.pan_u8 ())
	}

	#[ inline ]
	#[ must_use ]
	pub fn incr (self, progress: i64, next: u8) -> i64 {
		if self.random {
			progress * 26 + next.pan_i64 () + self.increment
		} else {
			progress / 26
		}
	}

}

pub fn steps_for (prog: & [Instr]) -> GenResult <[Step; 14]> {
	if prog.len () != 14 * 18 { Err ("Programme is not the right length") ? }
	Ok (prog.chunks (18).enumerate ().map (|(chunk_idx, chunk)| {
		let err = |line_idx|
			format! ("Unable to generate quick steps for programme at line {}",
				chunk_idx * 18 + line_idx + 1).into ();
		let expect = |line_idx, val| -> GenResult <()> {
			if chunk [line_idx] == val { Ok (()) } else { Err (err (line_idx)) ? }
		};
		expect (0, Instr::Inp (Reg::W)) ?;
		expect (1, Instr::Mul (Reg::X, RegOrInt::Int (0))) ?;
		expect (2, Instr::Add (Reg::X, RegOrInt::Z)) ?;
		expect (3, Instr::Mod (Reg::X, RegOrInt::Int (26))) ?;
		let random = if let Instr::Div (Reg::Z, RegOrInt::Int (val)) = chunk [4] {
			if val == 26 { false } else if val == 1 { true } else { return Err (err (4)); }
		} else { Err (err (4)) ? };
		let check = if let Instr::Add (Reg::X, RegOrInt::Int (check)) = chunk [5] {
			check
		} else { Err (err (5)) ? };
		expect (6, Instr::Eql (Reg::X, RegOrInt::W)) ?;
		expect (7, Instr::Eql (Reg::X, RegOrInt::Int (0))) ?;
		expect (8, Instr::Mul (Reg::Y, RegOrInt::Int (0))) ?;
		expect (9, Instr::Add (Reg::Y, RegOrInt::Int (25))) ?;
		expect (10, Instr::Mul (Reg::Y, RegOrInt::X)) ?;
		expect (11, Instr::Add (Reg::Y, RegOrInt::Int (1))) ?;
		expect (12, Instr::Mul (Reg::Z, RegOrInt::Y)) ?;
		expect (13, Instr::Mul (Reg::Y, RegOrInt::Int (0))) ?;
		expect (14, Instr::Add (Reg::Y, RegOrInt::W)) ?;
		let increment = if let Instr::Add (Reg::Y, RegOrInt::Int (increment)) = chunk [15] {
			increment
		} else { Err (err (15)) ? };
		expect (16, Instr::Mul (Reg::Y, RegOrInt::X)) ?;
		expect (17, Instr::Add (Reg::Z, RegOrInt::Y)) ?;
		Ok (Step { random, check, increment })
	}).collect::<GenResult <Vec <Step>>> () ?.try_into ().unwrap ())
}

pub fn iterator (steps: & [Step; 14], reverse: bool) -> impl Iterator <Item = Input> + '_ {
	NextNumIter::new (steps, reverse).nest ().nest ().nest ().nest ().nest ().nest ().nest ().nest ()
		.nest ().nest ().nest ().nest ().nest ().filter_map (|(nums, progress)| {
			(progress == 0).then_some ({
				let mut answer = [0; 14];
				for idx in 0 .. 14 { answer [idx] = nums [idx] }
				answer
			})
		})
}

type TempAnswer = (TinyVec <u8, 14>, i64);

pub enum NextNumIter <'stp, Nested> {
	Outer {
		steps: & 'stp [Step; 14],
		nested: Nested,
		reverse: bool,
	},
	Inner {
		steps: & 'stp [Step; 14],
		nested: Nested,
		reverse: bool,
		nums: TinyVec <u8, 14>,
		progress: i64,
		next_iter: ops::RangeInclusive <u8>,
	},
	Poison,
}

impl <'stp> NextNumIter <'stp, std::option::IntoIter <TempAnswer>> {
	fn new (
		steps: & 'stp [Step; 14],
		reverse: bool,
	) -> NextNumIter <'stp, std::option::IntoIter <TempAnswer>> {
		let a: TempAnswer = (TinyVec::new (), 0);
		NextNumIter::Outer { steps, nested: Some (a).into_iter (), reverse }
	}
}

impl <'stp, Nested: Iterator <Item = TempAnswer>> NextNumIter <'stp, Nested> {
	fn nest (self) -> NextNumIter <'stp, Self> {
		match & self {
			& NextNumIter::Outer { steps, reverse, .. } =>
				NextNumIter::Outer { steps, nested: self, reverse },
			& NextNumIter::Inner { .. } | & NextNumIter::Poison => panic! (),
		}
	}
}

impl <'stp, Nested: Iterator <Item = TempAnswer>> Iterator for NextNumIter <'stp, Nested> {
	type Item = TempAnswer;
	#[ allow (clippy::reversed_empty_ranges) ]
	fn next (& mut self) -> Option <TempAnswer> {
		loop { match mem::replace (self, NextNumIter::Poison) {
			NextNumIter::Outer { steps, mut nested, reverse } => match nested.next () {
				Some ((nums, progress)) => {
					let step = steps [nums.len ()];
					let next_iter = if step.random {
						1 ..= 9
					} else if let Some (next) = step.solve (progress) {
						next ..= next
					} else {
						10 ..= 9
					};
					* self = NextNumIter::Inner { steps, nested, reverse, nums, progress, next_iter };
					continue;
				},
				None => {
					return None;
				},
			},
			NextNumIter::Inner { steps, nested, reverse, nums, progress, mut next_iter } => {
				if let Some (next) = if reverse { next_iter.next_back () } else { next_iter.next () } {
					let step = steps [nums.len ()];
					let mut new_nums = nums.clone ();
					new_nums.push (next);
					let new_progress = step.incr (progress, next);
					* self = NextNumIter::Inner { steps, nested, reverse, nums, progress, next_iter };
					return Some ((new_nums, new_progress));
				}
				* self = NextNumIter::Outer { steps, nested, reverse };
				continue;
			},
			NextNumIter::Poison => panic! (),
		} }
	}
}
