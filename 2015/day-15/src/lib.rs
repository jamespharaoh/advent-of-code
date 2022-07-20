//! Advent of Code 2015: Day 15: Science for Hungry People
//!
//! [https://adventofcode.com/2015/day/15](https://adventofcode.com/2015/day/15)

use aoc_common::*;

puzzle_info! {
	name = "Science for Hungry People";
	year = 2015;
	day = 15;
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Ingredient;

	/// Part one: Find the combination of ingredients which gives the maximum possible score.
	///
	/// Uses [`find_start_ingredients`] to work out which ingredients to include as a minimum, and
	/// [`calc_score`] to work out the score for a given combination of ingredients.
	///
	pub fn part_one (input: & [& str]) -> GenResult <u64> {

		let all_ingrs = model::parse_input (input) ?;
		let start_ingrs = find_start_ingredients (& all_ingrs);

		// keep adding one ingredient at a time, maximising the score each time, until we have the
		// right number

		let mixed_ingrs =
			iter::successors (Some (start_ingrs), |ingrs| Some (iter::empty ()
					.chain (ingrs.iter ().cloned ())
					.chain (iter::once ((all_ingrs).iter ()
						.map (|ingr| (ingr, calc_score (
							ingrs.iter ().cloned ().chain (iter::once (ingr)))))
						.max_by_key (|& (_, score)| score)
						.map (|(ingr, _)| ingr)
						.unwrap ()))
					.collect ()))
				.find (|ingrs| ingrs.len () == 100)
				.unwrap ();

		// work out the final score

		let max_score = calc_score (mixed_ingrs.iter ().cloned ());
		Ok (max_score)

	}

	/// Part two: Find the combination of ingredients which gives the maximum score and exactly 500
	/// calories in total.
	///
	/// This first iterates all combinations of ingredients which total 500 calories. This builds
	/// up the number of each ingredient iteratively, starting with [0, ..., 0], then [0, ..., 1],
	/// etc. At any point, if the number of calories exceeds 500 we short circuit to prevent
	/// needless iteration.
	///
	/// If the number of calories is exactly 500, and the number of ingredients is exactly 100, we
	/// note the score, and once iteration is complete the highest score is returned.
	///
	/// Uses [`calc_score`] to work out the score for a given combination of ingredients.
	///
	pub fn part_two (input: & [& str]) -> GenResult <u64> {

		let all_ingrs = {
			let mut ingrs = model::parse_input (input) ?;
			ingrs.sort_by_key (|ingr| cmp::Reverse (ingr.calories));
			ingrs
		};

		let mut stack: Vec <i32> = vec! [];
		let ingr_combos = iter::from_fn (|| {

			// pop off the last element, it only has one possible value. or, if the stack isn't
			// full this must be our first iteration and it's empty, so add a zero to start us off.

			if stack.len () == all_ingrs.len () {
				stack.pop ().unwrap ();
			} else if stack.is_empty () {
				stack.push (0);
			} else { unreachable! () }

			loop {

				// increment the ingredient counts

				* stack.last_mut ().unwrap () += 1;

				// short circuit if calories is over 500

				let calories: i32 =
					stack.iter ().copied ().enumerate ()
						.map (|(idx, num)| all_ingrs [idx].calories * num)
						.sum ();

				if calories > 500 {
					stack.pop ();
					if stack.is_empty () { return None }
					continue;
				}

				// fill in rest of stack with zeros...

				while stack.len () + 1 < all_ingrs.len () {
					stack.push (0);
				}

				// ...execpt for last place which we pick to get 100 ingredients total

				let num_final = 100 - stack.iter ().copied ().sum::<i32> ();
				if num_final < 0 { continue }
				let calories = calories + num_final * all_ingrs.last ().unwrap ().calories;

				// if the total calories is exactly 500, return the list of ingredients

				if calories == 500 {
					stack.push (num_final);
					return Some (
						stack.iter ().copied ().enumerate ()
							.flat_map (|(idx, num)|
								iter::repeat (& all_ingrs [idx])
									.take (num as usize))
							.collect::<Vec <_>> ()
					)
				}

			}

		});

		// go through the list and pick the highest score

		let max_score = ingr_combos
			.map (|ingrs| calc_score (ingrs.iter ().cloned ()))
			.max ()
			.unwrap ();

		Ok (max_score)

	}

	/// Find a minimum number of ingredients to get a positive score.
	///
	/// Goes through all combinations of ingredients totaling 1, then 2, etc, until it finds the
	/// minimum number of ingredients needed for a positive score. It then returns the combination
	/// which gives the max score for that number of ingredients.
	///
	pub fn find_start_ingredients (all_ingrs: & [Ingredient]) -> Vec <& Ingredient> {
		(1 ..= 100)
			.flat_map (|num| all_ingrs.iter ()
				.combinations_with_replacement (num)
				.map (|ingrs: Vec <& Ingredient>| (ingrs.clone (),
					calc_score (ingrs.iter ().cloned ())))
				.max_by_key (|& (_, score)| score)
				.filter (|& (_, score)| score > 0)
				.map (|(ingrs, _)| ingrs))
			.next ()
			.unwrap ()
	}

	/// Works out the score for a given combination of ingredients.
	///
	/// As per the puzzle description, this adds together each ingredient's value for capacity,
	/// durability, flavour and texture, then multiplies them all together. If any total is below
	/// zero, then the score is always zero.
	///
	pub fn calc_score <'a> (ingrs: impl IntoIterator <Item = & 'a Ingredient>) -> u64 {
		let (cap, dur, fla, tex) = ingrs.into_iter ()
			.fold ((0, 0, 0, 0), |sums, ingr| (
				sums.0 + ingr.capacity,
				sums.1 + ingr.durability,
				sums.2 + ingr.flavour,
				sums.3 + ingr.texture,
			));
		if cap < 0 || dur < 0 || fla < 0 || tex < 0 { return 0 }
		cap as u64 * dur as u64 * fla as u64 * tex as u64
	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;

	pub type Input = Vec <Ingredient>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Ingredient {
		pub name: String,
		pub capacity: i32,
		pub durability: i32,
		pub flavour: i32,
		pub texture: i32,
		pub calories: i32,
	}

	/// Parse a list of ingredients and their properties, according to the defined format.
	///
	/// ```
	/// # use aoc_2015_day_15::model::{ Ingredient, parse_input };
	/// let ingrs = parse_input (& [
	///   "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
	/// ]).unwrap ();
	/// assert_eq! (ingrs.len (), 1);
	/// assert_eq! (ingrs [0], Ingredient {
	///   name: "Butterscotch".to_string (),
	///   capacity: -1,
	///   durability: -2,
	///   flavour: 6,
	///   texture: 3,
	///   calories: 8,
	/// });
	/// ```
	///
	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
				parser.set_ignore_whitespace (true)
					.set_word_pred (char::is_alphanumeric);
				let name = parser.word () ?.to_string ();
				let capacity = parser.expect (": capacity") ?.int () ?;
				let durability = parser.expect (", durability") ?.int () ?;
				let flavour = parser.expect (", flavor") ?.int () ?;
				let texture = parser.expect (", texture") ?.int () ?;
				let calories = parser.expect (", calories") ?.int () ?;
				parser.end () ?;
				Ok (Ingredient { name, capacity, durability, flavour, texture, calories })
			}).map_parse_err (|char_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, char_idx + 1, line)
			)
		).collect ()
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
		"Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (62842880, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (57600000, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
