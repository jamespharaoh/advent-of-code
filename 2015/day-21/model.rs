//! Data structures and types

use super::*;

#[ derive (Clone, Copy, Debug) ]
pub struct Stats {
	pub hit_points: u32,
	pub damage: u32,
	pub armor: u32,
}

struct_parser_display! {
	Stats { hit_points, damage, armor } = [
		"Hit Points: ", hit_points, "\n",
		"Damage: ", damage, "\n",
		"Armor: ", armor,
	]
}
