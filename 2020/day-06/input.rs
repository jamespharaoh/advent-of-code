use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub groups: Vec <InputGroup <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { groups, params } = [ params, @delim "\n\n" groups ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug) ]
	pub struct InputGroup <'inp> {
		pub people: Vec <InputPerson <'inp>>,
	}
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputGroup <'inp> { people } = [ @lines people ]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug) ]
	pub struct InputPerson <'inp> {
		pub answers: InpStr <'inp>,
	}
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputPerson <'inp> { answers } = [ @str answers = ('a' ..= 'z', 1 ..= 26) ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
