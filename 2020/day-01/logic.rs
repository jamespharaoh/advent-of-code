//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let entries: Vec <u16> =
		input.entries.iter ().copied ()
			.filter (|& val| val < 2020)
			.sorted ()
			.dedup ()
			.collect ();
	let (entry_0, entry_1) =
		find_two (
				entries.iter ().copied (),
				entries.iter ().rev ().copied (),
				2020)
			.ok_or ("No solution found") ?;
	Ok (entry_0.pan_u32 () * entry_1.pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let entries: Vec <u16> =
		input.entries.iter ().copied ()
			.filter (|& val| val < 2020)
			.sorted ()
			.dedup ()
			.collect ();
	for & entry_0 in entries.iter () {
		if let Some ((entry_1, entry_2)) =
			find_two (
				entries.iter ().copied ().filter (|& entry_1| entry_1 != entry_0),
				entries.iter ().rev ().copied ().filter (|& entry_2| entry_2 != entry_0),
				2020 - entry_0) {
			return Ok (entry_0.pan_u32 () * entry_1.pan_u32 () * entry_2.pan_u32 ());
		}
	}
	Err ("No solution found".into ())
}

fn find_two (
	iter_fwd: impl Iterator <Item = u16>,
	iter_rev: impl Iterator <Item = u16>,
	sum: u16,
) -> Option <(u16, u16)> {
	let mut iter_fwd = iter_fwd.peekable ();
	let mut iter_rev = iter_rev.peekable ();
	loop {
		let low_val = * iter_fwd.peek () ?;
		let high_val = * iter_rev.peek () ?;
		match Ord::cmp (& (low_val + high_val), & sum) {
			Ordering::Less => iter_fwd.next ().unwrap (),
			Ordering::Greater => iter_rev.next ().unwrap (),
			Ordering::Equal => return Some ((low_val, high_val)),
		};
	}
}
