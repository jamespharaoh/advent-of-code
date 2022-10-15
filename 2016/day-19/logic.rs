use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	if input.num_elves < 2 { return Err ("Must have at least two elves".into ()) }
	let mut first_with = 0;
	let mut sep = 1;
	let mut rem = input.num_elves;
	let mut take = false;
	while rem > 1 {
		let odd = (rem & 1) == 1;
		if ! take && odd { rem = (rem + 1) / 2; } else { rem /= 2; }
		if take { first_with += sep; }
		if odd { take = ! take; }
		sep = u32::mul_2 (sep, 2) ?;
	}
	Ok (first_with + 1)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	if input.num_elves < 2 { return Err ("Must have at least two elves".into ()) }
	let mut elves =
		((input.num_elves + 2) / 2 ..= input.num_elves)
			.chain (1 .. (input.num_elves + 2) / 2)
			.collect::<Vec <_>> ();
	let mut seq = 1 + (input.num_elves) % 2;
	while elves.len () > 3 {
		let next_seq = (seq + elves.len ().pan_u32 ()) % 3;
		elves = elves.iter ()
			.scan (seq, |state, & elf| {
				let seq = * state;
				* state += 1;
				if * state == 3 { * state = 0; }
				Some ((seq, elf))
			})
			.filter (|& (seq, _)| seq == 0)
			.map (|(_, elf)| elf)
			.collect ();
		seq = next_seq;
	}
	while elves.len () > 1 {
		let elf = elves.remove (0);
		if seq == 0 { elves.push (elf); }
		seq = (seq + 1) % 3;
	}
	Ok (elves [0])
}
