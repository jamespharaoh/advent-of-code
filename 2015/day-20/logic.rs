//! Logic for solving the puzzles.

use super::*;

use input::Input;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	Ok (calc_result (input.target, 10, Val::MAX) ?)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	Ok (calc_result (input.target, 11, 50) ?)
}

pub fn calc_result (target: Val, mul: Val, lim: Val) -> NumResult <Val> {

	let mut divs = Vec::new ();
	let mut extend_sqrt = Val::ONE;
	let mut extend = Val::ONE;

	for house in Val::ONE .. {
		let mut total = Val::ZERO;

		// decrement all the values in `divs`, or if they are at zero then include the
		// corresponding "elf" and its complement in the total and reset to the elf's number
		// minus one

		let mut div = Val::ZERO;
		for next in divs.iter_mut () {
			div = Val::add_2 (div, Val::ONE) ?;
			if * next == Val::ZERO {
				* next = Val::sub_2 (div, 1) ?;
				let comp =
					if div == Val::ONE { house }
					else if div == 2 { house >> 1_i32 }
					else { Val::div_2 (house, div) ? };
				if comp <= lim {
					total = Val::add_2 (total, Val::mul_2 (div, mul) ?) ?;
				}
				if comp != div && div <= lim {
					total = Val::add_2 (total, Val::mul_2 (comp, mul) ?) ?;
				}
			} else {
				* next = Val::sub_2 (* next, Val::ONE) ?;
			}
		}

		// once the house square root reaches a new integer we add it to divs, also we have to
		// include the corresponding elf in our total

		if house == extend {
			if divs.is_empty () {
				divs.push (Val::ZERO);
			} else {
				divs.push (Val::from_usize (divs.len ()) ?);
			}
			total = Val::add_2 (total, Val::mul_2 (extend_sqrt, mul) ?) ?;
			extend_sqrt = Val::add_2 (extend_sqrt, Val::ONE) ?;
			extend = Val::mul_2 (extend_sqrt, extend_sqrt) ?;
		}

		// return when we find a solution

		if total >= target { return Ok (house) }

	}

	unreachable! ();

}
