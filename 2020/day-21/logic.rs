//! Logic for solving the puzzles

use super::*;

use input::Input;

type Alrgns <'inp> = HashMap <InpStr <'inp>, Ingrs <'inp>>;
type Ingrs <'inp> = HashSet <InpStr <'inp>>;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let (_, ingrs) = get_alrgns_ingrs (input);
	Ok (
		input.foods.iter ()
			.flat_map (|food| & food.ingrs)
			.filter (|ingr| ! ingrs.contains (ingr))
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let (mut pending_alrgns, mut pending_ingrs) = get_alrgns_ingrs (input);
	let mut result: Vec <(InpStr, InpStr)> = Vec::new ();
	while ! pending_alrgns.is_empty () {
		let mut progress = false;
		let mut error = false;
		pending_alrgns.retain (|alrgn, ingrs| {
			ingrs.retain (|ingr| pending_ingrs.contains (ingr));
			if 1 < ingrs.len () { return true }
			if ingrs.is_empty () { error = true; return true }
			let ingr = ingrs.iter ().next ().unwrap ().clone ();
			pending_ingrs.remove (& ingr);
			result.push ((alrgn.clone (), ingr));
			progress = true;
			false
		});
		if ! progress || error { return Err ("No solution found".into ()) }
	}
	result.sort_by_key (|& (ref alrgn, _)| alrgn.clone ());
	Ok (result.iter ().map (|& (_, ref ingr)| ingr).display_delim (",").to_string ())
}

fn get_alrgns_ingrs <'inp> (input: & 'inp Input) -> (Alrgns <'inp>, Ingrs <'inp>) {
	let mut alrgns: HashMap <InpStr, HashSet <InpStr>> = default ();
	for food in & input.foods {
		for alrgn in & food.alrgns {
			alrgns.entry (alrgn.clone ())
				.and_modify (|ingrs| ingrs.retain (|ingr| food.ingrs.contains (ingr)))
				.or_insert_with (|| food.ingrs.iter ().cloned ().collect ());
		}
	}
	let ingrs: HashSet <InpStr> = alrgns.values ().flatten ().cloned ().collect ();
	(alrgns, ingrs)
}
