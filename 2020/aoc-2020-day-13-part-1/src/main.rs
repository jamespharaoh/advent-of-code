use std::fs;

fn main () {
	let output_string = fs::read_to_string ("input").unwrap ();
	let output_lines: Vec <& str> = output_string.trim ().split ('\n').collect ();
	let start_time: u64 = output_lines [0].parse ().unwrap ();
	let bus_ids: Vec <u64> = output_lines [1].split (',').filter (
		|bus_id_str| * bus_id_str != "x",
	).map (
		|bus_id_str| bus_id_str.parse ().unwrap (),
	).collect ();
	println! ("Start time: {}", start_time);
	println! ("Bus IDs: {:?}", bus_ids);
	let mut time_now: u64 = start_time;
	let (bus_id, bus_time) = 'OUTER: loop {
		for bus_id in bus_ids.iter ().cloned () {
			if time_now % bus_id == 0 { break 'OUTER (bus_id, time_now) }
		}
		time_now += 1;
	};
	println! ("Next bus ID: {}", bus_id);
	println! ("Next bus time: {}", bus_time);
	let wait_time = bus_time - start_time;
	println! ("Time to wait: {}", wait_time);
	println! ("Puzzle solution: {}", wait_time * bus_id);
}
