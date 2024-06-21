use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub rows: Vec <InputRow>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { rows, params } = [ params, @lines rows ]
}

#[ derive (Clone, Debug) ]
pub struct InputRow {
	pub springs: Vec <Spring>,
	pub groups: Vec <u32>,
}

struct_parser_display! {
	InputRow { springs, groups } = [ @collect_some springs, " ", @delim_some "," groups ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Spring {
		Operational = [ "." ],
		Damaged = [ "#" ],
		Unknown = [ "?" ],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
