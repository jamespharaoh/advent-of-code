//! Advent of Code 2015: Day 22: Wizard Simulator 20XX
//!
//! [https://adventofcode.com/2015/day/22](https://adventofcode.com/2015/day/22)

use aoc_common::*;

puzzle_info! {
	name = "Wizard Simulator 20XX";
	year = 2015;
	day = 22;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Boss;
	use model::Input;
	use model::Player;
	use search::PrioritySearch;
	use search::PrioritySearchAdder;

	pub fn part_one (input: Input) -> GenResult <u16> {
		Ok (calc_result (input, Difficulty::Easy).ok_or ("No solution found") ?)
	}

	pub fn part_two (input: Input) -> GenResult <u16> {
		Ok (calc_result (input, Difficulty::Hard).ok_or ("No solution found") ?)
	}

	fn calc_result (input: Input, difficulty: Difficulty) -> Option <u16> {
		outcomes (input, difficulty)
			.filter (|outcome| outcome.winner == Contender::Player)
			.map (|outcome| outcome.mana)
			.next ()
	}

	fn outcomes (input: Input, difficulty: Difficulty) -> impl Iterator <Item = Outcome> {

		let mut search = PrioritySearch::with_hash_map (
			|state: State, _, mut adder: PrioritySearchAdder <'_, _, _, _>| {
				if state.boss.hit_points == 0 {
					Some (Outcome { winner: Contender::Player, mana: state.mana })
				} else if state.player.hit_points == 0 {
					Some (Outcome { winner: Contender::Boss, mana: state.mana })
				} else {
					for next_state in next_states (state) {
						adder.add (next_state, next_state.mana);
					}
					None
				}
			},
		);

		search.push (State {
			player: input.player,
			boss: input.boss,
			effects: Effects { shield: 0, poison: 0, recharge: 0 },
			turn: Contender::Player,
			difficulty,
			mana: 0,
		}, 0);

		search.flatten ()

	}

	fn next_states (mut state: State) -> ArrayVec <State, 5> {

		if state.effects.shield > 0 {
			state.effects.shield -= 1;
		}

		if state.effects.poison > 0 {
			state.boss.hit_points -= cmp::min (state.boss.hit_points, 3);
			if state.boss.hit_points == 0 { return array_vec! [ state ] }
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
				return array_vec! [ state ];
			}

			if state.difficulty == Difficulty::Hard {
				state.player.hit_points -= 1;
				if state.player.hit_points == 0 {
					return array_vec! [ state ];
				}
			}

			let mut results = ArrayVec::new ();

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
			array_vec! [ state ]

		}

	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	struct Outcome {
		winner: Contender,
		mana: u16,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	struct Effects {
		shield: u8,
		poison: u8,
		recharge: u8,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	struct State {
		player: Player,
		boss: Boss,
		effects: Effects,
		turn: Contender,
		difficulty: Difficulty,
		mana: u16,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	enum Contender { Player, Boss }

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	enum Difficulty { Easy, Hard }

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Input {
		pub player: Player,
		pub boss: Boss,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Player {
		pub hit_points: u16,
		pub mana: u16,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Boss {
		pub hit_points: u16,
		pub damage: u16,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Input> {
			let player = Player { hit_points: 50, mana: 500 };
			if input.len () != 2 { Err ("Invalid input") ?; }
			fn parse_line (line_idx: usize, line: & str, expect: & str) -> GenResult <u16> {
				Parser::wrap (line, |parser| {
					let value = parser.expect (expect) ?.int () ?;
					parser.end () ?;
					Ok (value)
				}).map_parse_err (|_| format! ("Invalid input: line {}: {}", line_idx + 1, line))
			}
			let hit_points = parse_line (0, input [0], "Hit Points: ") ?;
			let damage = parse_line (1, input [1], "Damage: ") ?;
			if hit_points > 100 { Err ("Boss hit points are limited to 100") ?; }
			if damage > 15 { Err ("Boss damage is limited to 15") ?; }
			Ok (Input { player, boss: Boss { hit_points, damage }})
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("491", puzzle.part_one (& [ "Hit Points: 40", "Damage: 8" ]));
		assert_eq_ok! ("787", puzzle.part_one (& [ "Hit Points: 50", "Damage: 8" ]));
		assert_eq_ok! ("1249", puzzle.part_one (& [ "Hit Points: 60", "Damage: 8" ]));
		assert_eq_ok! ("734", puzzle.part_one (& [ "Hit Points: 40", "Damage: 9" ]));
		assert_eq_ok! ("900", puzzle.part_one (& [ "Hit Points: 50", "Damage: 9" ]));
		assert_eq_ok! ("1269", puzzle.part_one (& [ "Hit Points: 60", "Damage: 9" ]));
		assert_eq_ok! ("754", puzzle.part_one (& [ "Hit Points: 40", "Damage: 10" ]));
		assert_eq_ok! ("900", puzzle.part_one (& [ "Hit Points: 50", "Damage: 10" ]));
		assert_eq_ok! ("1309", puzzle.part_one (& [ "Hit Points: 60", "Damage: 10" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("734", puzzle.part_two (& [ "Hit Points: 40", "Damage: 8" ]));
		assert_eq_ok! ("900", puzzle.part_two (& [ "Hit Points: 50", "Damage: 8" ]));
		assert_eq_ok! ("1309", puzzle.part_two (& [ "Hit Points: 60", "Damage: 8" ]));
		assert_eq_ok! ("754", puzzle.part_two (& [ "Hit Points: 40", "Damage: 9" ]));
		assert_eq_ok! ("920", puzzle.part_two (& [ "Hit Points: 50", "Damage: 9" ]));
		assert_eq_ok! ("1309", puzzle.part_two (& [ "Hit Points: 60", "Damage: 9" ]));
		assert_eq_ok! ("794", puzzle.part_two (& [ "Hit Points: 40", "Damage: 10" ]));
		assert_eq_ok! ("1256", puzzle.part_two (& [ "Hit Points: 50", "Damage: 10" ]));
		assert_eq_ok! ("1442", puzzle.part_two (& [ "Hit Points: 60", "Damage: 10" ]));
	}

}
