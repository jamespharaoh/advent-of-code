//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {

	let mut deps: HashMap <char, HashSet <char>> = HashMap::new ();
	for (before, after) in input.deps.iter_vals () {
		deps.entry (before).or_insert_with (HashSet::new);
		deps.entry (after).or_insert_with (HashSet::new).insert (before);
	}

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

	let mut deps: HashMap <char, HashSet <char>> = HashMap::new ();
	for (before, after) in input.deps.iter_vals () {
		deps.entry (before).or_insert_with (HashSet::new);
		deps.entry (after).or_insert_with (HashSet::new).insert (before);
	}

	let mut remaining: HashSet <char> = deps.keys ().copied ().collect ();
	let mut completed: HashSet <char> = HashSet::new ();
	let mut queue: HashSet <char> = HashSet::new ();
	let mut workers: Vec <Option <(char, u32)>> =
		iter::repeat (None)
			.take (input.params.num_workers.pan_usize ())
			.collect ();
	let mut elapsed: u32 = 0;

	while ! remaining.is_empty ()
			|| ! queue.is_empty ()
			|| workers.iter ().any (Option::is_some) {

		for next_step in remaining.iter_vals ()
				.filter (|step| deps [step].iter ().all (|dep| completed.contains (dep))) {
			queue.insert (next_step);
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
		}

	}

	Ok (elapsed)

}
