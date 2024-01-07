use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub blueprints: Vec <Blueprint>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { blueprints, params } = [
		params,
		@lines blueprints,
	]
}

#[ derive (Clone, Debug) ]
pub struct Blueprint {
	pub id: u32,
	pub ore_cost_ore: u32,
	pub clay_cost_ore: u32,
	pub obsidian_cost_ore: u32,
	pub obsidian_cost_clay: u32,
	pub geode_cost_ore: u32,
	pub geode_cost_obsidian: u32,
}

struct_parser_display! {
	Blueprint {
		id,
		ore_cost_ore,
		clay_cost_ore,
		obsidian_cost_ore,
		obsidian_cost_clay,
		geode_cost_ore,
		geode_cost_obsidian,
	} = [
		"Blueprint ", id, ": ",
		"Each ore robot costs ", ore_cost_ore, " ore. ",
		"Each clay robot costs ", clay_cost_ore, " ore. ",
		"Each obsidian robot costs ", obsidian_cost_ore, " ore and ", obsidian_cost_clay, " clay. ",
		"Each geode robot costs ", geode_cost_ore, " ore and ", geode_cost_obsidian, " obsidian.",
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters: u64 = ("MAX_ITERS=", 100_000_000_000, 0 .. ),
		pub minutes_one: u32 = ("MINUTES_ONE=", 24, 1 .. ),
		pub minutes_two: u32 = ("MINUTES_TWO=", 32, 1 .. ),
	}
}
