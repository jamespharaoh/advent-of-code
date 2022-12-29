use super::*;

#[ derive (Clone, Debug) ]
pub struct PassPolicy <'inp> {
	pub num_0: u32,
	pub num_1: u32,
	pub ch: char,
	pub password: InpStr <'inp>,
}

struct_parser! {
	input_lifetime = 'inp;
	#[ allow (clippy::manual_is_ascii_check) ]
	PassPolicy <'inp> { num_0, num_1, ch, password } = [
		num_0 = 1 .. , "-",
		num_1 = 1 .. , " ",
		ch = 'a' ..= 'z', ": ",
		@str password = ('a' ..= 'z', 1 .. ),
	]
}

struct_display! {
	input_lifetime = 'inp;
	PassPolicy <'inp> { num_0, num_1, ch, password } = [
		num_0 = 1 .. , "-",
		num_1 = 1 .. , " ",
		ch = 'a' ..= 'z', ": ",
		@str password = ('a' ..= 'z', 1 .. ),
	]
}
