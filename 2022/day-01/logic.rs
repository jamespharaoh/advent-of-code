use super::*;

use input::Input;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.elves.iter ()
			.map (|elf| elf.items.iter ().sum ())
			.max ()
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.elves.iter ()
			.map (|elf| elf.items.iter ().sum ())
			.sorted_by_key (|& elf: & Val| cmp::Reverse (elf))
			.into_iter ()
			.take (3)
			.sum ()
	)
}
