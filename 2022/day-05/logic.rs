use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {
	calc_result (input, false)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	calc_result (input, true)
}

fn calc_result (input: & Input, in_order: bool) -> GenResult <String> {
	check_input (input) ?;
	let mut piles = get_piles (input) ?;
	apply_steps (input, & mut piles, in_order) ?;
	get_top_crates (& piles)
}

fn check_input (input: & Input) -> GenResult <()> {
	for line in & input.lines {
		if line.crates.len () != input.pile_nums.len () {
			return Err ("Input piles don't line up".into ());
		}
	}
	for step in & input.steps {
		if step.from < 1 || input.pile_nums.len ().pan_u8 () < step.from {
			return Err ("Invalid step".into ());
		}
		if step.to < 1 || input.pile_nums.len ().pan_u8 () < step.to {
			return Err ("Invalid step".into ());
		}
	}
	Ok (())
}

fn get_piles (input: & Input) -> GenResult <Vec <Vec <char>>> {
	let piles: Vec <Vec <char>> =
		(0 .. input.pile_nums.len ())
			.map (|pile_idx| {
				input.lines.iter ().rev ()
					.map_while (|line| line.crates [pile_idx])
					.collect ()
			})
			.collect ();
	for (pile_idx, pile) in piles.iter ().enumerate () {
		if input.lines.iter ().filter_map (|line| line.crates [pile_idx]).count () != pile.len () {
			return Err ("Crates must be stacked according to gravity".into ());
		}
	}
	Ok (piles)
}

fn apply_steps (input: & Input, piles: & mut [Vec <char>], in_order: bool) -> GenResult <()> {
	let mut temp = Vec::new ();
	for step in & input.steps {
		temp.clear ();
		for _ in 0 .. step.num {
			let crate_ch =
				piles [(step.from - 1).pan_usize ()].pop ()
					.ok_or ("Unable to follow steps") ?;
			temp.push (crate_ch);
		}
		if in_order { temp.reverse (); }
		for & crate_ch in temp.iter () {
			piles [(step.to - 1).pan_usize ()].push (crate_ch);
		}
	}
	Ok (())
}

fn get_top_crates (piles: & [Vec <char>]) -> GenResult <String> {
	if piles.iter ().any (Vec::is_empty) {
		return Err ("Some piles are empty after applying all steps".into ());
	}
	let val =
		piles.iter ()
			.map (|pile| * pile.last ().unwrap ())
			.collect ();
	Ok (val)
}
