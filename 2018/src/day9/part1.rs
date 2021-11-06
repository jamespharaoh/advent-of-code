use std::error::Error;

use super::shared;
use super::shared::GameParams;

pub fn aoc2018_day9_part1 (input: & str) -> Result <String, Box <dyn Error>> {
	let params: GameParams = input.trim ().parse () ?;
	let result = shared::play (& params);
	Ok (format! ("{}", result))
}
