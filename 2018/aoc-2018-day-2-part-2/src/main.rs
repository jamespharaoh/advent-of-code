use std::error::Error;
use std::fs;

fn main () -> Result <(), Box::<dyn Error>> {
	let input_string = fs::read_to_string ("input") ?;
	let lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
	for index_0 in 0 .. lines.len () - 2 {
		let line_0 = lines [index_0];
		for index_1 in index_0 + 1 .. lines.len () - 1 {
			let line_1 = lines [index_1];
			let num_different: u64 = Iterator::zip (line_0.chars (), line_1.chars ()).filter (
				|(ch_0, ch_1)| ch_0 != ch_1
			).count () as u64;
			if num_different != 1 { continue }
			let common_chars: String = Iterator::zip (line_0.chars (), line_1.chars ()).filter (
				|(ch_0, ch_1)| ch_0 == ch_1
			).map (
				|(ch, _)| ch
			).collect ();
			println! ("First box ID:      {}", line_0);
			println! ("Second box ID:     {}", line_1);
			println! ("Common characters: {}", common_chars);
			return Ok (())
		}
	}
	panic! ();
}
