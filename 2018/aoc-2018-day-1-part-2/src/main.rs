use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;
use std::iter;

fn main () -> Result <(), Box <dyn Error>> {
	let file = File::open ("input") ?;
	let reader = BufReader::new (file);
	let deltas: Result <Vec <i64>, Box <dyn Error>> = reader.lines ().map (|line| {
		Ok (line ?.parse () ?)
	}).collect ();
	let deltas = deltas ?;
	let mut total: i64 = 0;
	let mut seen: HashSet <i64> = HashSet::new ();
	for delta in iter::repeat (deltas).flatten () {
		total += delta;
		if seen.contains (& total) { break }
		seen.insert (total);
	}
	println! ("First repeated frequency: {}", total);
	Ok (())
}
