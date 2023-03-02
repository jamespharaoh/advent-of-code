use super::*;

use input::Input;
use model::Item;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.pairs.iter ().enumerate ()
			.filter (|& (_, pair)| pair.one < pair.two)
			.map (|(pair_idx, _)| (pair_idx + 1).pan_u32 ())
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let div_two = Item::List (vec! [ Item::List (vec! [ Item::Value (2) ]) ]);
	let div_six = Item::List (vec! [ Item::List (vec! [ Item::Value (6) ]) ]);
	let packets: Vec <Item> =
		input.pairs.iter ()
			.flat_map (|pair| [ pair.one.clone (), pair.two.clone () ])
			.chain ([ div_two.clone (), div_six.clone () ])
			.sorted ()
			.collect ();
	let idx_two = packets.iter ().position (|packet| packet == & div_two).unwrap ();
	let idx_six = packets.iter ().position (|packet| packet == & div_six).unwrap ();
	Ok ((idx_two.pan_u32 () + 1) * (idx_six.pan_u32 () + 1))
}
