use std::collections::HashMap;
use std::error::Error;
use std::fs;
use text_io::try_scan;

fn main () -> Result <(), Box::<dyn Error>> {
	let input_string = fs::read_to_string ("input") ?;
	let input_lines = input_string.trim ().split ("\n");
	let mut usage: HashMap <(u64, u64), u64> = HashMap::new ();
	for line in input_lines {
		let id: u64;
		let (left, top, width, height): (u64, u64, u64, u64);
		try_scan! (line.bytes () => "#{} @ {},{}: {}x{}", id, left, top, width, height);
		for x in left .. left + width {
			for y in top .. top + height {
				(* usage.entry ((x, y)).or_insert (0)) += 1;
			}
		}
	}
	let num_common = usage.into_values ().filter (|num| * num > 1).count ();
	println! ("Number of common squares: {}", num_common);
	Ok (())
}
