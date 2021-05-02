use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input_data: Vec <(char, usize, usize, String)> = input_lines.into_iter ().map (
		|input_line| {
			let line_parts: Vec <& str> = input_line.split (": ").collect ();
			if line_parts.len () != 2 { panic! () }
			let policy_str = line_parts [0];
			let password_str = line_parts [1];
			let policy_parts: Vec <& str> = policy_str.split (' ').collect ();
			if policy_parts.len () != 2 { panic! () }
			let policy_limits_str = policy_parts [0];
			let policy_char_str = policy_parts [1];
			let policy_limits_parts: Vec <& str> = policy_limits_str.split ('-').collect ();
			if policy_limits_parts.len () != 2 { panic! () }
			let policy_idx_0 = policy_limits_parts [0].parse::<usize> ().unwrap () - 1;
			let policy_idx_1 = policy_limits_parts [1].parse::<usize> ().unwrap () - 1;
			if policy_char_str.len () != 1 { panic! () }
			let policy_char = policy_char_str.chars ().next ().unwrap ();
			(policy_char, policy_idx_0, policy_idx_1, password_str.to_string ())
		},
	).collect ();
	let num_valid = input_data.iter ().filter (
		|(policy_char, policy_idx_0, policy_idx_1, password)| {
			let mut num_valid: u64 = 0;
			if password.chars ().skip (* policy_idx_0).next ().unwrap () == * policy_char {
				num_valid += 1;
			}
			if password.chars ().skip (* policy_idx_1).next ().unwrap () == * policy_char {
				num_valid += 1;
			}
			num_valid == 1
		},
	).count ();
	println! ("Number of valid passwords: {}", num_valid);
}
