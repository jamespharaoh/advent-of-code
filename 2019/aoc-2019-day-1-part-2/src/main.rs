use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

fn main () {
	let mut sum: u64 = 0;
	let file = File::open ("input").unwrap ();
	let reader = BufReader::new (file);
	for line in reader.lines () {
		let mut line: u64 = line.unwrap ().parse ().unwrap ();
		loop {
			line /= 3;
			if line <= 2 { break; }
			line -= 2;
			sum += line;
		}
	}
	println! ("Total fuel: {}", sum);
}
