//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Group;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	sanity_check (input) ?;
	let (num_0, num_1) = calc_result (input, 0) ?;
	Ok (num_0 + num_1)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	sanity_check (input) ?;
	let mut boost_min = 0;
	let mut boost_max = input.params.max_boost;
	let mut num_units = 0;
	while boost_min == boost_max || boost_min + 1 < boost_max {
		let boost = (boost_min + boost_max) / 2;
		let (num_0, num_1) = calc_result (input, boost) ?;
		if num_0 > 0 && num_1 == 0 {
			boost_max = boost;
			num_units = num_0;
		} else {
			boost_min = boost;
		}
	}
	if num_units == 0 { return Err ("Max boost exceeded".into ()) }
	Ok (num_units)
}

fn sanity_check (input: & Input) -> GenResult <()> {
	for group in iter::empty ()
			.chain (& input.immune_system)
			.chain (& input.infection) {
		if group.num_units < 1 || group.hit_points < 1 || group.attack_damage < 1 {
			return Err ("Each group must have at least one unit, one hit point, and one attack \
				damage".into ());
		}
	}
	Ok (())
}

fn calc_result (input: & Input, boost: u32) -> GenResult <(u32, u32)> {
	let mut immune_system = input.immune_system.clone ();
	for group in & mut immune_system {
		group.attack_damage += boost;
	}
	let mut infection = input.infection.clone ();
	let mut num_rounds = 0;
	loop {
		if num_rounds == input.params.max_rounds { return Err ("Max rounds exceeded".into ()) }
		let progress = one_round (& mut immune_system, & mut infection);
		if ! progress { break }
		num_rounds += 1;
	}
	let num_0 = immune_system.iter ().map (|group| group.num_units).sum ();
	let num_1 = infection.iter ().map (|group| group.num_units).sum ();
	Ok ((num_0, num_1))
}

fn one_round (left: & mut Vec <Group>, right: & mut Vec <Group>) -> bool {
	#[ derive (Clone, Copy, Debug) ]
	enum Side { Left, Right }
	use Side::{ Left, Right };
	let attacks: Vec <(Side, usize, usize)> = iter::empty ()
		.chain (choose_targets (left, right).into_iter ()
			.map (|(att_idx, def_idx)| (Left, att_idx, def_idx)))
		.chain (choose_targets (right, left).into_iter ()
			.map (|(att_idx, def_idx)| (Right, att_idx, def_idx)))
		.sorted_by_key (|& (side, att_idx, _)|
			cmp::Reverse (match side {
				Left => left [att_idx].initiative,
				Right => right [att_idx].initiative,
			}))
		.collect ();
	let mut progress = false;
	for (side, att_idx, def_idx) in attacks {
		let (att_group, def_group) = match side {
			Left => (& mut left [att_idx], & mut right [def_idx]),
			Right => (& mut right [att_idx], & mut left [def_idx]),
		};
		let damage = if def_group.has_weakness (att_group.attack_type) {
			att_group.effective_power () * 2
		} else {
			att_group.effective_power ()
		};
		let lost_units = cmp::min (damage / def_group.hit_points, def_group.num_units);
		if lost_units > 0 { progress = true; }
		def_group.num_units -= lost_units;
	}
	left.retain (|group| group.num_units > 0);
	right.retain (|group| group.num_units > 0);
	progress
}

fn choose_targets (
	att_groups: & [Group],
	def_groups: & [Group],
) -> Vec <(usize, usize)> {
	let mut result = Vec::new ();
	let att_groups: Vec <(usize, & Group)> =
		att_groups.iter ()
			.enumerate ()
			.sorted_by_key (|& (_, group)|
				cmp::Reverse ((group.effective_power (), group.initiative)))
			.collect ();
	let mut def_groups: Vec <(usize, & Group)> =
		def_groups.iter ()
			.enumerate ()
			.collect ();
	for (att_idx, att_group) in att_groups.iter ().copied () {
		if let Some (def_idx) =
			def_groups.iter ().copied ()
				.filter (|& (_, def_group)|
					! def_group.has_immunity (att_group.attack_type))
				.map (|(def_idx, def_group)| (
					def_idx,
					def_group,
					if def_group.has_weakness (att_group.attack_type) {
						att_group.effective_power () * 2
					} else {
						att_group.effective_power ()
					}
				))
				.max_by_key (|& (_, def_group, damage)| (
					damage,
					def_group.effective_power (),
					def_group.initiative,
				))
				.map (|(def_idx, _, _)| def_idx) {
			result.push ((att_idx, def_idx));
			def_groups.retain (|& (idx, _)| idx != def_idx);
		}
	}
	result
}
