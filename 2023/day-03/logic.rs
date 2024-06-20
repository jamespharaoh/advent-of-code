use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let lines: Vec <Vec <char>> =
		input.lines.iter ()
			.map (|line| line.chars ().collect::<Vec <char>> ())
			.collect ();
	let mut sum = 0;
	for (line_idx, line) in lines.iter ().enumerate () {
		for result in find_numbers (line) {
			let (num, start, end) = result ?;
			if find_symbol (& lines, line_idx, start, end) {
				chk! (sum += num) ?;
			}
		}
	}
	Ok (sum)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let lines: Vec <Vec <char>> =
		input.lines.iter ()
			.map (|line| line.chars ().collect::<Vec <char>> ())
			.collect ();
	let mut sum = 0;
	for (line_idx, line) in lines.iter ().enumerate () {
		for (char_idx, _) in line.iter ().copied ().enumerate ().filter (|& (_, ch)| ch == '*') {
			let mut nums = Vec::new ();
			let search_start = if 0 < char_idx { char_idx - 1 } else { 0 };
			let search_end = char_idx + 2;
			if 0 < line_idx { nums.extend (find_numbers (& lines [line_idx - 1])); }
			nums.extend (find_numbers (& lines [line_idx]));
			if line_idx + 1 < lines.len () { nums.extend (find_numbers (& lines [line_idx + 1])); }
			let nums: Vec <u64> =
				nums.into_iter ()
					.filter_ok (|& (_, num_start, num_end)|
						num_start < search_end && search_start < num_end)
					.map_ok (|(num, _, _)| num)
					.try_collect () ?;
			if nums.len () != 2 { continue }
			chk! (sum += nums [0] * nums [1]) ?;
		}
	}
	Ok (sum)
}

fn find_symbol (lines: & [Vec <char>], line_idx: usize, start: usize, end: usize) -> bool {
	(0 < line_idx && find_symbol_line (lines [line_idx - 1].as_slice (), start, end))
		|| find_symbol_line (lines [line_idx].as_slice (), start, end)
		|| (line_idx + 1 < lines.len ()
			&& find_symbol_line (lines [line_idx + 1].as_slice (), start, end))
}

fn find_symbol_line (line: & [char], start: usize, end: usize) -> bool {
	let start = cmp::min (if 0 < start { start - 1 } else { start }, line.len ());
	let end = cmp::min (end + 1, line.len ());
	line [start .. end].iter ().copied ().any (|ch| ch != '.' && ! ch.is_ascii_digit ())
}

fn find_numbers (line: & [char]) -> impl Iterator <Item = GenResult <(u64, usize, usize)>> + '_ {
	let mut char_idx = 0;
	iter::from_fn (move || {
		while char_idx < line.len () {
			if line [char_idx].is_ascii_digit () {
				let start = char_idx;
				let mut num = 0_u64;
				while char_idx < line.len () && line [char_idx].is_ascii_digit () {
					match chk! (num * 10 + line [char_idx].to_digit (10).unwrap ().pan_u64 ()) {
						Ok (new_num) => num = new_num,
						Err (err) => return Some (Err (err.into ())),
					}
					char_idx += 1;
				}
				let end = char_idx;
				return Some (Ok ((num, start, end)));
			} else {
				char_idx += 1;
			}
		}
		None
	})
}
