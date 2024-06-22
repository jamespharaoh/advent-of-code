use super::*;

use input::Input;
use model::Runner;
use model::Signal;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let mut runner = Runner::build (input) ?;
	for _ in 0_u32 .. 1000 {
		runner.run () ?;
	}
	Ok (runner.num_high * runner.num_low)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut runner = Runner::build (input) ?;
	let button_idx = runner.module_for_name ("button") ?.index;
	let broadcaster_idx = runner.module_for_name ("broadcaster") ?.index;
	let rx_idx = runner.module_for_name ("rx") ?.index;
	let combo_idx =
		runner.modules.iter ()
			.filter (|module| module.outputs == [ rx_idx ])
			.filter (|module| module.index != button_idx)
			.filter (|module| module.index != broadcaster_idx)
			.exactly_one ()
			.ok_or ("Can't find combo module") ?
			.index;
	let group_idxs: HashSet <usize> =
		runner.modules.iter ()
			.filter (|module| module.outputs == [ combo_idx ])
			.filter (|module| module.index != button_idx)
			.filter (|module| module.index != broadcaster_idx)
			.map (|module| module.index)
			.collect ();
	if group_idxs.is_empty () { return Err ("Can't find group modules".into ()); }
	let group_state_idxs = {
		let mut global_state_idxs = HashSet::new ();
		let mut group_state_idxs = HashMap::new ();
		for & module_idx in & group_idxs {
			let mut module_idxs = HashSet::new ();
			let mut state_idxs = HashSet::new ();
			let mut todo = vec! [ module_idx ];
			while let Some (module_idx) = todo.pop () {
				let module = & runner.modules [module_idx];
				if ! module_idxs.insert (module_idx) { continue }
				for & state_idx in & module.state {
					if ! state_idxs.insert (state_idx) { continue }
					if ! global_state_idxs.insert (state_idx) {
						return Err ("Combo module state overlap".into ());
					}
				}
				for & module_idx in & module.inputs {
					todo.push (module_idx);
				}
			}
			let state_idxs: Vec <usize> = state_idxs.into_iter ().sorted ().collect ();
			group_state_idxs.insert (module_idx, state_idxs);
		}
		group_state_idxs
	};
	let mut history = HashMap::new ();
	let mut patterns = HashMap::new ();
	let mut num_iters = 0_u32;
	while patterns.len () < group_idxs.len () {
		if num_iters == 10_000 { return Err ("Max iterations exceeded".into ()); }
		num_iters += 1;
		if 1000 < history.len () { return Err ("Max history exceeded".into ()); }
		let signals = runner.run () ?;
		let run_idx = runner.num_runs;
		let mut prev_state = runner.state.clone ();
		for (from_idx, to_idx, signal) in signals {
			if to_idx != combo_idx { continue }
			if signal != Signal::High { continue }
			if ! group_idxs.contains (& from_idx) { continue }
			let state: Vec <Signal> =
				group_state_idxs [& from_idx].iter ()
					.map (|& state_idx| prev_state [state_idx])
					.collect ();
			let key = (from_idx, state);
			if let Some (prev_run_idx) = history.get (& key) {
				patterns.insert (from_idx, run_idx - prev_run_idx);
			} else {
				history.insert (key, run_idx);
			}
			prev_state = runner.state.clone ();
		}
	}
	Ok (patterns.values ().copied ().reduce (u64::lcm).unwrap ())
}
