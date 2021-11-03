use std::collections::HashMap;
use std::error::Error;
use std::fs;
use text_io::try_scan;

fn main () -> Result <(), Box::<dyn Error>> {
	let input_string = fs::read_to_string ("input") ?;
	let input_lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
	let mut usage: HashMap <(u64, u64), u64> = HashMap::new ();
	for line in input_lines.iter () {
		let id: u64;
		let (left, top, width, height): (u64, u64, u64, u64);
		try_scan! (line.bytes () => "#{} @ {},{}: {}x{}", id, left, top, width, height);
		for x in left .. left + width {
			for y in top .. top + height {
				(* usage.entry ((x, y)).or_insert (0)) += 1;
			}
		}
	}
	'LINES: for line in input_lines {
		let id: u64;
		let (left, top, width, height): (u64, u64, u64, u64);
		try_scan! (line.bytes () => "#{} @ {},{}: {}x{}", id, left, top, width, height);
		for x in left .. left + width {
			for y in top .. top + height {
				if usage [& (x, y)] > 1 { continue 'LINES }
			}
		}
		println! ("Non overlapping ID: {}", id);
		return Ok (());
	}
	panic! ();
}
