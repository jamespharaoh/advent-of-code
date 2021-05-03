use std::collections::HashSet;
use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let seat_ids: HashSet <u32> = input_lines.iter ().map (
		|input_line| input_line.chars ().fold (0, |mut seat_id, ch| {
			seat_id <<= 1;
			if ch == 'B' || ch == 'R' {
				seat_id |= 1;
			}
			seat_id
		}),
	).collect ();
	let min_seat_id = seat_ids.iter ().cloned ().min ().unwrap ();
	let max_seat_id = seat_ids.iter ().cloned ().max ().unwrap ();
	let missing_seat_ids: Vec <u32> = (min_seat_id ..= max_seat_id).filter (
		|seat_id| ! seat_ids.contains (seat_id),
	).collect ();
	if missing_seat_ids.len () != 1 { panic! () }
	println! ("Missing seat ID: {}", missing_seat_ids [0]);
}
