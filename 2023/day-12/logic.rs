use super::*;

use input::Input;
use input::InputRow;
use input::Spring;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (input)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let input = Input {
		rows: input.rows.iter ()
			.map (|row| InputRow {
				springs: iter::repeat (iter::empty ()
						.chain (iter::once (Spring::Unknown))
						.chain (row.springs.iter ().copied ()))
					.take (5)
					.flatten ()
					.skip (1)
					.collect (),
				groups: iter::repeat (row.groups.iter ().copied ())
					.take (5)
					.flatten ()
					.collect (),
			})
			.collect (),
		params: input.params.clone (),
	};
	calc_result (& input)
}

fn calc_result (input: & Input) -> GenResult <u64> {
	let mut sum = 0;
	for row in & input.rows {
		sum += calc_row (row) ?;
	}
	Ok (sum)
}

fn calc_row (row: & InputRow) -> GenResult <u64> {
	let num_springs = row.springs.len ().pan_u32 ();
	if num_springs.pan_u64 () < row.groups.iter ().map (|& num| num.pan_u64 () + 1).sum::<u64> () - 1 {
		return Err ("Damaged groups won't fit".into ());
	}
	if 128 < num_springs { return Err ("Max number of springs is 128".into ()); }
	let num_groups = row.groups.len ().pan_u32 ();
	let check = Check::new (& row.springs);
	let mut cache: HashMap <(u32, u32), u64> = HashMap::new ();
	for pos in (0 ..= num_springs).rev () {
		let mut state = State::default ();
		state.push_unknown (pos);
		state.push_operational (num_springs - pos);
		if check.matches (state) {
			cache.insert ((num_groups, pos), 1);
		}
	}
	for (group_idx, & group) in row.groups.iter ().enumerate ().rev () {
		let start: u32 = row.groups [ .. group_idx].iter ().map (|& num| num + 1).sum ();
		let end: u32 = row.groups [group_idx + 1 .. ].iter ().map (|& num| num + 1).sum ();
		let group_idx = group_idx.pan_u32 ();
		for pos in (start ..= num_springs - end - group).rev () {
			let mut num = 0;
			let mut state = State::default ();
			if group_idx == 0 { state.push_operational (pos); } else { state.push_unknown (pos); }
			{
				let mut state = state;
				state.push_damaged (group);
				if group_idx + 1 < num_groups { state.push_operational (1); }
				if check.matches (state) {
					if let Some (& cache_num) = cache.get (& (group_idx + 1, state.bits_used)) {
						chk! (num += cache_num) ?;
					}
				}
			}
			if pos < num_springs - group {
				let mut state = state;
				state.push_operational (1);
				if check.matches (state) {
					if let Some (& cache_num) = cache.get (& (group_idx, state.bits_used)) {
						chk! (num += cache_num) ?;
					}
				}
			}
			if 0 < num { cache.insert ((group_idx, pos), num); }
		}
	}
	Ok (cache.get (& (0, 0)).copied ().unwrap_or_default ())
}

#[ derive (Clone, Copy, Debug, Default) ]
struct State {
	damaged: u128,
	operational: u128,
	bits_used: u32,
}

impl State {
	fn push_unknown (& mut self, num: u32) {
		for _ in 0 .. num {
			self.bits_used += 1;
		}
	}
	fn push_damaged (& mut self, num: u32) {
		for _ in 0 .. num {
			self.damaged |= 1 << self.bits_used;
			self.bits_used += 1;
		}
	}
	fn push_operational (& mut self, num: u32) {
		for _ in 0 .. num {
			self.operational |= 1 << self.bits_used;
			self.bits_used += 1;
		}
	}
}

#[ derive (Clone, Copy, Debug) ]
struct Check {
	damaged: u128,
	operational: u128,
}

impl Check {
	fn new (springs: & [Spring]) -> Self {
		let damaged =
			springs.iter ().enumerate ()
				.filter (|& (_, & spring)| matches! (spring, Spring::Damaged))
				.fold (0_u128, |layout, (idx, _)| layout | 1 << idx);
		let operational =
			springs.iter ().enumerate ()
				.filter (|& (_, & spring)| matches! (spring, Spring::Operational))
				.fold (0_u128, |layout, (idx, _)| layout | 1 << idx);
		Self { damaged, operational }
	}
	fn matches (& self, state: State) -> bool {
		assert! (state.damaged & state.operational == 0);
		state.damaged & self.operational == 0 && state.operational & self.damaged == 0
	}
}
