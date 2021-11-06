use chrono::Duration;
use chrono::Timelike as _;
use std::collections::HashMap;
use std::error::Error;

use crate::day4::shared;
use crate::day4::shared::LineIter;

pub fn aoc2018_day4_part1 (input: & str) -> Result <(), Box <dyn Error>> {

	let lines = shared::parse_lines (input) ?;

	let mut guard_total_mins: HashMap <u32, i32> = HashMap::new ();
	let mut guard_sleep_mins: HashMap <u32, HashMap <u32, u32>> = HashMap::new ();
	for (guard_id, sleep_time, wake_time) in LineIter::new (& lines) {
		let diff_mins = (wake_time - sleep_time).num_minutes () as i32;
		(* guard_total_mins.entry (guard_id).or_insert (0)) += diff_mins;
		let sleep_mins = guard_sleep_mins.entry (guard_id).or_insert (HashMap::new ());
		for time in itertools::iterate (sleep_time, |time| * time + Duration::minutes (1))
				.take_while (|time| * time < wake_time) {
			(* sleep_mins.entry (time.time ().minute ()).or_insert (0)) += 1;
		}
	}

	let (sleepiest_id, _) = guard_total_mins.iter ().max_by_key (|(_, mins)| * mins).unwrap ();
	let (sleepiest_min, _) = guard_sleep_mins [& sleepiest_id].iter ().max_by_key (
		|(_, num)| * num,
	).unwrap ();

	println! ("Guard id: {}", sleepiest_id);
	println! ("Time: 00:{:02}", sleepiest_min);
	println! ("Puzzle anser: {}", sleepiest_id * sleepiest_min);

	Ok (())

}
