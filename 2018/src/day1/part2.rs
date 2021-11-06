use std::collections::HashSet;
use std::error::Error;
use std::iter;

pub fn aoc2018_day1_part2 (input: & str) -> Result <String, Box <dyn Error>> {
	let deltas: Result <Vec <i64>, Box <dyn Error>> = input.trim ().split ("\n").map (
		|line| Ok (line.parse () ?),
	).collect ();
	let deltas = deltas ?;
	let mut total: i64 = 0;
	let mut seen: HashSet <i64> = HashSet::new ();
	for delta in iter::repeat (deltas).flatten () {
		total += delta;
		if seen.contains (& total) { break }
		seen.insert (total);
	}
	Ok (format! ("{}", total))
}
