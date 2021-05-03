use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let seat_ids: Vec <u32> = input_lines.iter ().map (
		|input_line| input_line.chars ().fold (0, |mut seat_id, ch| {
			seat_id <<= 1;
			if ch == 'B' || ch == 'R' {
				seat_id |= 1;
			}
			seat_id
		}),
	).collect ();
	let max_seat_id = seat_ids.iter ().max ().unwrap ();
	println! ("Highest seat id: {}", max_seat_id);
}
