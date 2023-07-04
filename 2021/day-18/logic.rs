#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use input::Input;
use model::Number;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	if input.nums.is_empty () {
		return Err ("Must provide at least one number".into ());
	}
	let nums: Vec <Number> =
		input.nums.iter ()
			.map (Number::try_from)
			.try_collect () ?;
	let sum = Number::sum (nums);
	Ok (sum.unwrap ().magnitude ())
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	let nums: Vec <Number> =
		input.nums.iter ()
			.map (Number::try_from)
			.try_collect () ?;
	let mut best: Val = Val::MIN;
	for i in 0 .. nums.len () {
		for j in 0 .. nums.len () {
			if i == j { continue }
			let value = Number::add (nums [i].clone (), nums [j].clone ()).magnitude ();
			if value > best { best = value; }
		}
	}
	Ok (best)
}
