use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input: Vec <u64> = input_lines.iter ().map (|line| line.parse ().unwrap ()).collect ();
	let mut part_one: Option <u64> = None;
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
			part_one = Some (input [input_idx]);
			break;
		}
	}
	let part_one = part_one.unwrap ();
	println! ("First number which isn't a sum: {}", part_one);
	let mut part_two: Option <(usize, usize)> = None;
	for input_idx_0 in 0 .. input.len () - 2 {
		for input_idx_1 in input_idx_0 + 2 .. input.len () {
			if input [input_idx_0 .. input_idx_1].iter ().sum::<u64> () == part_one {
				part_two = Some ((input_idx_0, input_idx_1));
			}
		}
	}
	let part_two = part_two.unwrap ();
	println! ("Range which sums to this value: {} to {}", part_two.0, part_two.1);
	let part_two = & input [part_two.0 .. part_two.1];
	let smallest = part_two.iter ().min ().unwrap ();
	println! ("Smallest number in this range: {}", smallest);
	let largest = part_two.iter ().max ().unwrap ();
	println! ("Largest number in this range: {}", largest);
	println! ("Sum of those two: {}", smallest + largest);
}
