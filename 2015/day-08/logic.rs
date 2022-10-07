use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <usize> {
	input.strings.iter ()
		.map (|orig| Ok::<_, GenError> ((orig.chars ().count (), decoded_len (orig) ?)))
		.map_ok (|(orig_len, decoded_len)| orig_len - decoded_len)
		.try_fold (0, |sum, item| Ok::<_, GenError> (sum + item ?))
}

pub fn part_two (input: & Input) -> GenResult <usize> {
	Ok (
		input.strings.iter ()
			.map (|orig| (orig.chars ().count (), encoded_len (orig)))
			.map (|(orig_len, encoded_len)| encoded_len - orig_len)
			.sum ()
	)
}

fn decoded_len (input: & str) -> GenResult <usize> {
	let mut ch_iter = input.chars ();
	let mut next_ch = || ch_iter.next ().ok_or ("Unexpected end");
	let mut num = 0;
	if next_ch () ? != '"' { return Err ("Missing open quote".into ()) }
	loop {
		match next_ch () ? {
			'"' => break,
			'\\' => match next_ch () ? {
				'"' | '\\' => num += 1,
				'x' => {
					let (ch_0, ch_1) = (next_ch () ?, next_ch () ?);
					if ! ch_0.is_ascii_hexdigit () || ! ch_1.is_ascii_hexdigit () {
						return Err ("Invalid string escape".into ());
					}
					num += 1;
				}
				_ => return Err ("Invalid string escape".into ()),
			},
			_ => num += 1,
		}
	}
	if ch_iter.next ().is_some () { return Err ("Extra chars at end".into ()) }
	Ok (num)
}

fn encoded_len (input: & str) -> usize {
	2 + input.chars ()
		.map (|ch| match ch { '\\' | '"' => 2, _ => 1 })
		.sum::<usize> ()
}
