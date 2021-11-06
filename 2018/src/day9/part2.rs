use std::error::Error;

use super::shared;
use super::shared::GameParams;

pub fn aoc2018_day9_part2 (input: & str) -> Result <String, Box <dyn Error>> {
	let mut params: GameParams = input.trim ().parse () ?;
	params.last_marble *= 100;
	let result = shared::play (& params);
	Ok (format! ("{}", result))
}
