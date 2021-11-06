use chrono::Duration;
use chrono::Timelike as _;
use std::collections::HashMap;
use std::error::Error;

use crate::day4::shared;
use crate::day4::shared::LineIter;

pub fn aoc2018_day4_part2 (input: & str) -> Result <String, Box <dyn Error>> {

	let lines = shared::parse_lines (input) ?;

	let mut sleep_mins: HashMap <(u32, u32), u32> = HashMap::new ();
	for (guard_id, sleep_time, wake_time) in LineIter::new (& lines) {
		for time in itertools::iterate (sleep_time, |time| * time + Duration::minutes (1))
				.take_while (|time| * time < wake_time) {
			(* sleep_mins.entry ((guard_id, time.time ().minute ())).or_insert (0)) += 1;
		}
	}

	let ((guard_id, minute), _) = sleep_mins.iter ().max_by_key (|(_, num)| * num).unwrap ();

	println! ("Guard id: {}", guard_id);
	println! ("Time: 00:{:02}", minute);

	Ok (format! ("{}", guard_id * minute))

}
