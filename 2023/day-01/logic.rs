use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut sum = 0;
	for line in & input.lines {
		let first_digit =
			line.chars ().find (char::is_ascii_digit)
				.ok_or ("Each line must contain at least one digit") ?;
		let last_digit =
			line.chars ().rev ().find (char::is_ascii_digit).unwrap ();
		sum += first_digit.to_digit (10).unwrap () * 10 + last_digit.to_digit (10).unwrap ();
	}
	Ok (sum)
}

#[ allow (clippy::string_slice) ]
pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut sum = 0;
	for line in & input.lines {
		let first_digit = {
			let mut line = line.as_str ();
			'DIGIT: loop {
				for & (digit_str, digit_val) in DIGITS {
					if line.starts_with (digit_str) { break 'DIGIT digit_val }
				}
				let Some (ch) = line.chars ().next () else {
					return Err ("Each line must contain at least one digit".into ());
				};
				line = & line [ch.len_utf8 () .. ];
			}
		};
		let last_digit = {
			let mut line = line.as_str ();
			'DIGIT: loop {
				for & (digit_str, digit_val) in DIGITS {
					if line.ends_with (digit_str) { break 'DIGIT digit_val }
				}
				let Some ((pos, _)) = line.char_indices ().next_back () else {
					return Err ("Each line must contain at least one digit".into ());
				};
				line = & line [ .. pos];
			}
		};
		sum += first_digit * 10 + last_digit;
	}
	Ok (sum)
}

static DIGITS: & [(& str, u32)] = & [
	("0", 0), ("zero", 0),
	("1", 1), ("one", 1),
	("2", 2), ("two", 2),
	("3", 3), ("three", 3),
	("4", 4), ("four", 4),
	("5", 5), ("five", 5),
	("6", 6), ("six", 6),
	("7", 7), ("seven", 7),
	("8", 8), ("eight", 8),
	("9", 9), ("nine", 9),
];
