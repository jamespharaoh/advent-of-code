//! Advent of Code 2015: Day 22: Wizard Simulator 20XX
//!
//! [https://adventofcode.com/2015/day/22](https://adventofcode.com/2015/day/22)
//!
//! # Input
//!
//! Boss stats for a simple deterministic turn-based battle, inlcuding hit points and damage. For
//! example:
//!
//! ```text
//! Hit Points: 100
//! Damage: 10
//! ```
//!
//! # Part one
//!
//! Work out the minimum amount of "mana" that can be spent to win the battle. In the battle each
//! participant takes turns, starting with the player. The player must choose one of five spells
//! to cast, or loses immediately if they don't have sufficient mana. The options are:
//!
//! * Spend `53` mana to cause `4` damage.
//! * Spend `73` mana to cause `2` damage and to heal `2` hit points.
//! * Spend `113` mana to increase the player's armour by `7` for six turns (three attacks).
//! * Spend `173` mana to cause `3` damage at the start of the next six turns.
//! * Spend `229` mana to gain `101` mana at the start of the next five turns.
//!
//! The multi-turn spells cannot be repeated until their effect has worn off. When it is the
//! boss's turn they will cause the amount of damage indicated in the puzzle input. Damage is
//! subtracted from the opponent's hit points, minus any amour from the player's spell, or one is
//! subtracted if the amount would be less.
//!
//! The first participant to reach zero hit points loses, or the player if they have no mana left
//! to cast any spell.
//!
//! # Part two
//!
//! Same as part one but the player loses one hit point at the very start of every turn.
//!
//! # Algorithm
//!
//! This uses a [`PrioritySearch`] with the amount of mana spent as the priority. Each node is
//! a combination of the player and boss's stats, the spells which are continuing to have an
//! effect and for how long, and whose turn it is. The total amount of mana spent is the distance
//! or priority.
//!
//! To work out the next state, we take the existing state and apply any effects. If it is the
//! player's turn, we then add a node for every spell which can be cast, or set hit points to zero
//! if we can't cast a spell. For the boss's turn there is only one type of attack so we simply
//! apply that.
//!
//! The first state we encounter which shows the player winning is the one which spends the least
//! amount of mana, since we use that as our priority.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_search::prelude::*;
use aoc_stvec::prelude::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Wizard Simulator 20XX";
	year = 2015;
	day = 22;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
