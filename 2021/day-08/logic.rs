use super::*;

use input::Input;
use model::Digit;
use model::Display;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let displays = Display::build_vec (& input.displays) ?;
	Ok (
		displays.iter ()
			.flat_map (|display| display.value.iter ().copied ()
				.map (Digit::num_segments))
			.filter (|& num_segments| matches! (num_segments, 2 | 3 | 4 | 7))
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let displays = Display::build_vec (& input.displays) ?;
	if input.params.use_solver {
		solver::part_two (& displays)
	} else {
		bits::part_two (& displays)
	}
}

fn decode_value (display: Display, digits: [u8; 10]) -> GenResult <u16> {
	let decode =
		|digit: Digit| digits.iter ()
			.position (|& val| digit.on () == val)
			.map (usize::pan_u16)
			.ok_or ("No solution found");
	display.value.iter ()
		.try_fold (0_u16, |sum, & digit| Ok (sum * 10 + decode (digit) ?))
}

mod bits {

	use super::*;

	pub fn part_two (displays: & [Display]) -> GenResult <u32> {	
		displays.iter ()
			.map (|& display| decode_display (display))
			.try_fold (0_u32, |sum, val| Ok (sum + val ?.pan_u32 ()))
	}

	fn decode_display (display: Display) -> GenResult <u16> {
		decode_value (display, get_digits (display) ?)
	}

	fn get_digits (display: Display) -> GenResult <[u8; 10]> {
		let mut temp = display.samples.map (Digit::num_segments);
		temp.sort ();
		if temp != [2, 3, 4, 5, 5, 5, 6, 6, 6, 7] {
			return Err ("No solution found".into ());
		}
		let [mut segs_acf, mut segs_bcdf, mut segs_bcef, mut segs_cde, mut segs_cf] = [0; 5];
		let segs_abcdefg = 0x7f;
		for & digit in & display.samples {
			match digit.num_segments () {
				2 => segs_cf = digit.on (),
				3 => segs_acf = digit.on (),
				4 => segs_bcdf = digit.on (),
				5 => segs_bcef |= digit.off (),
				6 => segs_cde |= digit.off (),
				7 => (),
				_ => unreachable! (),
			}
		}
		let seg_a = segs_acf & ! segs_cf;
		let seg_d = segs_cde & ! segs_bcef;
		let seg_b = segs_bcdf & ! (segs_cf | seg_d);
		let seg_f = segs_cf & ! segs_cde;
		let seg_c = segs_cf & ! seg_f;
		let seg_e = segs_cde & ! (seg_c | seg_d);
		let seg_g = segs_abcdefg & ! (segs_bcdf | seg_a | seg_e);
		Ok ([
			segs_abcdefg & ! seg_d,
			seg_c | seg_f,
			segs_abcdefg & ! (seg_b | seg_f),
			segs_abcdefg & ! (seg_b | seg_e),
			segs_abcdefg & ! (seg_a | seg_e | seg_g),
			segs_abcdefg & ! (seg_c | seg_e),
			segs_abcdefg & ! seg_c,
			seg_a | seg_c | seg_f,
			segs_abcdefg,
			segs_abcdefg & ! seg_e,
		])
	}

}

mod solver {

	use super::*;
	use crate::solver::*;

	pub fn part_two (displays: & [Display]) -> GenResult <u32> {	
		let solver = DigitsSolver::build ();
		displays.iter ()
			.map (|& display| decode_display (& solver, display))
			.try_fold (0_u32, |sum, val| Ok (sum + val ?.pan_u32 ()))
	}

	fn decode_display (solver: & DigitsSolver, display: Display) -> GenResult <u16> {
		decode_value (display, solver.solve (display.samples) ?)
	}

	pub struct DigitsSolver {
		solver: Solver <u8>,
		digit_vars: [SolverVar; 10],
	}

	impl DigitsSolver {

		fn build () -> Self {

			#[ inline ] const fn inv (val: u8) -> u8 { val ^ 0x7f }

			let (mut solver, vars) = make_solver! {
				value_type = u8;
				variables {
					(zero, one, two, three, four, five, six, seven, eight, nine) = 0 ..= 0x7f;
					unique (a, b, c, d, e, f, g) = (0_u8 .. 7).map (|bit| 1 << bit);
				}
				constraints {
					|zero, d| inv (zero) == d;
					|one, c, f| one == c | f;
					|two, b, f| inv (two) == b | f;
					|three, b, e| inv (three) == b | e;
					|four, a, e, g| inv (four) == a | e | g;
					|five, c, e| inv (five) == c | e;
					|six, c| inv (six) == c;
					|seven, a, c, f| seven == a | c | f;
					|eight| inv (eight) == 0;
					|nine, e| inv (nine) == e;
				}
			};

			let digit_vars = [
				vars.zero, vars.one, vars.two, vars.three, vars.four,
				vars.five, vars.six, vars.seven, vars.eight, vars.nine,
			];

			solver.reduce ();

			Self { solver, digit_vars }

		}

		pub fn solve (& self, digits: [Digit; 10]) -> GenResult <[u8; 10]> {
			let mut solver = self.solver.clone ();
			for (digit_idx, & num_segs) in DIGIT_SEGMENTS.iter ().enumerate () {
				let samples: ArrayVec <u8, 3> = digits.iter ()
					.filter (|digit| digit.num_segments () == num_segs)
					.map (|digit| digit.on ())
					.collect ();
				solver.constrain (
					& [ self.digit_vars [digit_idx] ],
					move |vars| samples.contains (& vars [0]));
			}
			solver.reduce ();
			let soln = solver.iter ().next ().ok_or ("No solution found") ?;
			Ok (self.digit_vars.map (|var| soln [var]))
		}

	}

	const DIGIT_SEGMENTS: [u32; 10] = [ 6, 2, 5, 5, 4, 5, 6, 3, 7, 6 ];

}
