use super::*;

#[ derive (Clone, Debug) ]
pub struct Reindeer <'inp> {
	pub name: InpStr <'inp>,
	pub fly_speed: u32,
	pub fly_time: u32,
	pub rest_time: u32,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Reindeer <'inp> { name, fly_speed, fly_time, rest_time } = [
		@str name = (|ch| { ch.is_ascii_alphabetic () }, 1 .. 20), " can fly ",
		fly_speed, " km/s for ",
		fly_time, " seconds, but then must rest for ",
		rest_time, " seconds.",
	]
}
