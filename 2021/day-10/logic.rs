use super::*;

use input::Input;
use model::Delim;
use model::Mode;

pub fn part_one (input: & Input) -> GenResult <u64> {
	input.lines.iter ()
		.map (|line| {
			let mut stack: Vec <Delim> = Vec::new ();
			line.iter ()
				.map (move |token| match token.mode () {
					Mode::Open => { stack.push (token.delim ()); Ok (0) },
					Mode::Close => {
						let stack_delim = stack.pop ()
							.ok_or ("Too many closing delimiters") ?;
						Ok::<_, GenError> (
							if stack_delim != token.delim () {
								token.delim ().mismatched_points ()
							} else { 0 }
						)
					},
				})
				.try_fold (0, |sum, item| {
					let item = item ?;
					Ok::<_, GenError> (chk! (sum + item) ?)
				})
		})
		.try_fold (0, |sum, item| {
			let item = item ?;
			Ok::<_, GenError> (chk! (sum + item) ?)
		})
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut scores: Vec <u64> =
		input.lines.iter ()
			.filter_map (|line| {
				let mut stack: Vec <Delim> = Vec::new ();
				for token in line.iter () {
					match token.mode () {
						Mode::Open => stack.push (token.delim ()),
						Mode::Close => {
							let expect = some_or! (
								stack.pop (),
								return Some (Err::<_, GenError> ("Too many closing delimters".into ())));
							if token.delim () != expect { return None }
						},
					}
				}
				Some (
					stack.into_iter ().rev ()
						.map (Delim::not_closed_points)
						.try_fold (0, |total, value| Ok (chk! (value + total * 5) ?))
				)
			})
			.try_collect () ?;
	if scores.is_empty () { return Err ("No solution found".into ()) }
	scores.sort_unstable ();
	Ok (scores [(scores.len () - 1) / 2])
}
