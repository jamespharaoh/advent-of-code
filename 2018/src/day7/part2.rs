use parse_display_derive::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::iter;

pub fn aoc2018_day7_part2 (input: & str) -> Result <String, Box <dyn Error>> {

	let mut deps: HashMap <char, HashSet <char>> = HashMap::new ();
	for line in input.trim ().split ("\n") {
		let line: Line = line.parse () ?;
		deps.entry (line.dependency).or_insert (HashSet::new ());
		(* deps.entry (line.target).or_insert (HashSet::new ())).insert (line.dependency);
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

	Ok (format! ("{}", elapsed))

}

#[ derive (FromStr) ]
#[ display ("Step {dependency} must be finished before step {target} can begin.") ]
struct Line {
	target: char,
	dependency: char,
}
