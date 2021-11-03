use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Timelike as _;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main () -> Result <(), Box <dyn Error>> {

	let line_regex = Regex::new (r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.+)$").unwrap ();

	let input_string = fs::read_to_string ("input") ?;
	let mut input_lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
	input_lines.sort ();
	let input_lines = input_lines;

	let mut guard_sleep_total: HashMap <u32, i32> = HashMap::new ();
	let mut guard_sleep_mins: HashMap <u32, HashMap <u32, u32>> = HashMap::new ();
	let mut guard_id: Option <u32> = None;
	let mut awake: bool = true;
	let mut sleep_time: Option <NaiveDateTime> = None;
	for line in input_lines.iter ().cloned () {
		let captures = line_regex.captures (line).ok_or (
			format! ("Invalid line format: {}", line),
		) ?;
		let year: i32 = captures.get (1).unwrap ().as_str ().parse () ?;
		let month: u32 = captures.get (2).unwrap ().as_str ().parse () ?;
		let day: u32 = captures.get (3).unwrap ().as_str ().parse () ?;
		let hour: u32 = captures.get (4).unwrap ().as_str ().parse () ?;
		let minute: u32 = captures.get (5).unwrap ().as_str ().parse () ?;
		let time = NaiveDateTime::new (
			NaiveDate::from_ymd (year, month, day),
			NaiveTime::from_hms (hour, minute, 0),
		);
		let text = captures.get (6).unwrap ().as_str ();
		if text.starts_with ("Guard #") {
			guard_id = Some (text [7 .. text.len () - 13].parse () ?);
		} else if text == "falls asleep" {
			if ! awake { panic! () }
			awake = false;
			sleep_time = Some (time);
		} else if text == "wakes up" {
			if awake { panic! () }
			(* guard_sleep_total.entry (guard_id.unwrap ()).or_insert (0)) +=
				(time - sleep_time.unwrap ()).num_minutes () as i32;
			let guard_sleep_mins =
				guard_sleep_mins.entry (guard_id.unwrap ()).or_insert (HashMap::new ());
			let mut cur_time = sleep_time.unwrap ();
			while cur_time < time {
				(* guard_sleep_mins.entry (cur_time.time ().minute ()).or_insert (0)) += 1;
				cur_time += Duration::minutes (1);
			}
			awake = true;
			sleep_time = None;
		} else {
			panic! ("Unexpected text: {}", text);
		}
	}
	let (sleepiest_id, _) = guard_sleep_total.iter ().max_by_key (|(_, mins)| * mins).unwrap ();
	let (sleepiest_min, _) = guard_sleep_mins [& sleepiest_id].iter ().max_by_key (
		|(_, num)| * num,
	).unwrap ();
	println! ("Sleepiest guard id: {}", sleepiest_id);
	println! ("Sleepiest time: 00:{:02}", sleepiest_min);
	Ok (())
}
