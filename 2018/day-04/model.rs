use super::*;

#[ derive (Clone, Copy, Debug) ]
pub struct Entry {
	pub date: Date,
	pub time: HourMinute,
	pub event: Event,
}

impl <'inp> FromParser <'inp> for Entry {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, "[", date, " ", time, "] ", event);
		Ok (Self { date, time, event })
	}
}

impl Display for Entry {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"[{date} {time}] {event}",
			date = self.date,
			time = self.time,
			event = self.event,
		) ?;
		Ok (())
	}
}

#[ derive (Clone, Copy, Debug) ]
pub enum Event {
	BeginsShift (u32),
	FallsAsleep,
	WakesUp,
}

impl Display for Event {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::BeginsShift (id) => write! (formatter, "Guard #{} begins shift", id) ?,
			Self::FallsAsleep => write! (formatter, "falls asleep") ?,
			Self::WakesUp => write! (formatter, "wakes up") ?,
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Event {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| {
				parse! (parser, "Guard #", id, " begins shift");
				Ok (Self::BeginsShift (id))
			})
			.of (|parser| { parse! (parser, "falls asleep"); Ok (Self::FallsAsleep) })
			.of (|parser| { parse! (parser, "wakes up"); Ok (Self::WakesUp) })
			.done ()
	}
}

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Date {
	year: u16,
	month: u8,
	day: u8,
}

impl Date {

	#[ inline ]
	#[ must_use ]
	pub fn next (self) -> Self {
		if self.day < Self::days_in_month (self.year, self.month) {
			Self { day: self.day + 1, .. self }
		} else if self.month < 12 {
			Self { month: self.month + 1, day: 1, .. self }
		} else {
			Self { year: self.year + 1, month: 1, day: 1 }
		}
	}

	fn days_in_month (year: u16, month: u8) -> u8 {
		match (year, month) {
			(_, 1 | 3 | 5 | 7 | 8 | 10 | 12) => 31,
			(_, 4 | 6 | 9 | 11) => 30,
			(year, 2) if year % 400 == 0 => 29,
			(year, 2) if year % 100 == 0 => 28,
			(year, 2) if year % 4 == 0 => 29,
			(_, 2) => 28,
			_ => unreachable! (),
		}
	}

}

impl Display for Date {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{year:04}-{month:02}-{day:02}",
			year = self.year,
			month = self.month,
			day = self.day,
		) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Date {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, (year = 1 ..= 9999), "-", (month = 1 ..= 12), "-");
		let max_day = match (year, month) {
			(_, 1 | 3 | 5 | 7 | 8 | 10 | 12) => 31,
			(_, 4 | 6 | 9 | 11) => 30,
			(year, 2) if year % 400 == 0 => 29,
			(year, 2) if year % 100 == 0 => 28,
			(year, 2) if year % 4 == 0 => 29,
			(_, 2) => 28,
			_ => unreachable! (),
		};
		parse! (parser, (day = 1 ..= max_day));
		Ok (Self { year, month, day })
	}
}

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct HourMinute {
	hour: u8,
	minute: u8,
}

impl HourMinute {

	#[ inline ]
	#[ must_use ]
	pub fn hour (& self) -> u32 {
		self.hour.as_u32 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn minute (& self) -> u32 {
		self.minute.as_u32 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn as_minutes (& self) -> u32 {
		self.hour.as_u32 () * 60 + self.minute.as_u32 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn as_seconds (& self) -> u32 {
		self.hour.as_u32 () * 3600 + self.minute.as_u32 () * 60
	}

}

impl Display for HourMinute {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{hour:02}:{minute:02}",
			hour = self.hour,
			minute = self.minute,
		) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for HourMinute {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, (hour = 0 ..= 23), ":", (minute = 0 ..= 59));
		Ok (Self { hour, minute })
	}
}
