use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub cards: Vec <InputCard>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { cards, params } = [ params, @lines cards ]
}

#[ derive (Clone, Debug) ]
pub struct InputCard {
	pub id: u64,
	pub winning: Vec <u8>,
	pub selected: Vec <u8>,
}

struct_parser_display! {
	InputCard { id, winning, selected } = [
		"Card ", @skip, id, ": ",
		@delim_some " " winning { num = [ @skip, num ] }, " | ",
		@delim_some " " selected { num = [ @skip, num ] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
