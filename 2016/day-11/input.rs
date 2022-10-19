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

struct_parser_display! {
	input_lifetime = 'inp;
	Floor <'inp> { components } = [
		components {
			type = Vec <Component <'inp>>;
			components if (components.is_empty ()) = [
				"nothing relevant",
				@parse components { Vec::new () },
			],
			components if (2 < components.len ()) = [
				@display {
					let components_last = components.last ().unwrap ();
					let components_main = & components [ .. components.len () - 1];
				},
				"a ", @delim ", a " components_main, ", and a ", components_last,
				@parse {
					let mut components: Vec <_> = components_main;
					components.push (components_last);
				},
			],
			components = [ "a ", @delim " and a " components ],
		},
	]
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
