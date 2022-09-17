use super::*;

use model::Passport;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub passports: Vec <Passport <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { passports, params } = [ params, @delim "\n" passports ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
