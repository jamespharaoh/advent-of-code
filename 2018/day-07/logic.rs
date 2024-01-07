//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {

	let deps = get_deps (input);

	let mut remaining: HashSet <char> = deps.keys ().copied ().collect ();
	let mut completed: HashSet <char> = HashSet::new ();
	let mut order = String::new ();
	while ! remaining.is_empty () {
		let next_step = remaining.iter ()
			.filter (|step| deps [step].iter ().all (|dep| completed.contains (dep)))
			.copied ()
			.min ()
			.ok_or ("No solution found") ?;
		remaining.remove (& next_step);
		completed.insert (next_step);
		order.push (next_step);
	}

	Ok (order)

}

pub fn part_two (input: & Input) -> GenResult <u32> {

	let deps = get_deps (input);

	let mut remaining: HashSet <char> = deps.keys ().copied ().collect ();
	let mut completed: HashSet <char> = HashSet::new ();
	let mut queue: HashSet <char> = HashSet::new ();
	let mut workers: Vec <Option <(char, u32)>> =
		vec! [None; input.params.num_workers.pan_usize ()];
	let mut elapsed: u32 = 0;
	let mut newly_completed = true;

	while ! remaining.is_empty ()
			|| ! queue.is_empty ()
			|| workers.iter ().any (Option::is_some) {

		if newly_completed {
			for & next_step in remaining.iter ()
					.filter (|step| deps [step].iter ().all (|dep| completed.contains (dep))) {
				queue.insert (next_step);
			}
			newly_completed = false;
		}

		for worker in workers.iter_mut ().filter (|worker| worker.is_none ()) {
			if queue.is_empty () { break }
			let next_step = queue.iter ().min ().copied ().unwrap ();
			let step_time = 1 + next_step.pan_u32 () - 'A'.pan_u32 () + input.params.extra_time;
			* worker = Some ((next_step, step_time));
			remaining.remove (& next_step);
			queue.remove (& next_step);
		}

		if workers.iter ().all (Option::is_none) {
			return Err ("No solution found".into ());
		}

		elapsed += 1;
		for worker_opt in workers.iter_mut ().filter (|worker| worker.is_some ()) {
			let worker = worker_opt.as_mut ().unwrap ();
			worker.1 -= 1;
			if worker.1 > 0 { continue }
			completed.insert (worker.0);
			* worker_opt = None;
			newly_completed = true;
		}

	}

	Ok (elapsed)

}

fn get_deps (input: & Input) -> HashMap <char, Vec <char>> {
	let mut deps: HashMap <char, Vec <char>> = HashMap::new ();
	for & (before, after) in input.deps.iter () {
		deps.entry (before).or_default ();
		deps.entry (after).or_default ().push (before);
	}
	deps
}
