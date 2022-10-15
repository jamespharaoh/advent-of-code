use super::*;

pub type Val = u16;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Step {
		Input { val: Val, bot: Val } = [
			"value ", val, " goes to bot ", bot,
		],
		Give { bot: Val, low: Target, high: Target } = [
			"bot ", bot, " gives low to ", low, " and high to ", high,
		],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Target {
		Bot (bot: Val) = [ "bot ", bot ],
		Output (out: Val) = [ "output ", out ],
	}
}
