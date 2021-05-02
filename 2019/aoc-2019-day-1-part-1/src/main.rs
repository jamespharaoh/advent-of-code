use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

fn main () {
	let mut sum: u64 = 0;
	let file = File::open ("input").unwrap ();
	let reader = BufReader::new (file);
	for line in reader.lines () {
		let line: u64 = line.unwrap ().parse ().unwrap ();
		let fuel = (line / 3) - 2;
		sum += fuel;
	}
	println! ("Total fuel: {}", sum);
}
