use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Timelike as _;
use itertools::Itertools as _;
use parse_display_derive::FromStr;
use std::collections::HashMap;
use std::error::Error;

pub fn aoc2018_day4_part1 (input: & str) -> Result <(), Box <dyn Error>> {

	let input_lines: Vec <Line> = input.trim ().split ("\n")
		.map (|line| Ok::<Line, Box <dyn Error>> (line.parse () ?))
		.collect::<Result <Vec <_>, _>> () ?
		.into_iter ().sorted_by_key (|line| line.time).collect ();

	let mut guard_total_mins: HashMap <u32, i32> = HashMap::new ();
	let mut guard_sleep_mins: HashMap <u32, HashMap <u32, u32>> = HashMap::new ();
	enum State { None, Awake (u32), Asleep (u32, NaiveDateTime) }
	let mut state = State::None;
	for line in input_lines.iter () {
		state = match (state, line.event) {
			(_, Event::BeginsShift (guard_id)) => State::Awake (guard_id),
			(State::Awake (guard_id), Event::FallsAsleep) => State::Asleep (guard_id, line.time),
			(State::Asleep (guard_id, sleep_time), Event::WakesUp) => {
				let diff_mins = (line.time - sleep_time).num_minutes () as i32;
				(* guard_total_mins.entry (guard_id).or_insert (0)) += diff_mins;
				let sleep_mins = guard_sleep_mins.entry (guard_id).or_insert (HashMap::new ());
				for time in itertools::iterate (sleep_time, |time| * time + Duration::minutes (1))
						.take_while (|time| * time < line.time) {
					(* sleep_mins.entry (time.time ().minute ()).or_insert (0)) += 1;
				}
				State::Awake (guard_id)
			},
			_ => panic! (),
		};
	}
	let (sleepiest_id, _) = guard_total_mins.iter ().max_by_key (|(_, mins)| * mins).unwrap ();
	let (sleepiest_min, _) = guard_sleep_mins [& sleepiest_id].iter ().max_by_key (
		|(_, num)| * num,
	).unwrap ();
	println! ("Sleepiest guard id: {}", sleepiest_id);
	println! ("Sleepiest time: 00:{:02}", sleepiest_min);
	println! ("Puzzle anser: {}", sleepiest_id * sleepiest_min);
	Ok (())
}

#[ derive (Clone, Copy, FromStr) ]
#[ display ("[{time}] {event}") ]
#[ from_str (new = Ok::<_, Box <dyn Error>> (()).and_then (|_| Ok (Line {
	time: NaiveDateTime::parse_from_str (String::as_str (& time), "%Y-%m-%d %H:%M") ?,
	event,
}))) ]
struct Line {
	time: NaiveDateTime,
	event: Event,
}

#[ derive (Clone, Copy, FromStr) ]
enum Event {
	#[display ("Guard #{0} begins shift")] BeginsShift (u32),
	#[display ("falls asleep")] FallsAsleep,
	#[display ("wakes up")] WakesUp,
}
