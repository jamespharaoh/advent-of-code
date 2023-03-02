use super::*;

use input::Input;
use model::Coord;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut ranges = Vec::new ();
	let mut ranges_temp = Vec::new ();
	row_ranges (input, input.params.check_row, & mut ranges, & mut ranges_temp) ?;
	let check_beacons: Vec <Coord> =
		input.sensors.iter ()
			.filter (|sensor| sensor.beacon.y == input.params.check_row)
			.map (|sensor| sensor.beacon.x)
			.sorted ()
			.dedup_consecutive ()
			.collect ();
	Ok (
		ranges.iter ()
			.map (|& (start, end)| {
				let size = Coord::abs_diff (end, start);
				let num_beacons = check_beacons.iter ()
					.filter (|&& beacon_x| (start .. end).contains (& beacon_x))
					.count ()
					.pan_u32 ();
				size - num_beacons
			})
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut ranges = Vec::new ();
	let mut ranges_temp = Vec::new ();
	let mut found = None;
	for row in Coord::ZERO ..= input.params.search_size {
		row_ranges (input, row, & mut ranges, & mut ranges_temp) ?;
		for (start, end) in ranges.iter ().copied () {
			let found_now = if start == Coord::ONE {
				Pos::new (row, Coord::ZERO)
			} else if (Coord::ZERO .. input.params.search_size).contains (& end) {
				Pos::new (row, end)
			} else { continue };
			if found.is_some () {
				return Err ("Multiple solutions found".into ());
			}
			found = Some (found_now);
		}
	}
	let found = found.ok_or ("No solution found") ?;
	Ok (found.x.pan_u64 () * 4_000_000 + found.y.pan_u64 ())
}

fn row_ranges (
	input: & Input,
	check_row: Coord,
	ranges: & mut Vec <(Coord, Coord)>,
	ranges_temp: & mut Vec <(Coord, Coord)>,
) -> GenResult <()> {
	ranges.clear ();
	for sensor in & input.sensors {
		let radius = chk! (
			chk! (sensor.sensor.x - sensor.beacon.x) ?.checked_abs ().unwrap ()
				+ chk! (sensor.sensor.y - sensor.beacon.y) ?.checked_abs ().unwrap ()) ?;
		let offset = chk! (check_row - sensor.sensor.y) ?.checked_abs ().unwrap ();
		let mut new_start = sensor.sensor.x - radius + offset;
		let mut new_end = chk! (sensor.sensor.x + Coord::ONE + radius - offset) ?;
		if new_end <= new_start { continue }
		ranges_temp.clear ();
		for & (range_start, range_end) in & * ranges {
			if range_end < new_start || new_end < range_start {
				ranges_temp.push ((range_start, range_end));
			} else {
				new_start = cmp::min (new_start, range_start);
				new_end = cmp::max (new_end, range_end);
			}
		}
		ranges_temp.push ((new_start, new_end));
		mem::swap (ranges, ranges_temp);
	}
	Ok (())
}
