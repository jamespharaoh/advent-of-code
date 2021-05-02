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

	let mut you_chain = chain (& parent_map, & "YOU".to_owned ());
	let mut san_chain = chain (& parent_map, & "SAN".to_owned ());

	loop {
		if you_chain.len () == 0 || san_chain.len () == 0 {
			break;
		}
		if you_chain.last ().unwrap () != san_chain.last ().unwrap () {
			break;
		}
		you_chain.pop ().unwrap ();
		san_chain.pop ().unwrap ();
	}

	println! ("Transfers: {}", you_chain.len () + san_chain.len () - 2);

}

fn chain (parent_map: & HashMap <String, String>, key: & String) -> Vec <String> {
	let mut result: Vec <String> = Vec::new ();
	let mut current = key;
	while current != "COM" {
		result.push (current.clone ());
		current = & parent_map [current];
	}
	result
}
