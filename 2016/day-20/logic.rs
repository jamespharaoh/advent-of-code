use super::*;

use input::Input;
use input::Rule;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let rules: Vec <Rule> =
		input.rules.iter ().copied ()
			.sorted ()
			.collect ();
	let mut first = 0;
	for rule in rules {
		if rule.end < rule.start { continue }
		if first < rule.start { continue }
		if rule.end == u32::MAX { return Err ("No solution found".into ()) }
		first = cmp::max (first, rule.end + 1);
	}
	Ok (first)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	if input.rules.is_empty () { return Err ("Overflow".into ()) }
	let rules: Vec <Rule> =
		input.rules.iter ().copied ()
			.sorted ()
			.collect ();
	let mut last = 0;
	let mut valid = 0;
	for rule in rules {
		if rule.end < rule.start { continue }
		if last < rule.start {
			valid += rule.start - last - 1;
		}
		last = cmp::max (last, rule.end);
	}
	Ok (valid + (u32::MAX - last))
}
