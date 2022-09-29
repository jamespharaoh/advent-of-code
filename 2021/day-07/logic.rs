use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let crabs_map = get_crabs_map (input) ?;
	Ok (
		calc_result (
			get_crabs_range (input),
			(0, 0),
			|& mut (ref mut fuel, ref mut crabs), pos| {
				* fuel += * crabs;
				* crabs += crabs_map.get (& pos).copied ().unwrap_or (0).pan_u64 ();
				Some (* fuel)
			})
	)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let crabs_map = get_crabs_map (input) ?;
	Ok (
		calc_result (
			get_crabs_range (input),
			(0, 0, 0),
			|& mut (ref mut fuel, ref mut incr, ref mut crabs), pos| {
				* incr += * crabs;
				* fuel += * incr;
				* crabs += crabs_map.get (& pos).copied ().unwrap_or (0).pan_u64 ();
				Some (* fuel)
			})
	)
}

fn get_crabs_range (input: & Input) -> RangeInclusive <u16> {
	let min = input.crabs.iter ().copied ().min ().unwrap ();
	let max = input.crabs.iter ().copied ().max ().unwrap ();
	min ..= max
}

fn get_crabs_map (input: & Input) -> GenResult <HashMap <u16, u16>> {
	if input.crabs.is_empty () {
		return Err ("Must provide at least one crab position".into ());
	}
	Ok (
		input.crabs.iter ().copied ()
			.fold (HashMap::new (), |mut all_crabs, crab| {
				all_crabs.entry (crab).and_modify (|num| * num += 1).or_insert (1);
				all_crabs
			})
	)
}

fn calc_result <State, ScanFn> (
	pos_range: RangeInclusive <u16>,
	initial_state: State,
	scan_fn: ScanFn,
) -> u64
	where
		ScanFn: Fn (& mut State, u16) -> Option <u64> + Clone,
		State: Copy {
	iter::zip (
			pos_range.clone ().scan (initial_state, scan_fn.clone ()),
			pos_range.rev ()
				.scan (initial_state, scan_fn)
				.collect::<Vec <_>> ()
				.into_iter ()
				.rev ())
		.map (|(lower, higher)| lower + higher)
		.min ()
		.unwrap ()
}
