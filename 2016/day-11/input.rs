//! Data structures to model the puzzle input

use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub floors: [Floor <'inp>; 4],
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { floors: [ floor_0, floor_1, floor_2, floor_3 ], params } = [
		params,
		"The first floor contains ", floor_0, ".\n",
		"The second floor contains ", floor_1, ".\n",
		"The third floor contains ", floor_2, ".\n",
		"The fourth floor contains ", floor_3, ".",
	]
}

wrapper_deref_mut! {
	#[ derive (Clone, Debug) ]
	pub struct Floor <'inp> {
		components: Vec <Component <'inp>>,
	}
}

impl <'inp> Display for Floor <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		if self.components.is_empty () {
			return write! (formatter, "nothing relevant");
		}
		for (component_idx, component) in self.components.iter ().enumerate () {
			if component_idx == 0 {
				write! (formatter, "a ") ?;
			} else if self.components.len () == 2 {
				write! (formatter, " and a ") ?;
			} else if component_idx < self.components.len () - 1 {
				write! (formatter, ", a ") ?;
			} else {
				write! (formatter, ", and a ") ?;
			}
			Display::fmt (component, formatter) ?;
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Floor <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let components = parser.any ()
			.of (|parser| {
				parser.expect ("nothing relevant") ?.confirm ();
				Ok (vec! [])
			})
			.of (|parser| {
				let comp = parser.expect ("a ") ?.item () ?;
				let mut comps = vec! [ comp ];
				loop {
					if parser.peek_rest ().starts_with (", and a ") {
						let comp: Component = parser.expect (", and a ") ?.item () ?;
						comps.push (comp);
						break;
					}
					let comp = parser.expect (", a ") ?.item () ?;
					comps.push (comp);
				}
				Ok (comps)
			})
			.of (|parser| {
				let comp_0 = parser.expect ("a ") ?.item () ?;
				let comp_1 = parser.expect (" and a ") ?.confirm ().item () ?;
				Ok (vec! [ comp_0, comp_1 ])
			})
			.of (|parser| {
				let comp: Component = parser.expect ("a ") ?.item () ?;
				Ok (vec! [ comp ])
			})
			.done () ?;
		Ok (Self { components })
	}
}

enum_decl_parser_display! {
	input_lifetime = 'inp;
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub enum Component <'inp> {
		Generator (name: InpStr <'inp>) = [
			@str name = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 16),
			" generator",
		],
		Microchip (name: InpStr <'inp>) = [
			@str name = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 16),
			"-compatible microchip",
		],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
