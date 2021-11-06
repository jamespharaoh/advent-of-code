use std::itertools;

use parse_display_derive::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main () -> Result <(), Box <dyn Error>> {

	let input_string = fs::read_to_string ("input") ?;

	let mut deps: HashMap <char, HashSet <char>> = HashMap::new ();
	for line in input_string.trim ().split ("\n") {
		let line: Line = line.parse () ?;
		deps.entry (line.dependency).or_insert (HashSet::new ());
		(* deps.entry (line.target).or_insert (HashSet::new ())).insert (line.dependency);
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

#[ derive (FromStr) ]
#[ display ("Step {dependency} must be finished before step {target} can begin.") ]
struct Line {
	target: char,
	dependency: char,
}
