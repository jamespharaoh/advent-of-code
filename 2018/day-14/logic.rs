//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {
	let num_recipes: u32 = input.value.parse ().unwrap ();
	let mut recipes: Vec <u8> = vec! [ 3, 7 ];
	let mut idx_0 = 0_usize;
	let mut idx_1 = 1_usize;
	while recipes.len () < num_recipes.pan_usize () + 10 {
		let recipe_0 = recipes [idx_0];
		let recipe_1 = recipes [idx_1];
		let sum = recipe_0 + recipe_1;
		if sum >= 10 {
			recipes.push (1);
			recipes.push (sum - 10);
		} else {
			recipes.push (sum);
		}
		idx_0 += recipe_0.pan_usize () + 1;
		while recipes.len () <= idx_0 { idx_0 -= recipes.len (); }
		idx_1 += recipe_1.pan_usize () + 1;
		while recipes.len () <= idx_1 { idx_1 -= recipes.len (); }
	}
	Ok (
		recipes.iter ()
			.skip (num_recipes.pan_usize ())
			.take (10)
			.map (|& recipe| char::from_digit (recipe.pan_u32 (), 10).unwrap ())
			.collect ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let search: Vec <u8> =
		input.value.chars ()
			.map (|ch| ch.to_digit (10).unwrap ().pan_u8 ())
			.collect ();
	let mut recipes: Vec <u8> = vec! [ 3, 7 ];
	let mut idx_0 = 0_usize;
	let mut idx_1 = 1_usize;
	let mut search_idx = 0;
	loop {
		let recipe_0 = recipes [idx_0];
		let recipe_1 = recipes [idx_1];
		let sum = recipe_0 + recipe_1;
		if sum >= 10 {
			recipes.push (1);
			recipes.push (sum - 10);
		} else {
			recipes.push (sum);
		}
		idx_0 += recipe_0.pan_usize () + 1;
		while recipes.len () <= idx_0 { idx_0 -= recipes.len (); }
		idx_1 += recipe_1.pan_usize () + 1;
		while recipes.len () <= idx_1 { idx_1 -= recipes.len (); }
		while search_idx + search.len () <= recipes.len () {
			if & recipes [search_idx .. search_idx + search.len ()] == search.as_slice () {
				return Ok (search_idx.pan_u32 ());
			}
			if search_idx == input.params.max_recipes.pan_usize () {
				return Err ("Giving up after reaching recipe limit".into ());
			}
			search_idx += 1;
		}
	}
}
