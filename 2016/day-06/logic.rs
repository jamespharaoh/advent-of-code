use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {
	calc_result (
		& input.lines,
		|col_counts| col_counts.iter ()
			.max_by_key (|& (_, num)| num)
			.map (|(& ch, _)| ch)
			.unwrap ())
}

pub fn part_two (input: & Input) -> GenResult <String> {
	calc_result (
		& input.lines,
		|col_counts| col_counts.iter ()
			.min_by_key (|& (_, num)| num)
			.map (|(& ch, _)| ch)
			.unwrap ())
}

pub fn calc_result (
	lines: & [InpStr],
	map_fn: fn (HashMap <char, usize>) -> char,
) -> GenResult <String> {
	Ok (
		lines.iter ()
			.fold (
				iter::repeat (HashMap::new ())
					.take (lines [0].chars ().count ())
					.collect::<Vec <_>> (),
				|mut all_counts, line| {
					all_counts.iter_mut ()
						.zip (line.chars ())
						.for_each (|(col_counts, ch)|
							* col_counts.entry (ch).or_insert (0) += 1_usize);
					all_counts
				})
			.into_iter ()
			.map (map_fn)
			.collect ()
	)
}
