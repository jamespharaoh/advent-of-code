use super::*;

use input::Input;
use input::InputMap;

pub fn part_one (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let mut locations = Vec::new ();
	for & seed in & input.seeds {
		let seed = seed.pan_u64 ();
		let soil = apply_maps_one (& input.seed_to_soil, seed);
		let fertilizer = apply_maps_one (& input.soil_to_fertilizer, soil);
		let water = apply_maps_one (& input.fertilizer_to_water, fertilizer);
		let light = apply_maps_one (& input.water_to_light, water);
		let temperature = apply_maps_one (& input.light_to_temperature, light);
		let humidity = apply_maps_one (& input.temperature_to_humidity, temperature);
		let location = apply_maps_one (& input.humidity_to_location, humidity);
		locations.push (location);
	}
	Ok (locations.into_iter ().min ().unwrap ())
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let seeds: Vec <(u64, u64)> =
		input.seeds.iter ().arrays ()
			.map (|[ & start, & len ]| (start.pan_u64 (), start.pan_u64 () + len.pan_u64 ()))
			.collect ();
	let soils = apply_maps_multi (& input.seed_to_soil, & seeds);
	let fertilizers = apply_maps_multi (& input.soil_to_fertilizer, & soils);
	let waters = apply_maps_multi (& input.fertilizer_to_water, & fertilizers);
	let lights = apply_maps_multi (& input.water_to_light, & waters);
	let temperatures = apply_maps_multi (& input.light_to_temperature, & lights);
	let humiditys = apply_maps_multi (& input.temperature_to_humidity, & temperatures);
	let locations = apply_maps_multi (& input.humidity_to_location, & humiditys);
	Ok (locations.into_iter ().map (|(start, _)| start).min ().unwrap ())
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.seeds.is_empty () {
		return Err ("Input must have seeds".into ());
	}
	if input.seeds.len () & 1 != 0 {
		return Err ("Input must have an even number of seeds".into ());
	}
	Ok (())
}

fn apply_maps_one (maps: & [InputMap], val: u64) -> u64 {
	for map in maps {
		if (map.src.pan_u64 () .. map.src.pan_u64 () + map.len.pan_u64 ()).contains (& val) {
			return val - map.src.pan_u64 () + map.dest.pan_u64 ();
		}
	}
	val
}

fn apply_maps_multi (maps: & [InputMap], ranges: & [(u64, u64)]) -> Vec <(u64, u64)> {
	let mut ranges = ranges.to_vec ();
	let mut result = Vec::new ();
	'OUTER: while let Some ((range_start, range_end)) = ranges.pop () {
		for map in maps {
			let map_start = map.src.pan_u64 ();
			let map_end = map_start + map.len.pan_u64 ();
			if range_start < map_end && map_start < range_end {
				let match_start = cmp::max (map_start, range_start);
				let match_end = cmp::min (map_end, range_end);
				result.push ((
					match_start - map.src.pan_u64 () + map.dest.pan_u64 (),
					match_end - map.src.pan_u64 () + map.dest.pan_u64 (),
				));
				if range_start < match_start {
					ranges.push ((range_start, match_start));
				}
				if match_end < range_end {
					ranges.push ((match_end, range_end));
				}
				continue 'OUTER;
			}
		}
		result.push ((range_start, range_end));
	}
	result
}
