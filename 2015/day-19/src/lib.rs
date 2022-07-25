//! Advent of Code 2015: Day 19: Medicine for Rudolph
//!
//! [https://adventofcode.com/2015/day/19](https://adventofcode.com/2015/day/19)

use aoc_common::*;

puzzle_info! {
	name = "Medicine for Rudolph";
	year = 2015;
	day = 19;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use list::CharList;
	use list::List;
	use model::Input;

	pub fn part_one (input: Input) -> GenResult <u32> {
		let mut results = HashSet::new ();
		for (from, to) in input.replacements.iter () {
			let mut last_pos = 0;
			while let Some (pos) = input.medicine [last_pos .. ].find (from) {
				let pos = last_pos + pos;
				let new_molecule = format! ("{}{}{}",
					& input.medicine [ .. pos],
					to,
					& input.medicine [pos + from.len () .. ],
				);
				results.insert (new_molecule);
				last_pos = pos + from.len ();
			}
		}
		Ok (results.len () as u32)
	}

	pub fn part_two (input: Input) -> GenResult <u32> {

		// sanity check

		if ! input.replacements.iter ().any (|(from, _)| from == "e") {
			Err ("Must have at least one replacement from \"e\"") ?;
		}

		// list of continuations to handle branching without recursion

		let mut todo: VecDeque <(u32, List <String>, CharList)> = VecDeque::new ();
		todo.push_back ((0, List::new (), CharList::from (& input.medicine)));

		// set of previous iterations to short circuit repeated evaluation of the same state

		let mut seen: HashSet <(List <String>, CharList)> = HashSet::new ();

		// iterate over items from todo

		let mut min_match = None;
		'OUTER: while let Some ((todo_steps, todo_prefix, todo_suffix)) = todo.pop_back () {

			// abort if it looks too complex, this is mostly to make fuzzing more practical

			if todo.len () >= 1000 { Err ("1k items in backlog, giving up") ?; }

			// skip duplicated items

			if ! seen.insert ((todo_prefix.clone (), todo_suffix.clone ())) { continue }

			// output a message, disabled for now but could be added as a flag

			const VERBOSE: bool = false;
			if VERBOSE {
				if todo_prefix.is_empty () {
					println! ("queue={} steps={} {}", todo.len (), todo_steps, todo_suffix);
				} else {
					println! ("queue={} steps={} {} | {}", todo.len (), todo_steps,
						todo_prefix.iter ().join (" | "), todo_suffix);
				}
			}

			// add todo items for any replacements at the start of the suffix, also detect success

			for (from, to) in input.replacements.iter () {
				if from == "e" {

					// detect success

					if todo_prefix.is_empty () && to == todo_suffix {
						// TODO I am not entirely convinced that this is so simple, but I got the
						// right answer, so... I am /guessing/ that this has something to do with
						// this being an implementation of a greedy matcher, always trying to match
						// on the left first then recursing. But i would like to think about this a
						// bit more.
						min_match = Some (todo_steps + 1);
						break 'OUTER;
					}

					// never reduce a partial match to "e", it's only valid if it replaces the
					// entire molecule

				} else if let Some (suffix) = todo_suffix.strip_prefix (to) {
					if todo_prefix.is_empty () {

						// no prefix: add the split with no prefix also

						todo.push_back ((
							todo_steps + 1,
							List::new (),
							suffix.prepend (from),
						));

					} else {

						// some prefix: continue with the last item removed

						todo.push_back ((
							todo_steps + 1,
							todo_prefix.tail ().cloned ().unwrap (),
							suffix.prepend (from)
								.prepend (todo_prefix.head ().unwrap ()),
						));

					}
				}
			}

			// add todo items for possible splits: at some point we have to match a replacement at
			// the start of the molecule, so we must be able to reduce some suffix of the current
			// molecule into something so that we can reduce the new combined prefix and suffix.
			// so, we iterate over prefixes but stop when our prefix is not itself a prefix of any
			// replacements, since we know that it can't possibly be reduced further whatever the
			// suffix changes into.

			let mut prefix = String::new ();
			let mut suffix = & todo_suffix;
			while let Some ((& head, tail)) = suffix.cons () {
				prefix.push (head);
				suffix = tail;
				if ! input.replacements.iter ()
						.any (|(_, to)| to.starts_with (& prefix)) {
					break;
				}
				todo.push_back ((
					todo_steps,
					todo_prefix.push_front (prefix.clone ()),
					suffix.clone (),
				));
			}

		}

		// return result or error

		Ok (min_match.ok_or ("No solution found") ?)

	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone) ]
	pub struct Input {
		pub replacements: Replacements,
		pub medicine: String,
	}

	pub type Replacements = Vec <Replacement>;
	pub type Replacement = (String, String);

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Input> {
			if input.len () < 2 { Err ("Invalid input") ?; }
			if ! input [input.len () - 2].is_empty () { Err ("Invalid input") ?; }
			let is_chem = |input: & str| input.chars ().all (|ch| ch.is_ascii_alphanumeric ());
			let replacements = input [0 .. input.len () - 2].iter ().enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						parser.set_ignore_whitespace (true);
						let from = parser.word_if (is_chem) ?;
						let to = parser.expect_word ("=>") ?.word_if (is_chem) ?;
						parser.end () ?;
						if to.len () < from.len () { Err (parser.err ()) ?; }
						Ok ((from.to_string (), to.to_string ()))
					}).map_parse_err (|col_idx| format! (
						"Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line))
				).collect::<GenResult <Replacements>> () ?;
			let medicine = input.last ().unwrap ().to_string ();
			if medicine.is_empty () { Err ("Medicine must be at least one character") ?; }
			Ok (Input { replacements, medicine })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"H => HO",
		"H => OH",
		"O => HH",
		"",
		"HOH",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"e => H",
		"e => O",
		"H => HO",
		"H => OH",
		"O => HH",
		"",
		"HOH",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (EXAMPLE_TWO));
	}

}
