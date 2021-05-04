use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let mut input: Vec <u64> = input_lines.iter ().map (|line| line.parse ().unwrap ()).collect ();
	let input_max = input.iter ().max ().cloned ().unwrap ();
	input.push (input_max + 3);
	input.sort ();
	let diffs: Vec <u64> = input.iter ().scan (0, |last, item| {
		let diff = * item - * last;
		* last = * item;
		Some (diff)
	}).collect ();
	let num_ones = diffs.iter ().filter (|diff| ** diff == 1).count ();
	let num_threes = diffs.iter ().filter (|diff| ** diff == 3).count ();
	println! ("Number of ones: {}", num_ones);
	println! ("Number of threes: {}", num_threes);
	println! ("Product: {}", num_ones * num_threes);
}
