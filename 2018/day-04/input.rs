use super::*;

use model::Date;
use model::HourMinute;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub entries: Vec <Entry>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { entries, params } = [ params, @lines entries ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct Entry {
	pub date: Date,
	pub time: HourMinute,
	pub event: Event,
}

struct_parser_display! {
	Entry { date, time, event } = [ "[", date, " ", time, "] ", event ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum Event {
		BeginsShift (id: u32) = [ "Guard #", id, " begins shift" ],
		FallsAsleep = [ "falls asleep" ],
		WakesUp = [ "wakes up" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
