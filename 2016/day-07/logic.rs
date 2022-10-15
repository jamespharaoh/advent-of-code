use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <usize> {
	Ok (
		input.lines.iter ()
			.filter (|line| has_abba_unbracketed (line) && ! has_abba_bracketed (line))
			.count ()
	)
}

pub fn part_two (input: & Input) -> GenResult <usize> {
	Ok (
		input.lines.iter ()
			.filter (|line| iter_abas_unbracketed (line)
				.any (|(a, b)| has_bab_bracketed (line, a, b)))
			.count ()
	)
}

fn has_abba_unbracketed (addr: & str) -> bool {
	addr.chars ()
		.scan (true, |state, ch| {
			Some (match ch {
				'[' => { * state = false; Some ('[') },
				']' => { * state = true; Some (']') },
				_ => (* state).then_some (ch),
			})
		})
		.flatten ()
		.tuple_windows::<(_, _, _, _)> ()
		.any (|(a, b, c, d)| a == d && b == c && a != b)
}

fn has_abba_bracketed (addr: & str) -> bool {
	addr.chars ()
		.scan (false, |state, ch| {
			Some (match ch {
				'[' => { * state = true; Some ('[') },
				']' => { * state = false; Some (']') },
				_ => (* state).then_some (ch),
			})
		})
		.flatten ()
		.tuple_windows::<(_, _, _, _)> ()
		.any (|(a, b, c, d)| a == d && b == c && a != b)
}

fn iter_abas_unbracketed (addr: & str) -> impl Iterator <Item = (char, char)> + '_ {
	addr.chars ()
		.scan (true, |state, ch| {
			Some (match ch {
				'[' => { * state = false; Some ('[') },
				']' => { * state = true; Some (']') },
				_ => (* state).then_some (ch),
			})
		})
		.flatten ()
		.tuple_windows::<(_, _, _)> ()
		.filter (|& (a, b, c)| a == c && a != b)
		.map (|(a, b, _)| (a, b))
}

fn has_bab_bracketed (addr: & str, a: char, b: char) -> bool {
	addr.chars ()
		.scan (false, |state, ch| {
			Some (match ch {
				'[' => { * state = true; Some ('[') },
				']' => { * state = false; Some (']') },
				_ => (* state).then_some (ch),
			})
		})
		.flatten ()
		.tuple_windows::<(_, _, _)> ()
		.any (|(a1, b1, c1)| a == b1 && b == a1 && b == c1)
}
