use super::*;

use input::Blueprint;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mins = input.params.minutes_one;
	input.blueprints.iter ()
		.map (|blueprint| GenOk (chk! (blueprint.id
			* calc_blueprint (blueprint, mins, input.params.max_iters) ?) ?))
		.try_fold (0, |sum, val| Ok (chk! (sum + val ?) ?))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mins = input.params.minutes_two;
	input.blueprints.iter ()
		.take (3)
		.map (|blueprint| calc_blueprint (blueprint, mins, input.params.max_iters))
		.try_fold (1, |prod, val| Ok (chk! (prod * val ?) ?))
}

pub fn calc_blueprint (
	blueprint: & Blueprint,
	num_minutes: u32,
	max_iters: u64,
) -> GenResult <u32> {
	let mut states = vec! [
		(State::START, num_minutes, Material::Ore),
		(State::START, num_minutes, Material::Clay),
	];
	let mut max_geodes = 0;
	let mut num_iters = 0;
	while let Some ((state, rem_minutes, next_robot)) = states.pop () {
		if max_iters <= num_iters {
			return Err ("Max iterations exceeded".into ());
		}
		num_iters += 1;
		if rem_minutes == 0 {
			max_geodes = cmp::max (max_geodes, state.materials.geode);
			continue;
		}
		let prev_materials = state.materials;
		let mut state = state.increment () ?;
		match next_robot {
			Material::Ore => {
				if blueprint.ore_cost_ore <= prev_materials.ore {
					chk! (state.materials.ore -= blueprint.ore_cost_ore) ?;
					chk! (state.robots.ore += 1) ?;
					add_states (blueprint, & mut states, state, rem_minutes - 1);
				} else {
					states.push ((state, rem_minutes - 1, next_robot));
				}
			},
			Material::Clay => {
				if blueprint.clay_cost_ore <= prev_materials.ore {
					chk! (state.materials.ore -= blueprint.clay_cost_ore) ?;
					chk! (state.robots.clay += 1) ?;
					add_states (blueprint, & mut states, state, rem_minutes - 1);
				} else {
					states.push ((state, rem_minutes - 1, next_robot));
				}
			},
			Material::Obsidian => {
				if blueprint.obsidian_cost_ore <= prev_materials.ore
						&& blueprint.obsidian_cost_clay <= prev_materials.clay {
					chk! (state.materials.ore -= blueprint.obsidian_cost_ore) ?;
					chk! (state.materials.clay -= blueprint.obsidian_cost_clay) ?;
					chk! (state.robots.obsidian += 1) ?;
					add_states (blueprint, & mut states, state, rem_minutes - 1);
				} else {
					states.push ((state, rem_minutes - 1, next_robot));
				}
			},
			Material::Geode => {
				if blueprint.geode_cost_ore <= prev_materials.ore
						&& blueprint.geode_cost_obsidian <= prev_materials.obsidian {
					chk! (state.materials.ore -= blueprint.geode_cost_ore) ?;
					chk! (state.materials.obsidian -= blueprint.geode_cost_obsidian) ?;
					chk! (state.robots.geode += 1) ?;
					add_states (blueprint, & mut states, state, rem_minutes - 1);
				} else {
					states.push ((state, rem_minutes - 1, next_robot));
				}
			},
		}
	}
	Ok (max_geodes)
}

fn add_states (
	blueprint: & Blueprint,
	states: & mut Vec <(State, u32, Material)>,
	state: State,
	rem_minutes: u32,
) {
	if 0 < state.robots.ore
			&& (state.robots.ore < blueprint.ore_cost_ore
				|| state.robots.ore < blueprint.clay_cost_ore
				|| state.robots.ore < blueprint.obsidian_cost_ore
				|| state.robots.ore < blueprint.geode_cost_ore) {
		states.push ((state, rem_minutes, Material::Ore));
	}
	if 0 < state.robots.ore
			&& state.robots.clay < blueprint.obsidian_cost_clay {
		states.push ((state, rem_minutes, Material::Clay));
	}
	if 0 < state.robots.ore && 0 < state.robots.clay
			&& state.robots.obsidian < blueprint.geode_cost_obsidian {
		states.push ((state, rem_minutes, Material::Obsidian));
	}
	if 0 < state.robots.ore && 0 < state.robots.obsidian {
		states.push ((state, rem_minutes, Material::Geode));
	}
}

#[ derive (Debug, Clone, Copy) ]
enum Material { Ore, Clay, Obsidian, Geode }

#[ derive (Debug, Clone, Copy, Eq, Hash, PartialEq) ]
struct State {
	materials: Materials,
	robots: Materials,
}

impl State {

	fn increment (mut self) -> GenResult <Self> {
		chk! (self.materials.ore += self.robots.ore) ?;
		chk! (self.materials.clay += self.robots.clay) ?;
		chk! (self.materials.obsidian += self.robots.obsidian) ?;
		chk! (self.materials.geode += self.robots.geode) ?;
		Ok (self)
	}

	const START: Self = Self {
		materials: Materials { ore: 0, clay: 0, obsidian: 0, geode: 0 },
		robots: Materials { ore: 1, clay: 0, obsidian: 0, geode: 0 },
	};

}

#[ derive (Debug, Clone, Copy, Eq, Hash, PartialEq) ]
struct Materials {
	ore: u32,
	clay: u32,
	obsidian: u32,
	geode: u32,
}
