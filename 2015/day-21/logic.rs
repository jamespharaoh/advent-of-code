//! Logic for solving the puzzles.

use super::*;

use input::Input;
use model::Stats;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		iter_choices ()
			.sorted_by_key (|& (gold, _)| gold)
			.find (|& (_, player)| calc_outcome (player, input.stats))
			.map (|(gold, _)| gold)
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		iter_choices ()
			.sorted_by_key (|& (gold, _)| cmp::Reverse (gold))
			.find (|& (_, player)| ! calc_outcome (player, input.stats))
			.map (|(gold, _)| gold)
			.ok_or ("No solution found") ?
	)
}

fn iter_choices () -> impl Iterator <Item = (u32, Stats)> {
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

const fn calc_outcome (player: Stats, boss: Stats) -> bool {

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
