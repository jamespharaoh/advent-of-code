use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn aoc2018_day2_part1 (input: & str) -> Result <String, Box <dyn Error>> {
	let mut num_twos: u64 = 0;
	let mut num_threes: u64 = 0;
	for line in input.trim ().split ("\n") {
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
	Ok (format! ("{}", num_twos * num_threes))
}
