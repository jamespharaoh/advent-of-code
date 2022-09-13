//! Advent of Code 2019: Day 18: Many-Worlds Interpretation
//!
//! [https://adventofcode.com/2019/day/18](https://adventofcode.com/2019/day/18)
//!
//! # Input
//!
//! Grid representation of a map, with the following characters:
//!
//! - `.` open space
//! - `#` wall
//! - `@` entrance
//! - `a`-`z` key
//! - `A`-`Z` door
//!
//! # Part one
//!
//! Find the lowest number of steps from the entrance to collect all keys. Doors can only be passed
//! once the corresponding key has been collected.
//!
//! # Part two
//!
//! Replace the entrance with a nine-by-nine grid of squares with entrances at each corner, and the
//! rest as walls. The goal is the same, but there are four actors to track instead of one. Keys
//! collected by any actor can be used by all of them.
//!
//! # Algorithm
//!
//! First we simplify the map, removing any obvious dead ends.
//!
//! Next, we perform path analysis. Find routes from every entrance and key to all other keys which
//! are in reach. Track any keys or doors which are passed through on each path, and the number of
//! steps. We track keys the same as doors because there's no point in passing a key without
//! picking it up, so this makes the subsequent search more efficient.
//!
//! Finally, perform a priority search where each node includes the position of all actors and the
//! keys collected so far. The next nodes at each point are the paths from this point which lead to
//! unclaimed keys and don't pass through any doors or keys which we haven't collected yet.
//!
//! The path-finding part of this algorithm only considers the shortest route, which works fine for
//! the maps supplied. This would not work if the shortest route passes through a door for which we
//! don't have a key, but there is a longer route which goes a different way, assuming this would
//! result in a shorter total number of steps.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_grid as grid;
use aoc_pos as pos;
use aoc_search::*;

mod examples;
pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Many-Worlds Interpretation";
	year = 2019;
	day = 18;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
