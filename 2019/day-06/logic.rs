//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one <'inp> (input: & Input <'inp>) -> GenResult <u32> {
	let parent_map: HashMap <InpStr <'inp>, InpStr <'inp>> =
		input.orbits.iter ()
			.map (|orbit| (orbit.satl.clone (), orbit.base.clone ()))
			.collect ();
	let com = InpStr::borrow ("COM");
	let mut sum: u32 = 0;
	for (start, _) in parent_map.iter () {
		let mut current = start.clone ();
		let mut seen = HashSet::new ();
		while current != com {
			if ! seen.insert (current.clone ()) { return Err ("Loop detected".into ()) }
			sum += 1;
			current = parent_map.get (& current)
				.ok_or (format! ("No parent for {current}")) ?
				.clone ();
		}
	}
	Ok (sum)
}

pub fn part_two <'inp> (input: & Input <'inp>) -> GenResult <u32> {

	let parent_map: HashMap <InpStr <'inp>, InpStr <'inp>> =
		input.orbits.iter ()
			.map (|orbit| (orbit.satl.clone (), orbit.base.clone ()))
			.collect ();

	let you = InpStr::borrow ("YOU");
	let san = InpStr::borrow ("SAN");

	let mut you_chain = chain (& parent_map, you.clone ()) ?;
	let mut san_chain = chain (& parent_map, san.clone ()) ?;

	if let Some (pos) = you_chain.iter ().position (|name| name == & san) {
		return Ok (pos.pan_u32 ())
	}
	if let Some (pos) = san_chain.iter ().position (|name| name == & you) {
		return Ok (pos.pan_u32 ())
	}

	loop {
		if you_chain.is_empty () || san_chain.is_empty () { break }
		if you_chain.last ().unwrap () != san_chain.last ().unwrap () { break }
		you_chain.pop ().unwrap ();
		san_chain.pop ().unwrap ();
	}

	Ok ((you_chain.len () + san_chain.len () - 2).pan_u32 ())

}

fn chain <'inp> (
	parent_map: & HashMap <InpStr <'inp>, InpStr <'inp>>,
	key: InpStr <'inp>,
) -> GenResult <Vec <InpStr <'inp>>> {
	let mut result: Vec <InpStr> = Vec::new ();
	let mut current = key;
	let com = InpStr::borrow ("COM");
	while current != com {
		if result.contains (& current) { return Err ("Loop detected".into ()) }
		result.push (current.clone ());
		current = parent_map.get (& current)
			.ok_or (format! ("No parent for {current}")) ?
			.clone ();
	}
	Ok (result)
}
