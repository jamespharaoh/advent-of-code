use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub games: Vec <InputGame>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { games, params } = [ params, @lines games ]
}

#[ derive (Clone, Debug) ]
pub struct InputGame {
	pub id: u32,
	pub rounds: Vec <InputRound>,
}

struct_parser_display! {
	InputGame { id, rounds } = [ "Game ", id, ": ", @delim "; " rounds ]
}

#[ derive (Clone, Debug) ]
pub struct InputRound {
	pub draws: Vec <InputDraw>,
}

struct_parser_display! {
	InputRound { draws } = [ @delim ", " draws ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct InputDraw {
	pub colour: InputColour,
	pub num: u32,
}

struct_parser_display! {
	InputDraw { colour, num } = [ num, " ", colour ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum InputColour {
		Red = [ "red" ],
		Green = [ "green" ],
		Blue = [ "blue" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
