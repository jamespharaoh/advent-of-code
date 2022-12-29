use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub lines: Vec <InputLine>,
	pub pile_nums: Vec <u8>,
	pub steps: Vec <InputStep>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { lines, pile_nums, steps, params } = [
		params,
		@lines lines, "\n",
		@delim " " pile_nums { num = [ " ", num, " " ] }, "\n",
		"\n",
		@lines steps,
	]
}

#[ derive (Clone, Debug) ]
pub struct InputLine {
	pub crates: Vec <Option <char>>,
}

struct_parser_display! {
	InputLine { crates } = [
		@delim_some " " crates {
			type = Option <char>;
			Some (ch) = [ "[", ch, "]" ],
			None = [ "   " ],
		},
	]
}

#[ derive (Clone, Debug) ]
pub struct InputStep {
	pub num: u8,
	pub from: u8,
	pub to: u8,
}

struct_parser_display! {
	InputStep { num, from, to } = [ "move ", num, " from ", from, " to ", to ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
