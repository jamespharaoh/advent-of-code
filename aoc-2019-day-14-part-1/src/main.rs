use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter;
use std::str::FromStr;

fn main () {
	let input_lines: Vec <String> =
		fs::read_to_string ("input").unwrap ().trim ().split ('\n').map (str::to_string).collect ();
	let num_ore = calculate_ore (input_lines);
	println! ("Total ORE required: {}", num_ore);
}

fn calculate_ore (input_lines: Vec <String>) -> u64 {

	// assemble reactions table

	let mut reactions: HashMap <String, Reaction> = HashMap::new ();
	for line in input_lines.into_iter () {
		let reaction: Reaction = line.parse ().unwrap ();
		if reactions.contains_key (& reaction.output.chemical) {
			panic! ("Duplicated output: {}", reaction.output.chemical);
		}
		reactions.insert (reaction.output.chemical.clone (), reaction);
	}

	// calculate dependencies

	let mut dependencies: HashMap <String, Vec <String>> = HashMap::new ();
	dependencies.insert ("ORE".to_string (), Vec::new ());
	loop {
		let mut progress = false;
		let mut pending = false;
		for reaction in reactions.values () {
			if dependencies.contains_key (& reaction.output.chemical) {
				continue;
			}
			if reaction.input.iter ().all (
				|input| dependencies.contains_key (& input.chemical),
			) {
				dependencies.insert (
					reaction.output.chemical.clone (),
					reaction.input.iter ().map (
						|input| iter::once (& input.chemical).chain (
							dependencies [& input.chemical].iter (),
						),
					).flatten ().cloned ().collect (),
				);
				progress = true;
			} else {
				pending = true;
			}
		}
		if ! progress { panic! () }
		if ! pending { break }
	}

	// determine order

	let mut completed: HashSet <String> = HashSet::new ();
	let mut order: Vec <String> = Vec::new ();
	completed.insert ("ORE".to_string ());
	loop {
		let mut pending = false;
		let mut progress = false;
		for reaction in reactions.values () {
			if completed.contains (& reaction.output.chemical) {
				continue;
			}
			if dependencies [& reaction.output.chemical].iter ().all (
				|input| completed.contains (input),
			) {
				completed.insert (reaction.output.chemical.clone ());
				order.push (reaction.output.chemical.clone ());
				progress = true;
			} else {
				pending = true;
			}
		}
		if ! progress { panic! () }
		if ! pending { break }
	}
	order.reverse ();

	// calculate quantities

	let mut quantities: HashMap <String, u64> = HashMap::new ();
	quantities.insert ("FUEL".to_string (), 1);
	for output_chemical in order {
		let reaction = & reactions [& output_chemical];
		let mut needed_quantity = quantities [& output_chemical];
		println! ("To make {}×{} we need:", needed_quantity, output_chemical);
		let mut reaction_times: u64 = 0;
		while needed_quantity > 0 {
			reaction_times += 1;
			if needed_quantity <= reaction.output.quantity {
				break;
			}
			needed_quantity -= reaction.output.quantity;
		}
		for input in reaction.input.iter () {
			println! ("  - {}×{}", input.quantity * reaction_times, input.chemical);
			let entry = quantities.entry (input.chemical.clone ()).or_insert (0);
			* entry += input.quantity * reaction_times;
		}
	}

	quantities ["ORE"]

}

#[ derive (Debug) ]
struct Reaction {
	input: Vec <Participant>,
	output: Participant,
}

impl FromStr for Reaction {
	type Err = String;
	fn from_str (input: & str) -> Result <Reaction, String> {
		let parts: Vec <& str> = input.split ("=>").collect ();
		if parts.len () != 2 {
			return Err (format! ("Invalid number of reaction sides: {}", parts.len ()));
		}
		let input_temp: Result <Vec <Participant>, _> =
			parts [0].trim ().split (',').map (str::parse).collect ();
		Ok (Reaction {
			input: input_temp.unwrap (),
			output: parts [1].parse ().unwrap (),
		})
	}
}

#[ derive (Debug) ]
struct Participant {
	chemical: String,
	quantity: u64,
}

impl FromStr for Participant {
	type Err = String;
	fn from_str (input: & str) -> Result <Participant, String> {
		let parts: Vec <& str> = input.trim ().split (' ').collect ();
		if parts.len () != 2 {
			return Err (format! ("Wrong number of participant parts: {}", parts.len ()));
		}
		Ok (Participant {
			chemical: parts [1].to_owned (),
			quantity: parts [0].parse ().map_err (
				|error| format! ("Error decoding quantity: {}", error),
			) ?,
		})
	}
}

#[ test ]
fn test_0 () {
	assert_eq! (165, calculate_ore (vec! [
		"9 ORE => 2 A",
		"8 ORE => 3 B",
		"7 ORE => 5 C",
		"3 A, 4 B => 1 AB",
		"5 B, 7 C => 1 BC",
		"4 C, 1 A => 1 CA",
		"2 AB, 3 BC, 4 CA => 1 FUEL",
	].into_iter ().map (str::to_string).collect ()));
}

#[ test ]
fn test_1 () {
	assert_eq! (13312, calculate_ore (vec! [
		"157 ORE => 5 NZVS",
		"165 ORE => 6 DCFZ",
		"44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
		"12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
		"179 ORE => 7 PSHF",
		"177 ORE => 5 HKGWZ",
		"7 DCFZ, 7 PSHF => 2 XJWVT",
		"165 ORE => 2 GPVTF",
		"3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
	].into_iter ().map (str::to_string).collect ()));
}

#[ test ]
fn test_2 () {
	assert_eq! (180697, calculate_ore (vec! [
		"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
		"17 NVRVD, 3 JNWZP => 8 VPVL",
		"53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
		"22 VJHF, 37 MNCFX => 5 FWMGM",
		"139 ORE => 4 NVRVD",
		"144 ORE => 7 JNWZP",
		"5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
		"5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
		"145 ORE => 6 MNCFX",
		"1 NVRVD => 8 CXFTF",
		"1 VJHF, 6 MNCFX => 4 RFSQX",
		"176 ORE => 6 VJHF",
	].into_iter ().map (str::to_string).collect ()));
}

#[ test ]
fn test_3 () {
	assert_eq! (2210736, calculate_ore (vec! [
		"171 ORE => 8 CNZTR",
		"7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
		"114 ORE => 4 BHXH",
		"14 VRPVC => 6 BMBT",
		"6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
		"6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
		"15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
		"13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
		"5 BMBT => 4 WPTQ",
		"189 ORE => 9 KTJDG",
		"1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
		"12 VRPVC, 27 CNZTR => 2 XDBXC",
		"15 KTJDG, 12 BHXH => 5 XCVML",
		"3 BHXH, 2 VRPVC => 7 MZWV",
		"121 ORE => 7 VRPVC",
		"7 XCVML => 6 RJRHP",
		"5 BHXH, 4 VRPVC => 5 LTCX",
	].into_iter ().map (str::to_string).collect ()));
}
