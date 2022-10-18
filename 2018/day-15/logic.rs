//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Tile;
use state::State;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let state = State::build (input.grid.clone (), 3, 3) ?;
	let state = calc_result (state) ?;
	if state.winner ().is_none () { return Err ("Stalemate".into ()) }
	Ok (state.score ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let num_elves = input.grid.values ().filter (|& tile| tile == Tile::Elf).count ();
	let mut lose = 0;
	let mut win = 200;
	let mut win_score = None;
	while win - lose > 1 {
		let elf_attack = if 40 < win - lose { lose + 20 } else { (lose + win) / 2 };
		let state = State::build (input.grid.clone (), 3, elf_attack) ?;
		let state = calc_result (state) ?;
		let winner = some_or! (state.winner (), return Err ("Stalemate".into ()));
		if winner == Tile::Elf && state.units ().len () == num_elves {
			win_score = Some (state.score ());
			win = elf_attack;
		} else {
			lose = elf_attack;
		}
	}
	Ok (win_score.ok_or ("No solution found") ?)
}

fn calc_result (mut state: State) -> GenResult <State> {
	while ! state.tick () {
		if state.num_rounds () == 200 {
			return Err ("Giving up after 200 rounds".into ());
		}
	}
	Ok (state)
}
