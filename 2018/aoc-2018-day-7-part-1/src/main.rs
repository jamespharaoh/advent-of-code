use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

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
	let mut order = String::new ();
	while ! remaining.is_empty () {
		let next_step = remaining.iter ().filter (
			|step| deps [step].iter ().all (|dep| completed.contains (dep)),
		).cloned ().min ().ok_or ("No next step") ?;
		remaining.remove (& next_step);
		completed.insert (next_step);
		order.push (next_step);
	}

	println! ("Order: {}", order);
	Ok (())

}
