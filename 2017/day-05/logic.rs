use super::*;

use input::Input;
use model::Tramp;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut tramps = input.tramps.clone ();
	let mut offset = Tramp::ZERO;
	let mut count = 0_u32;
	while offset >= Tramp::ZERO && offset.pan_usize () < tramps.len () {
		let tramp = & mut tramps [offset.pan_usize ()];
		offset += * tramp;
		* tramp += Tramp::ONE;
		count += 1_u32;
	}
	Ok (count)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	const THREE: Tramp = 3;
	let mut tramps = input.tramps.clone ();
	let mut offset = Tramp::ZERO;
	let mut count = 0_u32;
	while offset >= Tramp::ZERO && offset.pan_usize () < tramps.len () {
		let tramp = & mut tramps [offset.pan_usize ()];
		offset += * tramp;
		* tramp = if * tramp >= THREE { * tramp - Tramp::ONE } else { * tramp + Tramp::ONE };
		count += 1_u32;
	}
	Ok (count)
}
