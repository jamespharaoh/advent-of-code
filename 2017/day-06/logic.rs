use super::*;

use input::Input;
use model::Banks;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let (_, cycle) = calc_result (input);
	Ok (cycle)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let (cycle_0, cycle_1) = calc_result (input);
	Ok (cycle_1 - cycle_0)
}

fn calc_result (input: & Input) -> (u32, u32) {
	let mut banks = input.banks.clone ();
	let mut seen: HashMap <Banks, u32> = HashMap::new ();
	for cycle in 0 .. {
		if let Some (prev_cycle) = seen.insert (banks.clone (), cycle) {
			return (prev_cycle, cycle);
		}
		let (mut idx, & val) = banks.iter ()
			.enumerate ()
			.max_by_key (|& (idx, & val)| (val, cmp::Reverse (idx)))
			.unwrap ();
		banks [idx] = 0;
		for _ in 0 .. val {
			idx += 1;
			if idx == banks.len () { idx = 0; }
			banks [idx] += 1;
		}
	}
	unreachable! ();
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.banks.len () < 2 {
		return Err ("Must have at least two banks".into ());
	}
	if input.banks.iter ().any (|& val| val > 24) {
		return Err ("Max size of memory in one bank is 24".into ());
	}
	Ok (())
}
