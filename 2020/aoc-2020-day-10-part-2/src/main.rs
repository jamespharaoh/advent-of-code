use std::fs;
use std::iter;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let mut input: Vec <u64> = input_lines.iter ().map (|line| line.parse ().unwrap ()).collect ();
	let input_max = input.iter ().max ().cloned ().unwrap ();
	input.push (0);
	input.push (input_max + 3);
	input.sort ();
	let mut combos: Vec <u64> = iter::repeat (0).take (input.len ()).collect ();
	combos [0] = 1;
	for input_idx in 0 .. input.len () {
		let num = combos [input_idx];
		if input_idx + 1 < input.len () && input [input_idx + 1] - input [input_idx] < 4 {
			combos [input_idx + 1] += num;
		}
		if input_idx + 2 < input.len () && input [input_idx + 2] - input [input_idx] < 4 {
			combos [input_idx + 2] += num;
		}
		if input_idx + 3 < input.len () && input [input_idx + 3] - input [input_idx] < 4 {
			combos [input_idx + 3] += num;
		}
	}
	println! ("Number of cominations: {}", combos.last ().unwrap ());
}
