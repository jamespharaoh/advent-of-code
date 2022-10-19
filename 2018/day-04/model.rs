use super::*;

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct Date {
	year: u16,
	month: u8,
	day: u8,
}

struct_parser_display! {
	Date { year, month, day } = [
		year = 1 ..= 9999, "-", month = 1 ..= 12, "-", day = 1 ..= 31,
		@parse |parser| {
			if Date::days_in_month (year, month) < day {
				return Err (parser.err ())
			}
		}
	]
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

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
pub struct HourMinute {
	hour: u8,
	minute: u8,
}

struct_parser_display! {
	HourMinute { hour, minute } = [ hour = 0 ..= 23, ":", minute = 0 ..= 59 ]
}

impl HourMinute {

	#[ inline ]
	#[ must_use ]
	pub fn hour (& self) -> u32 {
		self.hour.pan_u32 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn minute (& self) -> u32 {
		self.minute.pan_u32 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn as_minutes (& self) -> u32 {
		self.hour.pan_u32 () * 60 + self.minute.pan_u32 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn as_seconds (& self) -> u32 {
		self.hour.pan_u32 () * 3600 + self.minute.pan_u32 () * 60
	}

}
