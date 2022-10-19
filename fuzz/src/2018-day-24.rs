#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2018::day_24::*;
use aoc_common::*;
use aoc_fuzz::*;

use input::Input;
use model::AttackType;
use model::Group;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.max_rounds.bounds_assign ( ..= 800);
		input.params.max_boost.bounds_assign ( ..= 40);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});

aoc_fuzz_mutator! {

	transform_lifetimes = <'inp>;
	input_type = Input;

	transform add (1000) = |input, rng| {
		let mut group = Group {
			num_units: rng.gen_range (1 ..= 9999),
			hit_points: rng.gen_range (1 ..= 9999),
			weaknesses: Vec::new (),
			immunities: Vec::new (),
			attack_damage: rng.gen_range (1 ..= 999),
			attack_type: AttackType::VARIANTS.choose (rng).copied ().unwrap (),
			initiative: rng.gen_range (1 ..= 99),
		};
		for attack_type in AttackType::VARIANTS {
			if ! rng.gen_bool (0.5) { continue }
			let target = pick_one! (rng, & mut group.weaknesses, & mut group.immunities);
			if ! target.contains (& attack_type) { target.push (attack_type); }
		}
		let groups = pick_one! (rng, & mut input.immune_system, & mut input.infection);
		let idx = rng.gen_range (0 ..= groups.len ());
		groups.insert (idx, group);
	}

	transform modify (1000) = |input, rng| {
		let groups = pick_one! (rng, & mut input.immune_system, & mut input.infection);
		if groups.is_empty () { return Some (()) }
		let group_idx = rng.gen_range (0 .. groups.len ());
		let group = & mut groups [group_idx];
		loop {
			pick_one! (
				rng, { // change num units
					group.num_units = rng.gen_range (1 ..= 9999);
				}, { // change hit points
					group.hit_points = rng.gen_range (1 ..= 9999);
				}, { // remove weakness
					if group.weaknesses.is_empty () { continue }
					let weak_idx = rng.gen_range (0 .. group.weaknesses.len ());
					group.weaknesses.remove (weak_idx);
				}, { // add weakness
					let attack_type = AttackType::VARIANTS.choose (rng).copied ().unwrap ();
					if group.weaknesses.contains (& attack_type) { continue }
					group.weaknesses.push (attack_type);
				}, { // remove immunity
					if group.immunities.is_empty () { continue }
					let weak_idx = rng.gen_range (0 .. group.immunities.len ());
					group.immunities.remove (weak_idx);
				}, { // add immunity
					let attack_type = AttackType::VARIANTS.choose (rng).copied ().unwrap ();
					if group.immunities.contains (& attack_type) { continue }
					group.immunities.push (attack_type);
				}, { // change damage
					group.attack_damage = rng.gen_range (1 ..= 999);
				}, { // change attack type
					group.attack_type = AttackType::VARIANTS.choose (rng).copied ().unwrap ();
				}, { // change initiative
					group.initiative = rng.gen_range (1 ..= 99);
				},
			);
			break;
		}
	}

	pub transform remove (1000) = |input, rng| {
		let groups =
			if rng.gen_bool (0.5) { & mut input.immune_system }
			else { & mut input.infection };
		if groups.is_empty () { return Some (()) }
		let idx = rng.gen_range (0 .. groups.len ());
		groups.remove (idx);
	}

	transform shuffle (1) = |input, rng| {
		let groups =
			if rng.gen_bool (0.5) { & mut input.immune_system }
			else { & mut input.infection };
		groups.shuffle (rng);
	}

	transform sort (1) = |input, rng| {
		let groups =
			if rng.gen_bool (0.5) { & mut input.immune_system }
			else { & mut input.infection };
		groups.sort ();
	}

}
