use std::fs;

fn main () {
	let output_string = fs::read_to_string ("input").unwrap ();
	let output_lines: Vec <& str> = output_string.trim ().split ('\n').collect ();
	let start_time: u128 = output_lines [0].parse ().unwrap ();
	let bus_idx_ids: Vec <(u128, u128)> = output_lines [1].split (',').enumerate ().filter (
		|(_, bus_id_str)| * bus_id_str != "x",
	).map (
		|(bus_idx, bus_id_str)| (bus_idx as u128, bus_id_str.parse ().unwrap ()),
	).collect ();
	let mut time: u128 = 0;
	loop {
		let mut incr: u128 = 1;
		let mut matches = true;
		for (bus_idx, bus_id) in bus_idx_ids.iter () {
			if (time + bus_idx) % bus_id == 0 {
				incr *= bus_id;
			} else {
				matches = false;
			}
		}
		if matches { break }
		time += incr;
	}
	println! ("TIme: {}", time);
}
