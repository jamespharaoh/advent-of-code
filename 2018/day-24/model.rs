use super::*;

pub type Val = u32;

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Group {
	pub num_units: Val,
	pub hit_points: Val,
	pub weaknesses: Vec <AttackType>,
	pub immunities: Vec <AttackType>,
	pub attack_damage: Val,
	pub attack_type: AttackType,
	pub initiative: Val,
}

struct_parser_display! {
	Group { num_units, hit_points, weaknesses, immunities, attack_damage, attack_type, initiative } = [
		num_units, " units each with ", hit_points, " hit points ",
		(weaknesses, immunities) {
			display_type = (& [AttackType], & [AttackType]);
			(weaknesses, immunities) if (! weaknesses.is_empty () && ! immunities.is_empty ()) = [
				"(weak to ", @delim ", " weaknesses, "; ",
				"immune to ", @delim ", " immunities, ") ",
			],
			(weaknesses, immunities) if (false) = [
				"(immune to ", @delim ", " immunities, "; ",
				"weak to ", @delim ", " weaknesses, ") ",
			],
			(weaknesses, immunities) if (! weaknesses.is_empty ()) = [
				"(weak to ", @delim ", " weaknesses, ") ",
				@parse immunities { Vec::new () },
			],
			(weaknesses, immunities) if (! immunities.is_empty ()) = [
				@parse weaknesses { Vec::new () },
				"(immune to ", @delim ", " immunities, ") ",
			],
			(weaknesses, immunities) = [
				@parse weaknesses { Vec::new () },
				@parse immunities { Vec::new () },
			],
		},
		"with an attack that does ", attack_damage, " ", attack_type, " damage ",
		"at initiative ", initiative,
	]
}

impl Group {

	#[ must_use ]
	pub const fn effective_power (& self) -> Val {
		self.num_units * self.attack_damage
	}

	#[ must_use ]
	pub fn has_weakness (& self, attack_type: AttackType) -> bool {
		self.weaknesses.contains (& attack_type)
	}

	#[ must_use ]
	pub fn has_immunity (& self, attack_type: AttackType) -> bool {
		self.immunities.contains (& attack_type)
	}

}

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum AttackType {
		Bludgeoning = "bludgeoning",
		Cold = "cold",
		Fire = "fire",
		Radiation = "radiation",
		Slashing = "slashing",
	}
}
