//! Logic for solving the puzzles

use super::*;

use input::Input;
use input::InputPerson;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (calc_result (input, |sum, item| sum | item))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (calc_result (input, |sum, item| sum & item))
}

fn calc_result (input: & Input, reduce_fn: fn (u32, u32) -> u32) -> u32 {
	input.groups.iter ()
		.map (|group| group.people.iter ()
			.map (get_bits)
			.reduce (reduce_fn)
			.unwrap_or (0))
		.map (u32::count_ones)
		.sum ()
}

fn get_bits (person: & InputPerson) -> u32 {
	person.chars ()
		.fold (0, |sum, ch| {
			assert! (('a' ..= 'z').contains (& ch));
			let bit = ch.pan_u32 () - 'a'.pan_u32 ();
			sum | (1 << bit)
		})
}
