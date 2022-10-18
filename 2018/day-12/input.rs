use super::*;
use model::Pot;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub start: Vec <Pot>,
	pub rules: Vec <InputRule>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { start, rules, params } = [
		params,
		"initial state: ", @collect start, "\n",
		"\n",
		@lines rules,
	]
}

#[ derive (Clone, Copy, Debug) ]
pub struct InputRule {
	pub from: [Pot; 5],
	pub to: Pot,
}

struct_parser_display! {
	InputRule { from: [ from_0, from_1, from_2, from_3, from_4 ], to } = [
		from_0, from_1, from_2, from_3, from_4, " => ", to,
	]
}

input_params! {
	#[ derive (Clone, Copy, Debug) ]
	pub struct InputParams {
		pub check_rules: bool = ("CHECK_RULES=", true, .. ),
	}
}
