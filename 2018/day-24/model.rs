use super::*;

pub type Val = u32;

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Group {
	pub num_units: Val,
	pub hit_points: Val,
	pub weaknesses: ArrayVec <AttackType, 4>,
	pub immunities: ArrayVec <AttackType, 4>,
	pub attack_damage: Val,
	pub attack_type: AttackType,
	pub initiative: Val,
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

impl Display for Group {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{num_units} units each with {hit_points} hit points ",
			num_units = self.num_units,
			hit_points = self.hit_points) ?;
		if ! self.weaknesses.is_empty () && ! self.immunities.is_empty () {
			write! (formatter,
				"(weak to {weaknesses}; immune to {immunities}) ",
				weaknesses = DisplayDelim::new (", ", & self.weaknesses),
				immunities = DisplayDelim::new (", ", & self.immunities)) ?;
		} else if ! self.weaknesses.is_empty () {
			write! (formatter,
				"(weak to {weaknesses}) ",
				weaknesses = DisplayDelim::new (", ", & self.weaknesses)) ?;
		} else if ! self.immunities.is_empty () {
			write! (formatter,
				"(immune to {immunities}) ",
				immunities = DisplayDelim::new (", ", & self.immunities)) ?;
		}
		write! (formatter,
			"with an attack that does {attack_damage} {attack_type} damage at initiative {initiative}",
			attack_damage = self.attack_damage,
			attack_type = self.attack_type,
			initiative = self.initiative) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Group {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, num_units, " units each with ", hit_points, " hit points ");
		let (weaknesses, immunities): (Vec <AttackType>, Vec <AttackType>) = parser.any ()
			.of (|parser| {
				parse! (parser, "(weak to ", @delim ", " weaknesses, ") ");
				Ok ((weaknesses, Vec::new ()))
			})
			.of (|parser| {
				parse! (parser, "(immune to ", @delim ", " immunities, ") ");
				Ok ((Vec::new (), immunities))
			})
			.of (|parser| {
				parse! (parser,
					"(weak to ", @delim ", " weaknesses, "; ",
					"immune to ", @delim ", " immunities, ") ",
				);
				Ok ((weaknesses, immunities))
			})
			.of (|parser| {
				parse! (parser,
					"(immune to ", @delim ", " immunities, "; ",
					"weak to ", @delim ", " weaknesses, ") ",
				);
				Ok ((weaknesses, immunities))
			})
			.of (|_parser| {
				Ok ((Vec::new (), Vec::new ()))
			})
			.done () ?;
		if weaknesses.len () > 4 { return Err (parser.err ()) }
		let weaknesses = weaknesses.into_iter ().sorted ().dedup ().collect ();
		if immunities.len () > 4 { return Err (parser.err ()) }
		let immunities = immunities.into_iter ().sorted ().dedup ().collect ();
		parse! (parser,
			"with an attack that does ", attack_damage, " ", attack_type, " damage ",
			"at initiative ", initiative,
		);
		Ok (Self {
			num_units,
			hit_points,
			weaknesses,
			immunities,
			attack_damage,
			attack_type,
			initiative,
		})
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
