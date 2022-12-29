use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	if input.rucksacks.is_empty () {
		return Err ("Must have at least one rucksack".into ());
	}
	input.rucksacks.iter ()
		.map (|items| {
			let num_chars = items.chars ().count ();
			if num_chars & 1 != 0 {
				return Err ("Rucksack must contain even number of items".into ());
			}
			let rucksack_bits_0 = get_rucksack_bits (items.chars ().take (num_chars / 2)) ?;
			let rucksack_bits_1 = get_rucksack_bits (items.chars ().rev ().take (num_chars / 2)) ?;
			let rucksack_bits = rucksack_bits_0 & rucksack_bits_1;
			if rucksack_bits.count_ones () != 1 {
				return Err ("No solution found".into ());
			}
			GenOk (rucksack_bits.trailing_zeros ())
		})
		.try_fold (0, |sum, val| Ok (chk! (sum + val ?) ?))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	if input.rucksacks.is_empty () {
		return Err ("Must have at least one rucksack".into ());
	}
	if input.rucksacks.len () % 3 != 0 {
		return Err ("Number of rucksacks must be a multiple of three".into ());
	}
	input.rucksacks.chunks (3)
		.map (move |group| {
			let group_bits =
				group.iter ()
					.map (|rucksack_items| get_rucksack_bits (rucksack_items.chars ()))
					.reduce (|bits_0, bits_1| Ok (bits_0 ? & bits_1 ?))
					.unwrap () ?;
			if group_bits.count_ones () != 1 {
				return Err ("No solution found".into ());
			}
			GenOk (group_bits.trailing_zeros ())
		})
		.try_fold (0, |sum, val| Ok (chk! (sum + val ?) ?))
}

fn get_rucksack_bits (mut items: impl Iterator <Item = char>) -> GenResult <u64> {
	items.try_fold (0_u64, |bits, item| Ok (bits | 1 << get_item_priority (item) ?))
}

fn get_item_priority (ch: char) -> GenResult <u32> {
	Ok (match ch {
		'a' ..= 'z' => ch.pan_u32 () - 'a'.pan_u32 () + 1,
		'A' ..= 'Z' => ch.pan_u32 () - 'A'.pan_u32 () + 27,
		_ => return Err ("Invalid item: {ch}".into ()),
	})
}
