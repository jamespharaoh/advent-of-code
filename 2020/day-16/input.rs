use super::*;

use model::Field;
use model::Ticket;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub fields: Vec <Field <'inp>>,
	pub your_ticket: Ticket,
	pub nearby_tickets: Vec <Ticket>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { fields, your_ticket, nearby_tickets, params } = [
		params,
		@lines fields, "\n",
		"\n",
		"your ticket:\n",
		your_ticket, "\n",
		"\n",
		"nearby tickets:\n",
		@lines nearby_tickets,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
