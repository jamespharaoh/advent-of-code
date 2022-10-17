use super::*;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input <'inp> {
	pub progs: Vec <Prog <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { progs, params } = [ params, @lines progs ]
}

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Prog <'inp> {
	pub name: InpStr <'inp>,
	pub weight: u32,
	pub holds: Vec <InpStr <'inp>>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Prog <'inp> { name, weight, holds } = [
		@str name = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 8), " ", @confirm,
		"(", weight, ")",
		holds {
			type = Vec <_>;
			holds if (! holds.is_empty ()) = [
				" -> ", @delim ", " holds { hold = [
					@str hold = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 8),
				] },
			],
			holds = [ @parse holds { Vec::new () } ],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
