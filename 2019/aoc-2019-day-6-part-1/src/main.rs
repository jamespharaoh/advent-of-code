use std::collections::HashMap;
use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

fn main () {

	let file = File::open ("input").unwrap ();
	let reader = BufReader::new (file);
	let parent_map: HashMap <String, String> = reader.lines ().map (
		|line| {
			let line = line.unwrap ();
			let parts: Vec <String> = line.split (')').map (str::to_owned).collect ();
			(parts [1].to_owned (), parts [0].to_owned ())
		},
	).collect ();

	let mut sum: u64 = 0;
	for (start, _) in parent_map.iter () {
		let mut current = start.to_owned ();
		while current != "COM" {
			sum += 1;
			current = parent_map [& current].to_owned ();
		}
	}

	println! ("Checksum: {}", sum);

}
