use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.split ('\n').collect ();
	let input_values: Vec <u64> = input_lines.into_iter ().filter (
		|line| line.len () > 0,
	).map (
		|line| line.parse ().unwrap (),
	).collect ();
	for x in 0 .. input_values.len () - 2 {
		for y in x + 1 .. input_values.len () - 1 {
			for z in y + 1 .. input_values.len () {
				if input_values [x] + input_values [y] + input_values [z] == 2020 {
					println! ("First input value: {}", input_values [x]);
					println! ("Second input value: {}", input_values [y]);
					println! ("Third input value: {}", input_values [z]);
					println! ("Product of both values: {}",
						input_values [x] * input_values [y] * input_values [z]);
				}
			}
		}
	}
}
