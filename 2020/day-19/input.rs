use super::*;

use model::Rule;
use model::RuleId;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub rules: Vec <InputRule>,
	pub messages: Vec <InputMessage <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { rules, messages, params } = [
		params,
		@lines rules, "\n",
		"\n",
		@lines messages,
	]
}

#[ derive (Clone, Debug) ]
pub struct InputRule  {
	pub id: RuleId,
	pub rule: Rule,
}

struct_parser_display! {
	InputRule { id, rule } = [ id, ": ", rule ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct InputMessage <'inp> {
		message: InpStr <'inp>,
	}
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputMessage <'inp> { message } = [ @str message = (|ch| { ch.is_ascii_lowercase () }, 1 .. ) ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
