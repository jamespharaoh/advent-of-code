use super::*;

use input::Input;
use input::InputColour;
use input::InputRound;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	Ok (input.games.iter ()
		.filter (|game| game.rounds.iter ().all (|round| {
			let (num_red, num_green, num_blue) = draws_for_round (round);
			num_red <= 12 && num_green <= 13 && num_blue <= 14
		}))
		.map (|game| game.id)
		.sum ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	Ok (input.games.iter ()
		.map (|game| game.rounds.iter ().fold (
			(0, 0, 0),
			|(max_red, max_green, max_blue), round| {
				let (num_red, num_green, num_blue) = draws_for_round (round);
				(
					cmp::max (max_red, num_red),
					cmp::max (max_green, num_green),
					cmp::max (max_blue, num_blue),
				)
			}))
		.map (|(max_red, max_green, max_blue)| max_red * max_green * max_blue)
		.sum ())
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.games.is_empty () {
		return Err ("Input must have at least one game".into ());
	}
	for game in & input.games {
		if game.rounds.is_empty () {
			return Err ("Game must have at least one round".into ());
		}
		for round in & game.rounds {
			if round.draws.is_empty () {
				return Err ("Round must have at least one draw".into ());
			}
			if 1 < round.draws.iter ().filter (|round| round.colour == InputColour::Red).count () {
				return Err ("Draw must not declare a colour more than once".into ());
			}
			if 1 < round.draws.iter ().filter (|round| round.colour == InputColour::Green).count () {
				return Err ("Draw must not declare a colour more than once".into ());
			}
			if 1 < round.draws.iter ().filter (|round| round.colour == InputColour::Blue).count () {
				return Err ("Draw must not declare a colour more than once".into ());
			}
		}
	}
	Ok (())
}

fn draws_for_round (round: & InputRound) -> (u32, u32, u32) {
	(
		round.draws.iter ()
			.find (|round| round.colour == InputColour::Red)
			.map (|round| round.num)
			.unwrap_or_default (),
		round.draws.iter ()
			.find (|round| round.colour == InputColour::Green)
			.map (|round| round.num)
			.unwrap_or_default (),
		round.draws.iter ()
			.find (|round| round.colour == InputColour::Blue)
			.map (|round| round.num)
			.unwrap_or_default (),
	)
}
