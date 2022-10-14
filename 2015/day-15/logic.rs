//! Logic for solving the puzzles.

use super::*;

use input::Input;
use model::Recipe;

/// Part one: Find the combination of ingredients which gives the maximum possible score.
///
/// Iterates all valid recipes and returns the maximum score.
///
pub fn part_one (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	Ok (
		iter_recipes (input)
			.map (|recipe| recipe.and_then (|recipe| recipe.score ()))
			.max ()
			.ok_or ("No solution found") ? ?
	)
}

/// Part two: Find the combination of ingredients which gives the maximum score and exactly 500
/// calories in total.
///
/// Iterates all valid recipes, filters on those with exactly `500` calories, and returns the
/// maximum score.
///
pub fn part_two (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	Ok (
		iter_recipes (input)
			.filter (|recipe| recipe
				.map (|recipe| recipe.calories () == 500_i32)
				.unwrap_or (true))
			.map (|recipe| recipe.and_then (|recipe| recipe.score ()))
			.max ()
			.ok_or ("No solution found") ? ?
	)
}

/// Validate extra conditions on the input.
///
pub fn check_input (input: & Input) -> GenResult <()> {
	if input.ingrs.len () < 2 { return Err ("Must have at least two ingredients".into ()) }
	if 6 < input.ingrs.len () { return Err ("Must have at most six ingredients".into ()) }
	Ok (())
}

/// Iterate through all recipes containing exactly 100 ingredients
///
/// We iterate through all values for all but the last ingredient, starting with all zero, then
/// increasing the last ingredient which can be increased without exceeding 100 total, and setting
/// all subsequent ingredients to zero. This gives for example, for four ingredients, `0, 0, 0`,
/// then `0, 0, 1`, etc...
///
/// The last ingredient we always set specifically to get exactly 100 ingredients total.
///
pub fn iter_recipes <'dat> (
	input: & 'dat Input,
) -> impl Iterator <Item = NumResult <Recipe>> + 'dat {

	let mut recipe = Recipe::default ();
	let mut finished = false;

	iter::from_fn (move || {

		if finished { return None }

		let mut result = recipe;
		if result.add_ingrs (
					& input.ingrs,
					input.ingrs.len () - 1,
					(100 - result.num_ingrs).pan_i8 ())
				.is_err () {
			finished = true;
			return Some (Err (Overflow));
		}

		for idx in (0 .. input.ingrs.len () - 1).rev () {
			if recipe.num_ingrs < 100 {
				if recipe.add_ingrs (& input.ingrs, idx, 1).is_err () {
					finished = true;
					return Some (Err (Overflow));
				}
				break;
			}
			if recipe.add_ingrs (
						& input.ingrs,
						idx,
						- recipe.ingrs [idx].pan_i8 ())
					.is_err () {
				finished = true;
				return Some (Err (Overflow));
			}
			if idx == 0 { finished = true; }
		}

		Some (Ok (result))

	})

}
