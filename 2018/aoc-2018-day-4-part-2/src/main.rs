use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Timelike as _;
use itertools::Itertools as _;
use parse_display_derive::FromStr;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main () -> Result <(), Box <dyn Error>> {

	let input_string = fs::read_to_string ("input") ?;
	let input_lines: Vec <Line> = input_string.trim ().split ("\n")
		.map (|line| Ok::<Line, Box <dyn Error>> (line.parse () ?))
		.collect::<Result <Vec <_>, _>> () ?
		.into_iter ().sorted_by_key (|line| line.time).collect ();

	let mut sleep_mins: HashMap <(u32, u32), u32> = HashMap::new ();
	enum State { None, Awake (u32), Asleep (u32, NaiveDateTime) }
	let mut state = State::None;
	for line in input_lines.iter ().cloned () {
		state = match (state, line.event) {
			(_, Event::BeginsShift (guard_id)) => State::Awake (guard_id),
			(State::Awake (guard_id), Event::FallsAsleep) => State::Asleep (guard_id, line.time),
			(State::Asleep (guard_id, sleep_time), Event::WakesUp) => {
				for time in itertools::iterate (sleep_time, |time| * time + Duration::minutes (1))
						.take_while (|time| * time < line.time) {
					(* sleep_mins.entry ((guard_id, time.time ().minute ())).or_insert (0)) += 1;
				}
				State::Awake (guard_id)
			},
			_ => panic! (),
		};
	}

	let ((guard_id, minute), _) = sleep_mins.iter ().max_by_key (|(_, num)| * num).unwrap ();
	println! ("Guard id: {}", guard_id);
	println! ("Time: 00:{:02}", minute);
	println! ("Puzzle answer: {}", guard_id * minute);

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
