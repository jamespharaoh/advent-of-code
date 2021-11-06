use parse_display_derive::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn aoc2018_day7_part1 (input: & str) -> Result <String, Box <dyn Error>> {

	let mut deps: HashMap <char, HashSet <char>> = HashMap::new ();
	for line in input.trim ().split ("\n") {
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

	Ok (order)

}

#[ derive (FromStr) ]
#[ display ("Step {dependency} must be finished before step {target} can begin.") ]
struct Line {
	target: char,
	dependency: char,
}
