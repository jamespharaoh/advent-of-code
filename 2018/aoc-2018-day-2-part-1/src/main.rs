use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;

fn main () -> Result <(), Box::<dyn Error>> {
	let file = File::open ("input") ?;
	let reader = BufReader::new (file);
	let mut num_twos: u64 = 0;
	let mut num_threes: u64 = 0;
	for line in reader.lines () {
		let line = line ?;
		let mut letters: HashMap <char, u64> = HashMap::new ();
		for ch in line.chars () {
			(* letters.entry (ch).or_insert (0)) += 1;
		}
		let nums: HashSet <u64> = letters.into_values ().collect ();
		if nums.contains (& 2) { num_twos += 1 }
		if nums.contains (& 3) { num_threes += 1 }
	}
	println! ("Number of twos: {}", num_twos);
	println! ("Number of threes: {}", num_threes);
	println! ("Rudimentary checksum: {}", num_twos * num_threes);
	Ok (())
}
