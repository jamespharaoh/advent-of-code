//! Representation of the puzzle input, etc.

use super::*;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Player {
	pub hit_points: u8,
	pub mana: u16,
}

impl Default for Player {
	fn default () -> Self {
		Self {
			hit_points: 50,
			mana: 500,
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Boss {
	pub hit_points: u8,
	pub damage: u8,
}

struct_parser_display! {
	Boss { hit_points, damage } = [
		"Hit Points: ", hit_points = 1 ..= 100, "\n",
		"Damage: ", damage = 1 ..= 15,
	]
}
