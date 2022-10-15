use super::*;

use input::Input;
use input::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (& input.first_row, input.params.num_rows_one)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (& input.first_row, input.params.num_rows_two)
}

fn calc_result (first_row: & [Tile], num_rows: u32) -> GenResult <u32> {
	if first_row.is_empty () { return Err ("Max row size is 1 tile".into ()) }
	if first_row.len () > 128 { return Err ("Max row size is 128 tiles".into ()) }
	let mut row: u128 = 0;
	let mut mask: u128 = 0;
	for tile in first_row.iter ().copied () {
		row <<= 1_u32;
		if tile == Tile::Trap { row |= 1; }
		mask <<= 1_u32;
		mask |= 1;
	}
	let num_tiles = mask.count_ones ();
	let mut num_safe = 0;
	for _ in 0 .. num_rows {
		num_safe += num_tiles - row.count_ones ();
		row = ((row << 1_u32) ^ (row >> 1_u32)) & mask;
	}
	Ok (num_safe)
}
