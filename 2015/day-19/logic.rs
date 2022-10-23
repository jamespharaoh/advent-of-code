//! Logic for solving the puzzles.

use super::*;

use input::Input;
use list::CharList;
use list::List;

#[ allow (clippy::string_slice) ]
pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut results = HashSet::new ();
	for & (ref from, ref to) in input.replacements.iter () {
		let mut last_pos = 0;
		while let Some (pos) = input.medicine [last_pos .. ].find (& ** from) {
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
	Ok (results.len ().pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {

	// sanity checks

	if ! input.replacements.iter ().any (|& (ref from, _)| from == "e") {
		Err ("Must have at least one replacement from \"e\"") ?;
	}

	if input.medicine.len () > 512 {
		Err ("Medicine must be 512 chars or less") ?;
	}

	if ! input.medicine.chars ().all (|ch| ch.is_ascii_alphanumeric ()) {
		Err ("Medicine must be ASCII alphanumeric") ?;
	}

	for & (ref from, ref to) in input.replacements.iter () {

		if ! from.chars ().all (|ch| ch.is_ascii_alphanumeric ())
				|| ! to.chars ().all (|ch| ch.is_ascii_alphanumeric ()) {
			Err ("All replacements must be ASCII alphanumeric") ?;
		}

		if from.chars ()
				.filter (|ch| ch.is_ascii_uppercase () || ch.is_ascii_digit ())
				.count ()
			>= to.chars ()
				.filter (|ch| ch.is_ascii_uppercase () || ch.is_ascii_digit ())
				.count () {
			Err ("Replacements must always increase number of more ASCII uppercase/digits") ?;
		}

	}

	// list of continuations to handle branching without recursion, and starting state

	let mut todo: VecDeque <(u32, List <String>, CharList)> = VecDeque::new ();
	todo.push_back ((0, List::new (), CharList::from (& * input.medicine)));

	// set of previous iterations to short circuit repeated evaluation of the same state

	let mut seen: HashSet <(List <String>, CharList)> = HashSet::new ();

	// iterate over items from todo

	let mut min_match = None;
	'OUTER: while let Some ((todo_steps, todo_prefix, todo_suffix)) = todo.pop_back () {

		// abort if it looks too complex, this is mostly to make fuzzing more practical

		if todo.len () >= 1500 { Err ("1500 states pending, giving up") ?; }
		if seen.len () >= 3000 { Err ("3000 unique states seen, giving up") ?; }

		// skip duplicated items

		if ! seen.insert ((todo_prefix.clone (), todo_suffix.clone ())) { continue }

		// output a message, disabled for now but could be added as a flag

		const VERBOSE: bool = false;
		#[ allow (clippy::print_stdout) ]
		if VERBOSE {
			if todo_prefix.is_empty () {
				println! ("queue={} steps={} {}", todo.len (), todo_steps, todo_suffix);
			} else {
				println! ("queue={todo_len} steps={todo_steps} {todo_prefix} | {todo_suffix}",
					todo_len = todo.len (),
					todo_prefix = todo_prefix.iter ().display_delim (" | "));
			}
		}

		// add todo items for any replacements at the start of the suffix, also detect success

		for & (ref from, ref to) in input.replacements.iter () {
			if from == "e" {

				// detect success

				if todo_prefix.is_empty () && ** to == todo_suffix {
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
			} else {
				// no match, carry on
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
			if ! input.replacements.iter ().any (|& (_, ref to)| to.starts_with (& prefix)) {
				break;
			}
			todo.push_back ((
				todo_steps,
				todo_prefix.with_push_front (prefix.clone ()),
				suffix.clone (),
			));
		}

	}

	// return result or error

	Ok (min_match.ok_or ("No solution found") ?)

}
