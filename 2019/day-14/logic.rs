//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Reaction;
use model::Qty;

pub fn part_one (input: & Input) -> GenResult <Qty> {
	let reactions = get_reactions (input) ?;
	let dependencies = calc_dependencies (& reactions) ?;
	let order = calc_order (& reactions, & dependencies) ?;
	calc_ore (& reactions, & order, 1)
}

pub fn part_two (input: & Input) -> GenResult <Qty> {
	let reactions = get_reactions (input) ?;
	let dependencies = calc_dependencies (& reactions) ?;
	let order = calc_order (& reactions, & dependencies) ?;
	let mut max: Qty = 1;
	while calc_ore (& reactions, & order, max) ? <= input.params.num_ore {
		max = Qty::mul_2 (max, 2) ?;
	}
	let mut min: Qty = max / 2;
	while max - min != 1 {
		let guess = min + (max - min) / 2;
		let success = calc_ore (& reactions, & order, guess) ? <= input.params.num_ore;
		if success { min = guess; } else { max = guess; }
	}
	Ok (min)
}

pub type Reactions <'inp> = HashMap <InpStr <'inp>, & 'inp Reaction <'inp>>;
pub type Dependencies <'inp> = HashMap <InpStr <'inp>, Vec <InpStr <'inp>>>;
pub type Order <'inp> = Vec <InpStr <'inp>>;

fn get_reactions <'inp> (input: & 'inp Input <'inp>) -> GenResult <Reactions <'inp>> {
	let mut reactions = Reactions::new ();
	for reaction in & input.reactions {
		let output_chem = & reaction.output.chem;
		if reactions.insert (output_chem.clone (), reaction).is_some () {
			return Err (format! ("Duplicated output: {output_chem}").into ())
		}
	}
	Ok (reactions)
}

fn calc_dependencies <'inp> (reactions: & Reactions <'inp>) -> GenResult <Dependencies <'inp>> {
	let mut dependencies: HashMap <InpStr, Vec <InpStr>> = HashMap::new ();
	dependencies.insert (InpStr::borrow ("ORE"), Vec::new ());
	loop {
		let mut progress = false;
		let mut pending = false;
		for reaction in reactions.values () {
			if dependencies.contains_key (& reaction.output.chem) {
				continue;
			}
			if reaction.inputs.iter ().all (|input| dependencies.contains_key (& input.chem)) {
				dependencies.insert (
					reaction.output.chem.clone (),
					reaction.inputs.iter ()
						.flat_map (|input| iter::once (& input.chem)
							.chain (dependencies [& input.chem].iter ()))
						.cloned ()
						.collect ());
				progress = true;
			} else {
				pending = true;
			}
		}
		if ! progress { return Err ("No solution found".into ()) }
		if ! pending { break }
	}
	Ok (dependencies)
}

fn calc_order <'inp> (
	reactions: & Reactions <'inp>,
	dependencies: & Dependencies <'inp>,
) -> GenResult <Order <'inp>> {
	let mut completed: HashSet <InpStr> = HashSet::new ();
	let mut order: Vec <InpStr> = Vec::new ();
	completed.insert (InpStr::borrow ("ORE"));
	loop {
		let mut pending = false;
		let mut progress = false;
		for reaction in reactions.values () {
			if completed.contains (& reaction.output.chem) {
				continue;
			}
			if dependencies [& reaction.output.chem].iter ()
					.all (|input| completed.contains (input)) {
				completed.insert (reaction.output.chem.clone ());
				order.push (reaction.output.chem.clone ());
				progress = true;
			} else {
				pending = true;
			}
		}
		if ! progress { return Err ("No solution found".into ()) }
		if ! pending { break }
	}
	order.reverse ();
	Ok (order)
}

fn calc_ore (reactions: & Reactions, order: & Order, num_fuel: Qty) -> GenResult <Qty> {
	let mut quantities: HashMap <InpStr, Qty> = HashMap::new ();
	quantities.insert (InpStr::borrow ("FUEL"), num_fuel);
	for output_chemical in order {
		let reaction = & reactions [output_chemical];
		let needed_qty = * quantities.get (output_chemical).ok_or ("No solution found") ?;
		if needed_qty == Qty::ZERO { continue }
		let reaction_times = Qty::div_2 (
			Qty::sub_2 (Qty::add_2 (needed_qty, reaction.output.qty) ?, 1) ?,
			reaction.output.qty) ?;
		for input in & reaction.inputs {
			let entry = quantities.entry (input.chem.clone ()).or_insert (0);
			* entry = Qty::add_2 (* entry, Qty::mul_2 (input.qty, reaction_times) ?) ?;
		}
	}
	Ok (quantities.get (& InpStr::borrow ("ORE")).copied ().ok_or ("No solution found") ?)
}
