//! Advent of Code 2015: Day 21: RPG Simulator 20XX
//!
//! [https://adventofcode.com/2015/day/21](https://adventofcode.com/2015/day/21)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "RPG Simulator 20XX";
	year = 2015;
	day = 21;
	parse = |input| model::Stats::parse (input);
	part_one = |input| Ok::<_, Infallible> (logic::part_one (input));
	part_two = |input| Ok::<_, Infallible> (logic::part_two (input));
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Stats;

	#[ must_use ]
	pub fn part_one (boss: Stats) -> u32 {
		choices ()
			.filter (|& (_, player)| outcome (player, boss))
			.map (|(gold, _)| gold)
			.min ()
			.unwrap ()
	}

	#[ must_use ]
	pub fn part_two (boss: Stats) -> u32 {
		choices ()
			.filter (|& (_, player)| ! outcome (player, boss))
			.map (|(gold, _)| gold)
			.max ()
			.unwrap ()
	}

	fn choices () -> impl Iterator <Item = (u32, Stats)> {
		PLAYER_STATS [0].iter ()
			.cartesian_product (PLAYER_STATS [1].iter ())
			.cartesian_product (PLAYER_STATS [2].iter ())
			.map (|((& a, & b), & c)| (a, b, c))
			.map (|(
					(gld_0, dmg_0, arm_0),
					(gld_1, dmg_1, arm_1),
					(gld_2, dmg_2, arm_2),
				)| (
					gld_0 + gld_1 + gld_2,
					Stats {
						hit_points: 100,
						damage: dmg_0 + dmg_1 + dmg_2,
						armor: arm_0 + arm_1 + arm_2,
					},
				)
			)
	}

	const fn outcome (player: Stats, boss: Stats) -> bool {

		let mut player_hp = player.hit_points;
		let mut boss_hp = boss.hit_points;

		let player_attack = if player.damage > boss.armor {
			player.damage - boss.armor
		} else { 1 };

		let boss_attack = if boss.damage > player.armor {
			boss.damage - player.armor
		} else { 1 };

		loop {

			if boss_hp <= player_attack { return true }
			boss_hp -= player_attack;

			if player_hp <= boss_attack { return false }
			player_hp -= boss_attack;

		}

	}

	const PLAYER_STATS: & [& [(u32, u32, u32)]] = & [

		// weapons
		& [ (8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0) ],

		// armor
		& [ (0, 0, 0), (13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5) ],

		// rings
		& [
			(0, 0, 0), (25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3),
			(75, 3, 0), (125, 4, 0), (45, 1, 1), (65, 1, 2), (105, 1, 3), (150, 5, 0), (70, 2, 1),
			(90, 2, 2), (130, 2, 3), (120, 3, 1), (140, 3, 2), (180, 3, 3), (60, 0, 3),
			(100, 0, 4), (120, 0, 5),
		],

	];

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;

	#[ derive (Clone, Copy, Debug) ]
	pub struct Stats {
		pub hit_points: u32,
		pub damage: u32,
		pub armor: u32,
	}

	impl Stats {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			if input.len () != 3 { Err ("Invalid input") ?; }
			fn parse_line (line_idx: usize, line: & str, expect: & str) -> GenResult <u32> {
				Parser::wrap (line, |parser| {
					let value = parser.expect (expect) ?.int () ?;
					parser.end () ?;
					Ok (value)
				}).map_parse_err (|_, _| format! ("Invalid input: line {}: {}", line_idx + 1, line))
			}
			let hit_points = parse_line (0, input [0], "Hit Points: ") ?;
			let damage = parse_line (1, input [1], "Damage: ") ?;
			let armor = parse_line (2, input [2], "Armor: ") ?;
			Ok (Self { hit_points, damage, armor })
		}
	}

}

#[ cfg (test) ]
mod tests {

	use super::*;

	const EXAMPLES: & [& [& str]] = & [
		& [ "Hit Points: 100", "Damage: 3", "Armor: 3" ],
		& [ "Hit Points: 100", "Damage: 3", "Armor: 4" ],
		& [ "Hit Points: 100", "Damage: 3", "Armor: 5" ],
		& [ "Hit Points: 100", "Damage: 4", "Armor: 3" ],
		& [ "Hit Points: 100", "Damage: 4", "Armor: 4" ],
		& [ "Hit Points: 100", "Damage: 4", "Armor: 5" ],
		& [ "Hit Points: 100", "Damage: 5", "Armor: 3" ],
		& [ "Hit Points: 100", "Damage: 5", "Armor: 4" ],
		& [ "Hit Points: 100", "Damage: 5", "Armor: 5" ],
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("23", puzzle.part_one (EXAMPLES [0]));
		assert_eq_ok! ("38", puzzle.part_one (EXAMPLES [1]));
		assert_eq_ok! ("39", puzzle.part_one (EXAMPLES [2]));
		assert_eq_ok! ("38", puzzle.part_one (EXAMPLES [3]));
		assert_eq_ok! ("53", puzzle.part_one (EXAMPLES [4]));
		assert_eq_ok! ("59", puzzle.part_one (EXAMPLES [5]));
		assert_eq_ok! ("53", puzzle.part_one (EXAMPLES [6]));
		assert_eq_ok! ("71", puzzle.part_one (EXAMPLES [7]));
		assert_eq_ok! ("79", puzzle.part_one (EXAMPLES [8]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("33", puzzle.part_two (EXAMPLES [0]));
		assert_eq_ok! ("58", puzzle.part_two (EXAMPLES [1]));
		assert_eq_ok! ("108", puzzle.part_two (EXAMPLES [2]));
		assert_eq_ok! ("58", puzzle.part_two (EXAMPLES [3]));
		assert_eq_ok! ("108", puzzle.part_two (EXAMPLES [4]));
		assert_eq_ok! ("133", puzzle.part_two (EXAMPLES [5]));
		assert_eq_ok! ("108", puzzle.part_two (EXAMPLES [6]));
		assert_eq_ok! ("133", puzzle.part_two (EXAMPLES [7]));
		assert_eq_ok! ("158", puzzle.part_two (EXAMPLES [8]));
	}

}
