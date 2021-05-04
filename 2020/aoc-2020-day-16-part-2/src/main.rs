use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::mem;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input_groups: Vec <& [& str]> = input_lines.split (|line| * line == "").collect ();
	if input_groups.len () != 3 { panic! (); }
	let fields: Vec <Field> = input_groups [0].iter ().map (
		|line| line.parse ().unwrap (),
	).collect ();
	println! ("Num fields: {}", fields.len ());
	let tickets: Vec <Ticket> = input_groups [2].iter ().skip (1).map (
		|line| line.split (',').map (|field_str| field_str.parse ().unwrap ()).collect (),
	).collect ();
	println! ("All tickets: {}", tickets.len ());
	let tickets: Vec <Ticket> = tickets.into_iter ().filter (
		|ticket| ticket.iter ().all (
			|value| fields.iter ().any (|field| field.contains (* value)),
		),
	).collect ();
	println! ("Valid tickets: {}", tickets.len ());
	let mut rem_fields: Vec <& Field> = fields.iter ().collect ();
	let mut rem_posns: HashSet <usize> = (0 .. fields.len ()).collect ();
	let mut field_posns: HashMap <String, usize> = HashMap::new ();
	while ! rem_fields.is_empty () {
		let mut progress = false;
		let mut rem_fields_temp: Vec <& Field> = Vec::new ();
		mem::swap (& mut rem_fields_temp, & mut rem_fields);
		for field in rem_fields_temp.iter () {
			let mut match_posns: Vec <usize> = Vec::new ();
			for pos in rem_posns.iter ().cloned () {
				if tickets.iter ().all (|ticket| field.contains (ticket [pos])) {
					match_posns.push (pos);
				}
			}
			if match_posns.len () == 0 { panic! () }
			if match_posns.len () == 1 {
				field_posns.insert (field.name.clone (), match_posns [0]);
				rem_posns.remove (& match_posns [0]);
				progress = true;
			} else {
				rem_fields.push (field);
			}
		}
		if ! progress { panic! () }
	}
	let my_ticket: Ticket = input_groups [1] [1].split (',').map (
		|value_str| value_str.parse ().unwrap (),
	).collect ();
	let mut product: u64 = 1;
	for field in fields {
		if ! field.name.starts_with ("departure ") { continue }
		let field_posn = field_posns [& field.name];
		product *= my_ticket [field_posn];
	}
	println! ("Product of my departure fields: {}", product);
}

type Ticket = Vec <u64>;

#[ derive (Debug) ]
struct Field {
	name: String,
	range_0: RangeInclusive <u64>,
	range_1: RangeInclusive <u64>,
}

impl Field {
	fn contains (& self, value: u64) -> bool {
		self.range_0.contains (& value) || self.range_1.contains (& value)
	}
}

impl FromStr for Field {
	type Err = String;
	fn from_str (source: & str) -> Result <Field, String> {
		lazy_static! {
			static ref RE: Regex = Regex::new (r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap ();
		}
		let captures = RE.captures (source).unwrap ();
		let name = captures.get (1).unwrap ().as_str ().to_string ();
		let range_0_min: u64 = captures.get (2).unwrap ().as_str ().parse ().unwrap ();
		let range_0_max: u64 = captures.get (3).unwrap ().as_str ().parse ().unwrap ();
		let range_1_min: u64 = captures.get (4).unwrap ().as_str ().parse ().unwrap ();
		let range_1_max: u64 = captures.get (5).unwrap ().as_str ().parse ().unwrap ();
		Ok (Field {
			name,
			range_0: range_0_min ..= range_0_max,
			range_1: range_1_min ..= range_1_max,
		})
	}
}
