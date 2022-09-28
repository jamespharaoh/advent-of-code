//! Logic for solving the puzzles

#![ allow (clippy::identity_op) ]
#![ allow (clippy::unusual_byte_groupings) ]

use super::*;

use input::Input;
use input::InputPos;
use input::InputTile;

type State = Vec <u32>;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut state = vec! [ get_state (input) ? ];
	let mut state_temp = Vec::with_capacity (state.len ());
	let mut seen = HashSet::new ();
	loop {
		let layer_val = state [0];
		if ! seen.insert (layer_val) {
			return Ok (layer_val);
		}
		next_state::<false> (& state, & mut state_temp);
		mem::swap (& mut state, & mut state_temp);
	}
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut state: Vec <u32> =
		iter::empty ()
			.chain (iter::repeat (0_u32).take (100))
			.chain (iter::once (get_state (input) ?))
			.chain (iter::repeat (0_u32).take (100))
			.collect ();
	let mut state_temp: Vec <u32> =
		Vec::with_capacity (state.len ());
	for _ in 0_u32 .. 200 {
		next_state::<true> (& state, & mut state_temp);
		mem::swap (& mut state, & mut state_temp);
	}
	Ok (
		state.iter ().copied ()
			.map (u32::count_ones)
			.sum ()
	)
}

fn get_state (input: & Input) -> GenResult <u32> {
	if input.grid.size () != InputPos::new (5, 5) {
		return Err ("Grid must be exactly 5×5".into ());
	}
	Ok (
		input.grid.values ()
			.fold (0, |mut sum, tile| {
				sum >>= 1_u32;
				if matches! (tile, InputTile::Bug) { sum |= 1 << 24_u32; }
				sum
			})
	)
}

fn next_state <const RECURSE: bool> (old_state: & State, new_state: & mut State) {
	new_state.clear ();
	for layer_idx in 0 .. old_state.len () {
		new_state.push (next_layer::<RECURSE> (old_state, layer_idx));
	}
}

macro_rules! bitrep {
	( $type:ty, $pat:expr, $bits:literal, $num:literal ) => {
		{
			const fn bitrep (num: u32, bits: u32) -> $type {
				if num == 0 { 0 } else {
					bitrep (num - 1, $bits) << bits | $pat
				}
			}
			const VAL: $type = bitrep ($num, $bits);
			VAL
		}
	};
}

#[ inline ]
fn next_layer <const RECURSE: bool> (state: & [u32], layer_idx: usize) -> u32 {
	let layer_val = state [layer_idx];
	let outer_val = if RECURSE && 0 < layer_idx { state [layer_idx - 1] } else { 0 };
	let inner_val = if RECURSE && layer_idx + 1 < state.len () { state [layer_idx + 1] } else { 0 };
	if layer_val == 0 && outer_val == 0 && inner_val == 0 { return 0_u32 }
	let mut result = 0_u32;
	let mut pat_layers = calc_pat_layers::<RECURSE> (layer_val, outer_val);
	let inner_pats = if RECURSE { calc_inner_pats (inner_val) } else { [0_u64; 5] };
	for y in 0_u32 .. 5 {
		let mut pat_inner = inner_pats [y.pan_usize ()];
		for _x in 0_u32 .. 5 {
			let val = pat_layers & 0b_0000000_0000010_0000000 != 0;
			let adj_count = (pat_layers & 0b_0000010_0000101_0000010).count_ones ()
				+ (pat_inner & bitrep! (u64, 0b_00000001, 8, 5)).count_ones ();
			result >>= 1_u32;
			if adj_count == 1 || (adj_count == 2 && ! val) { result |= 1 << 24_u32; }
			pat_layers >>= 1_u32;
			pat_inner >>= 1_u32;
		}
		pat_layers >>= 2_u32;
	}
	if RECURSE { result &= 0b_11111_11111_11011_11111_11111; }
	result
}

#[ inline ]
fn calc_pat_layers <const RECURSE: bool> (layer_val: u32, outer_val: u32) -> u64 {
	let mut pat_layers = 0_u64
		| (layer_val.pan_u64 () & (0b_11111 << 0_u32)) << 8_u32
		| (layer_val.pan_u64 () & (0b_11111 << 5_u32)) << 10_u32
		| (layer_val.pan_u64 () & (0b_11111 << 10_u32)) << 12_u32
		| (layer_val.pan_u64 () & (0b_11111 << 15_u32)) << 14_u32
		| (layer_val.pan_u64 () & (0b_11111 << 20_u32)) << 16_u32;
	if RECURSE {
		if outer_val & (0b_00100 << 5_u32) != 0 { pat_layers |= 0b_0111110; }
		if outer_val & (0b_00100 << 15_u32) != 0 { pat_layers |= 0b_0111110 << 28_u32; }
		if outer_val & (0b_00010 << 10_u32) != 0 { pat_layers |= bitrep! (u64, 0b_0000001, 7, 5) << 7_u32; }
		if outer_val & (0b_01000 << 10_u32) != 0 { pat_layers |= bitrep! (u64, 0b_1000000, 7, 5) << 7_u32; }
	}
	pat_layers
}

#[ inline ]
const fn calc_inner_pats (inner_val: u32) -> [u64; 5] {
	let mut result = [0_u64; 5];
	let num_inner_up = (inner_val & (0b_11111 << 20)).count_ones ();
	result [3] |= bitrep! (u64, 0b_00000100, 8, 5) >> (40 - num_inner_up * 8);
	let num_inner_down = (inner_val & 0b_11111).count_ones ();
	result [1] |= bitrep! (u64, 0b_00000100, 8, 5) >> (40 - num_inner_down * 8);
	let num_inner_left = (inner_val & bitrep! (u32, 0b_10000, 5, 5)).count_ones ();
	result [2] |= bitrep! (u64, 0b_00001000, 8, 5) >> (40 - num_inner_left * 8);
	let num_inner_right = (inner_val & bitrep! (u32, 0b_00001, 5, 5)).count_ones ();
	result [2] |= bitrep! (u64, 0b_00000010, 8, 5) >> (40 - num_inner_right * 8);
	result
}
