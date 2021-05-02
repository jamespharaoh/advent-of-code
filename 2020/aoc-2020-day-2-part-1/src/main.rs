use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input_data: Vec <(char, u64, u64, String)> = input_lines.into_iter ().map (
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
			let policy_min: u64 = policy_limits_parts [0].parse ().unwrap ();
			let policy_max: u64 = policy_limits_parts [1].parse ().unwrap ();
			if policy_char_str.len () != 1 { panic! () }
			let policy_char = policy_char_str.chars ().next ().unwrap ();
			(policy_char, policy_min, policy_max, password_str.to_string ())
		},
	).collect ();
	let num_valid = input_data.iter ().filter (
		|(policy_char, policy_min, policy_max, password)| {
			let num_found = password.chars ().filter (|ch| * ch == * policy_char).count () as u64;
			num_found >= * policy_min && num_found <= * policy_max
		},
	).count ();
	println! ("Number of valid passwords: {}", num_valid);
}
