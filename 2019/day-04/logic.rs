//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	Ok (
		iter_passwords (input)
			.filter (is_ascending)
			.filter (has_repeated)
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	Ok (
		iter_passwords (input)
			.filter (is_ascending)
			.filter (has_repeated_alone)
			.count ()
			.pan_u32 ()
	)
}

fn sanity_check (input: & Input) -> GenResult <()> {
	let range = 100_000 ..= 999_999;
	if ! range.contains (& input.min) || ! range.contains (& input.max) {
		return Err ("Password must be a six digit number".into ());
	}
	Ok (())
}

fn iter_passwords (input: & Input) -> impl Iterator <Item = [char; 6]> {
	let first_password = num_to_char_array (input.min);
	let last_password = num_to_char_array (input.max);
	iter::successors (Some (first_password), |& password| next_password (password))
		.take_while (move |& password| password <= last_password)
}

fn num_to_char_array (num: u32) -> [char; 6] {
	num.to_string ().chars ()
		.collect::<Vec <char>> ()
		.try_into ()
		.unwrap ()
}

fn next_password (mut password: [char; 6]) -> Option <[char; 6]> {
	let mut idx = 5;
	let mut ch;
	loop {
		ch = password [idx];
		if ch != '9' { break }
		if idx == 0 { return None }
		idx -= 1;
	}
	ch = (ch.pan_u32 () + 1).pan_char ();
	password.iter_mut ().skip (idx).for_each (|password_ch| * password_ch = ch);
	Some (password)
}

fn is_ascending (password: & [char; 6]) -> bool {
	password.iter ()
		.tuple_windows ()
		.all (|(a, b)| a <= b)
}

fn has_repeated (password: & [char; 6]) -> bool {
	password.iter ()
		.tuple_windows ()
		.any (|(a, b)| a == b)
}

fn has_repeated_alone (password: & [char; 6]) -> bool {
	iter::empty ()
		.chain (iter::once ('X'))
		.chain (password.iter ().copied ())
		.chain (iter::once ('X'))
		.tuple_windows ()
		.any (|(a, b, c, d)| a != b && b == c && c != d)
}
