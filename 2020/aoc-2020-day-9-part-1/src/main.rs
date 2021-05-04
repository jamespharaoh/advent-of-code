use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input: Vec <u64> = input_lines.iter ().map (|line| line.parse ().unwrap ()).collect ();
	for input_idx in 25 .. input.len () {
		let prev = & input [input_idx - 25 .. input_idx];
		let mut found = false;
		for prev_idx_0 in 0 .. prev.len () - 1 {
			for prev_idx_1 in prev_idx_0 + 1 .. prev.len () {
				if input [input_idx] == prev [prev_idx_0] + prev [prev_idx_1] {
					found = true;
				}
			}
		}
		if ! found {
			println! ("First number which isn't a sum: {}", input [input_idx]);
			return;
		}
	}
}
