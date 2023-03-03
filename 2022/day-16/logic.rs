use super::*;

use input::Input;
use model::Volcano;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let volcano = Volcano::build (input) ?;
	let plans = build_plans (input, & volcano, 30) ?;
	Ok (plans.iter ().map (|& (_, released)| released).max ().unwrap ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let volcano = Volcano::build (input) ?;
	let plans = build_plans (input, & volcano, 26) ?;
	if input.params.max_plans.pan_usize () < plans.len () {
		return Err ("Max complexity exceeded".into ());
	}
	let mut best_released = 0;
	for plan_0_idx in 0 .. plans.len () - 1 {
		let (plan_0_open, plan_0_released) = plans [plan_0_idx];
		for plan_1_idx in plan_0_idx + 1 .. plans.len () {
			let (plan_1_open, plan_1_released) = plans [plan_1_idx];
			if plan_0_open & plan_1_open != 0 { continue }
			best_released = cmp::max (best_released, plan_0_released + plan_1_released);
		}
	}
	Ok (best_released)
}

fn check_input (input: & Input) -> GenResult <()> {
	let valve_names: HashSet <InpStr> =
		input.valves.iter ()
			.map (|valve| valve.name.clone ())
			.collect ();
	if ! valve_names.contains (& InpStr::borrow ("AA")) {
		return Err ("Starting valve AA does not exist".into ());
	}
	for valve in & input.valves {
		for tunnel in & valve.tunnels {
			if ! valve_names.contains (tunnel) {
				return Err (format! ("Invalid tunnel from {} to {}", valve.name, tunnel).into ());
			}
		}
	}
	let num_flowable =
		input.valves.iter ()
			.filter (|valve| 0 < valve.flow_rate)
			.count ();
	if 16 < num_flowable {
		return Err (format! ("Can't handle more than 16 flowable valves (found {num_flowable})")
			.into ());
	}
	Ok (())
}

fn build_plans (
	input: & Input,
	volcano: & Volcano,
	initial_remain: u32,
) -> GenResult <Vec <(u16, u32)>> {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	struct State {
		posn: u16,
		open: u16,
		released: u32,
	}
	let mut search = PrioritySearch::with_hash_map (
		|state: State, cmp::Reverse (remain), mut adder: PrioritySearchAdder <_, _, _>| {
			let cavern = & volcano.caverns [state.posn.pan_usize ()];
			for next_cavern in & volcano.caverns {
				if state.open.check_bit (next_cavern.idx.pan_u32 ()) { continue }
				let travel_time = cavern.travel_times [next_cavern.idx.pan_usize ()];
				if remain < travel_time + 2 { continue }
				let new_remain = remain - travel_time - 1;
				adder.add (
					State {
						posn: next_cavern.idx,
						open: state.open.bit_set (next_cavern.idx.pan_u32 ()),
						released: state.released + next_cavern.flow_rate * new_remain,
					},
					cmp::Reverse (new_remain));
			}
			(state.open, state.released)
		});
	for cavern in & volcano.caverns {
		if initial_remain < cavern.initial_travel_time + 2 { continue }
		let new_remain = initial_remain - cavern.initial_travel_time - 1;
		search.push (
			State {
				posn: cavern.idx,
				open: 0.bit_set (cavern.idx.pan_u32 ()),
				released: cavern.flow_rate * new_remain,
			},
			cmp::Reverse (new_remain));
	}
	let mut plans = HashMap::new ();
	let mut num_iters = 0_u32;
	#[ allow (clippy::explicit_counter_loop) ]
	for (open, released) in search {
		if num_iters == input.params.max_iters {
			return Err ("Max iterations exceeded".into ());
		}
		plans.entry (open)
			.and_modify (|plans_released| {
				* plans_released = cmp::max (* plans_released, released);
			})
			.or_insert (released);
		num_iters += 1;
	}
	Ok (iter::once ((0, 0)).chain (plans).collect ())
}
