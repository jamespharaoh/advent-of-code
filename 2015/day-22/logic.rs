//! Logic for solving the puzzles.

use super::*;

use input::Input;
use model::Boss;
use model::Player;

pub fn part_one (input: & Input) -> GenResult <u16> {
	calc_result (Player::default (), input.boss, Difficulty::Easy)
}

pub fn part_two (input: & Input) -> GenResult <u16> {
	calc_result (Player::default (), input.boss, Difficulty::Hard)
}

fn calc_result (player: Player, boss: Boss, difficulty: Difficulty) -> GenResult <u16> {
	outcomes (player, boss, difficulty)
		.filter (|outcome| outcome.winner == Contender::Player)
		.map (|outcome| outcome.mana)
		.next ()
		.ok_or ("No solution found".into ())
}

fn outcomes (
	player: Player,
	boss: Boss,
	difficulty: Difficulty,
) -> impl Iterator <Item = Outcome> {

	let mut search = PrioritySearch::with_hash_map (
		|state: State, _, mut adder: PrioritySearchAdder <'_, _, _, _>|
			match (state.player.hit_points != 0, state.boss.hit_points != 0) {
				(true, false) => Some (Outcome { winner: Contender::Player, mana: state.mana }),
				(false, true) => Some (Outcome { winner: Contender::Boss, mana: state.mana }),
				(true, true) => {
					for next_state in next_states (state) {
						adder.add (next_state, next_state.mana);
					}
					None
				},
				(false, false) => unreachable! (),
			});

	search.push (State {
		player,
		boss,
		effects: Effects { shield: 0, poison: 0, recharge: 0 },
		turn: Contender::Player,
		difficulty,
		mana: 0,
	}, 0);

	search.flatten ()

}

fn next_states (mut state: State) -> TinyVec <State, 5> {

	if state.effects.shield > 0 {
		state.effects.shield -= 1;
	}

	if state.effects.poison > 0 {
		state.boss.hit_points -= cmp::min (state.boss.hit_points, 3);
		if state.boss.hit_points == 0 { return tiny_vec! [ state ] }
		state.effects.poison -= 1;
	}

	if state.effects.recharge > 0 {
		state.player.mana += 101;
		state.effects.recharge -= 1;
	}

	if state.turn == Contender::Player {

		state.turn = Contender::Boss;

		if state.player.mana < 53 {
			state.player.hit_points = 0;
			return tiny_vec! [ state ];
		}

		if state.difficulty == Difficulty::Hard {
			state.player.hit_points -= 1;
			if state.player.hit_points == 0 {
				return tiny_vec! [ state ];
			}
		}

		let mut results = TinyVec::new ();

		if state.player.mana >= 53 {
			let mut state = state;
			state.player.mana -= 53;
			state.boss.hit_points -= cmp::min (state.boss.hit_points, 4);
			state.mana += 53;
			results.push (state);
		}

		if state.player.mana >= 73 {
			let mut state = state;
			state.player.hit_points += 2;
			state.player.mana -= 73;
			state.boss.hit_points -= cmp::min (state.boss.hit_points, 2);
			state.mana += 73;
			results.push (state);
		}

		if state.player.mana >= 113 && state.effects.shield == 0 {
			let mut state = state;
			state.player.mana -= 113;
			state.effects.shield = 6;
			state.mana += 113;
			results.push (state);
		}

		if state.player.mana >= 173 && state.effects.poison == 0 {
			let mut state = state;
			state.player.mana -= 173;
			state.effects.poison = 6;
			state.mana += 173;
			results.push (state);
		}

		if state.player.mana >= 229 && state.effects.recharge == 0 {
			let mut state = state;
			state.player.mana -= 229;
			state.effects.recharge = 5;
			state.mana += 229;
			results.push (state);
		}

		results

	} else {

		state.turn = Contender::Player;

		let armor = if state.effects.shield > 0 { 7 } else { 0 };
		let attack = if state.boss.damage <= armor { 1 } else { state.boss.damage - armor };
		state.player.hit_points -= cmp::min (state.player.hit_points, attack);
		tiny_vec! [ state ]

	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct Outcome {
	winner: Contender,
	mana: u16,
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct Effects {
	shield: u8,
	poison: u8,
	recharge: u8,
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct State {
	player: Player,
	boss: Boss,
	effects: Effects,
	turn: Contender,
	difficulty: Difficulty,
	mana: u16,
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
enum Contender { Player, Boss }

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
enum Difficulty { Easy, Hard }
