use std::collections::HashMap;
use std::error::Error;
use text_io::try_scan;

pub fn aoc2018_day3_part2 (input: & str) -> Result <(), Box <dyn Error>> {
	let mut usage: HashMap <(u64, u64), u64> = HashMap::new ();
	for line in input.trim ().split ("\n") {
		let id: u64;
		let (left, top, width, height): (u64, u64, u64, u64);
		try_scan! (line.bytes () => "#{} @ {},{}: {}x{}", id, left, top, width, height);
		for x in left .. left + width {
			for y in top .. top + height {
				(* usage.entry ((x, y)).or_insert (0)) += 1;
			}
		}
	}
	'LINES: for line in input.trim ().split ("\n") {
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
