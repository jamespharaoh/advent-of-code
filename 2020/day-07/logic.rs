//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut direct_containers: HashMap <InpStr, HashSet <InpStr>> = HashMap::new ();
	for bag_contains in & input.bags_contain {
		for contents in & bag_contains.contains {
			direct_containers.entry (contents.colour.clone ())
				.or_default ()
				.insert (bag_contains.colour.clone ());
		}
	}
	let mut todo = Vec::new ();
	todo.push (InpStr::borrow ("shiny gold"));
	let mut seen = HashSet::new ();
	let mut num_iters = 0;
	while let Some (this_colour) = todo.pop () {
		if num_iters == input.params.max_iters_one {
			return Err ("Giving up after max iters".into ());
		}
		num_iters += 1;
		for container in direct_containers.get (& this_colour).unwrap_or (& HashSet::new ()) {
			if ! seen.insert (container.clone ()) { continue }
			todo.push (container.clone ());
		}
	}
	Ok (seen.len ().pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let direct_contents: HashMap <InpStr, Vec <(InpStr, u32)>> =
		input.bags_contain.iter ()
			.map (|bag_contains| (
				bag_contains.colour.clone (),
				bag_contains.contains.iter ()
					.map (|contained| (contained.colour.clone (), contained.num))
					.collect ()
			))
			.collect ();
	let mut todo = Vec::new ();
	todo.push ((InpStr::borrow ("shiny gold"), 1));
	let mut total_bags = 0_u32;
	let mut num_iters = 0;
	while let Some ((this_colour, this_num_bags)) = todo.pop () {
		if num_iters == input.params.max_iters_two {
			return Err ("Giving up after max iters".into ());
		}
		num_iters += 1;
		total_bags = chk! (total_bags + this_num_bags) ?;
		for & (ref contains_colour, contains_num_bags) in
			direct_contents.get (& this_colour)
				.ok_or_else (|| format! ("No rules for {this_colour} bags")) ? {
			todo.push ((
				contains_colour.clone (),
				chk! (this_num_bags * contains_num_bags) ?,
			));
		}
	}
	Ok (total_bags - 1)
}
