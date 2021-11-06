use chrono::NaiveDateTime;
use itertools::Itertools as _;
use parse_display_derive::FromStr;
use std::error::Error;
use std::slice::Iter;

#[ derive (Clone, Copy, FromStr) ]
#[ display ("[{time}] {event}") ]
#[ from_str (new = Ok::<_, Box <dyn Error>> (()).and_then (|_| Ok (Line {
	time: NaiveDateTime::parse_from_str (String::as_str (& time), "%Y-%m-%d %H:%M") ?,
	event,
}))) ]
pub struct Line {
	pub time: NaiveDateTime,
	pub event: Event,
}

#[ derive (Clone, Copy, FromStr) ]
pub enum Event {
	#[display ("Guard #{0} begins shift")] BeginsShift (u32),
	#[display ("falls asleep")] FallsAsleep,
	#[display ("wakes up")] WakesUp,
}

pub struct LineIter <'a> {
	iter: Iter <'a, Line>,
	state: LineIterState,
}

impl <'a> LineIter <'a> {
	pub fn new (lines: & 'a [Line]) -> LineIter <'a> {
		LineIter {
			iter: lines.iter (),
			state: LineIterState::None,
		}
	}
}

impl <'a> Iterator for LineIter <'a> {
	type Item = (u32, NaiveDateTime, NaiveDateTime);
	fn next (& mut self) -> Option <Self::Item> {
		loop {
			let line = match self.iter.next () {
				Some (line) => line,
				None => return None,
			};
			match (self.state, line.event) {
				(_, Event::BeginsShift (guard_id)) => {
					self.state = LineIterState::Awake (guard_id);
				},
				(LineIterState::Awake (guard_id), Event::FallsAsleep) => {
					self.state = LineIterState::Asleep (guard_id, line.time);
				},
				(LineIterState::Asleep (guard_id, sleep_time), Event::WakesUp) => {
					self.state = LineIterState::Awake (guard_id);
					return Some ((guard_id, sleep_time, line.time));
				},
				_ => panic! (),
			};
		}
	}
}

#[ derive (Clone, Copy) ]
enum LineIterState {
	None,
	Awake (u32),
	Asleep (u32, NaiveDateTime)
}

pub fn parse_lines (input: & str) -> Result <Vec <Line>, Box <dyn Error>> {
	Ok (
		input.trim ().split ("\n")
			.map (|line| Ok::<Line, Box <dyn Error>> (line.parse () ?))
			.collect::<Result <Vec <_>, _>> () ?
			.into_iter ().sorted_by_key (|line| line.time).collect ()
	)
}
