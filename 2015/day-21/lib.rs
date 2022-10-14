//! Advent of Code 2015: Day 21: RPG Simulator 20XX
//!
//! [https://adventofcode.com/2015/day/21](https://adventofcode.com/2015/day/21)
//!
//! # Input
//!
//! Boss stats for a simple turn based fight game, including hit points, damage and armour. For
//! example:
//!
//! ```text
//! Hit Points: 100
//! Damage: 10
//! Armor: 5
//! ```
//!
//! # Part one
//!
//! What is the smallest amount of "gold" the player can spend and still defeat this boss? They
//! can choose to spend any combination of the following:
//!
//! * Exactly one of: damage `+4` for `8`, damage `+5` for `10`, damage `+6` for `25`, damage `+7`
//!   for `40`, damage `+8` for `74`
//! * Zero or one of: armour `+1` for `13`, armour `+2` for `31`, armour `+3` for `53`, armour
//!   `+4` for `75`, armour `+5` for `102`.
//! * Zero, one or two of: damage `+1` for `25`, damage `+2` for `50`, damage `+3` for `100`,
//!   armour `+1` for `20`, armour `+2` for `40`, armour `+3` for `60`.
//!
//! To determine the winner the player and the boss take turns, player first, subtracting their
//! total damage, minus the opponent's armour, from the opponent's hit points. If the amount to be
//! subtracted is less than one then one is subtracted instead. The first party to reach zero hit
//! points is the loser.
//!
//! # Part two
//!
//! Same as part one, but what is the *most* that can be spent which results in a loss for the
//! player.
//!
//! # Algorithm
//!
//! We simplify the three types of item into a single table for each containing every option:
//!
//! * With exactly one option this list is the same
//! * To make this optional we add a free option with no stats
//! * We include zero items, similar to the previous item, plus each option alone, plus each
//!   combination of two options.
//!
//! We then iterate over the cartesian product of these three dimensions working out the resulting
//! stats and cost for the player. This is sorted according to the cost, in ascending order for
//! part one and descending order for part two. Then we simulate the battle as described until we
//! find a win for part one, or a lose for part two.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "RPG Simulator 20XX";
	year = 2015;
	day = 21;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
