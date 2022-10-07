use super::*;

use input::Input;
use model::WireId;
use model::WireInput;
use model::WireVal;

pub fn part_one (input: & Input) -> GenResult <u16> {
	check_input (input) ?;
	let resolved = resolve (input, default ());
	let a_id: WireId = "a".try_into ().unwrap ();
	let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
	Ok (a_val)
}

pub fn part_two (input: & Input) -> GenResult <u16> {
	check_input (input) ?;
	let resolved = resolve (input, default ());
	let a_id: WireId = "a".try_into ().unwrap ();
	let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
	let b_id: WireId = "b".try_into ().unwrap ();
	let resolved = resolve (input, HashMap::from_iter ([ (b_id, a_val) ]));
	let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
	Ok (a_val)
}

fn resolve <'inp> (
	input: & Input <'inp>,
	mut resolved: HashMap <WireId <'inp>, WireVal>,
) -> HashMap <WireId <'inp>, WireVal> {

	// setup data about each wire and their inter-dependencies

	#[ derive (Debug) ]
	struct Wire <'inp> {
		id: WireId <'inp>,
		input: WireInput <'inp>,
		inputs: ArrayVec <WireId <'inp>, 2>,
		outputs: Vec <WireId <'inp>>,
	}

	let mut wires: HashMap <WireId, Wire> =
		input.wires.iter ()
			.map (|wire| (wire.id.clone (), Wire {
				id: wire.id.clone (),
				input: wire.input.clone (),
				inputs: wire.input.inputs ().iter ().copied ().cloned ().collect (),
				outputs: default (),
			}))
			.collect ();

	// work out which wires we can handle now and later

	let mut queue = VecDeque::new ();

	for wire in input.wires.iter () {

		for inpt_id in wire.input.inputs () {
			let inpt_wire = wires.get_mut (inpt_id).unwrap ();
			inpt_wire.outputs.push (wire.id.clone ());
		}

		if wire.input.inputs ().is_empty () {
			queue.push_back (wire.id.clone ());
		}

	}

	// iterate the ready wires

	while let Some (id) = queue.pop_front () {
		let wire = wires.get_mut (& id).unwrap ();
		let val = if let Some (& val) = resolved.get (& wire.id) {
		    val
		} else {
			match wire.input.clone () {
				WireInput::Static (val) => Some (val),
				WireInput::Wire (arg) => resolved.get (& arg).copied (),
				WireInput::Not (arg) => resolved.get (& arg).copied ().map (|arg| ! arg),
				WireInput::And (arg_0, arg_1) =>
					resolved.get (& arg_0)
						.and_then (|& arg_0| resolved.get (& arg_1)
							.map (|& arg_1| arg_0 & arg_1)),
				WireInput::AndOne (arg) => resolved.get (& arg).map (|& arg| 1 & arg),
				WireInput::Or (arg_0, arg_1) =>
					resolved.get (& arg_0)
						.and_then (|& arg_0| resolved.get (& arg_1)
							.map (|& arg_1| arg_0 | arg_1)),
				WireInput::LeftShift (arg_0, arg_1) =>
					resolved.get (& arg_0).map (|& arg_0| arg_0 << arg_1),
				WireInput::RightShift (arg_0, arg_1) =>
					resolved.get (& arg_0).map (|& arg_0| arg_0 >> arg_1),
			}.unwrap ()
		};
		resolved.insert (wire.id.clone (), val);
		let wire_id = wire.id.clone ();
		let wire_outputs = mem::take (& mut wire.outputs);
		for outp in wire_outputs {
			let other = wires.get_mut (& outp).unwrap ();
			other.inputs.retain (|other_id| * other_id != wire_id);
			if ! other.inputs.is_empty () { continue }
			queue.push_back (other.id.clone ());
		}
	}

	resolved

}

fn check_input (input: & Input) -> GenResult <()> {
	let all_wire_ids =
		input.wires.iter ()
			.map (|wire| wire.id.clone ())
			.collect::<HashSet <_>> ();
	if all_wire_ids.len () < input.wires.len () {
		Err ("Duplicate wire ids") ?;
	}
	if let Some ((wire, input)) =
		input.wires.iter ()
			.flat_map (|wire| {
				let wire_id = wire.id.clone ();
				wire.input.inputs ().clone ().iter ()
					.map (move |& input| (wire_id.clone (), input.clone ()))
					.collect::<ArrayVec <_, 2>> ()
			})
			.find (|& (_, ref input)| ! all_wire_ids.contains (input)) {
		Err (format! ("Wire {} refers to non-existant input {}", wire, input)) ?;
	}
	Ok (())
}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_resolve () -> GenResult <()> {
		let input = input::Input::parse_from_lines (& [
			"123 -> x",
			"456 -> y",
			"x AND y -> d",
			"x OR y -> e",
			"x LSHIFT 2 -> f",
			"y RSHIFT 2 -> g",
			"NOT x -> h",
			"NOT y -> i",
		]) ?;
		let resolved = resolve (& input, default ());
		assert_eq! (resolved.len (), 8);
		let resolve = |id_str: & str| resolved [& id_str.try_into ().unwrap ()];
		assert_eq! (resolve ("d"), 72);
		assert_eq! (resolve ("e"), 507);
		assert_eq! (resolve ("f"), 492);
		assert_eq! (resolve ("g"), 114);
		assert_eq! (resolve ("h"), 65412);
		assert_eq! (resolve ("i"), 65079);
		assert_eq! (resolve ("x"), 123);
		assert_eq! (resolve ("y"), 456);
		Ok (())
	}

}
