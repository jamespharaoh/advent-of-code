use std::error::Error;
use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;

fn main () -> Result <(), Box <dyn Error>> {
	let file = File::open ("input") ?;
	let reader = BufReader::new (file);
	let mut total: i64 = 0;
	for line in reader.lines () {
		let line = line ?;
		let delta: i64 = line.parse () ?;
		total += delta;
	}
	println! ("Total: {}", total);
	Ok (())
}
