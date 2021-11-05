use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter;

fn main () -> Result <(), Box <dyn Error>> {

	let line_re = Regex::new (
		r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin\.$",
	).unwrap ();

	let input_string = fs::read_to_string ("input") ?;

	let mut deps: HashMap <char, HashSet <char>> = HashMap::new ();
	for line in input_string.trim ().split ("\n") {
		let captures = line_re.captures (line).ok_or_else (
			|| format! ("Failed to parse line: {}", line),
		) ?;
		let dep_step = captures.get (1).unwrap ().as_str ().chars ().next ().unwrap ();
		let target_step = captures.get (2).unwrap ().as_str ().chars ().next ().unwrap ();
		deps.entry (dep_step).or_insert (HashSet::new ());
		(* deps.entry (target_step).or_insert (HashSet::new ())).insert (dep_step);
	}

	let mut remaining: HashSet <char> = deps.keys ().cloned ().collect ();
	let mut completed: HashSet <char> = HashSet::new ();
	let mut queue: HashSet <char> = HashSet::new ();
	let mut workers: Vec <Option <(char, u8)>> = iter::repeat (None).take (5).collect ();
	let mut elapsed: u64 = 0;

	while ! remaining.is_empty ()
		|| ! queue.is_empty ()
		|| workers.iter ().any (Option::is_some) {

		for next_step in remaining.iter ().filter (
			|step| deps [step].iter ().all (|dep| completed.contains (dep)),
		).cloned () {
			queue.insert (next_step);
		}

		for worker in workers.iter_mut ().filter (|worker| worker.is_none ()) {
			if queue.is_empty () { break }
			let next_step = queue.iter ().min ().cloned ().unwrap ();
			let step_time = 61 + (next_step as u32 - 'A' as u32) as u8;
			* worker = Some ((next_step, step_time));
			remaining.remove (& next_step);
			queue.remove (& next_step);
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

	println! ("Elapsed seconds: {}", elapsed);
	Ok (())

}
