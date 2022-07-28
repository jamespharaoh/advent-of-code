//! Advent of Code 2015: Day 7: Some Assembly Required
//!
//! [https://adventofcode.com/2015/day/7](https://adventofcode.com/2015/day/7)

use aoc_common::*;

puzzle_info! {
	name = "Some Assembly Required";
	year = 2015;
	day = 7;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::WireId;
	use model::WireInput;
	use model::WireVal;

	pub fn part_one (input: Input) -> GenResult <u16> {
		let resolved = resolve (& input, default ());
		let a_id: WireId = "a".try_into ().unwrap ();
		let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
		Ok (a_val)
	}

	pub fn part_two (input: Input) -> GenResult <u16> {
		let resolved = resolve (& input, default ());
		let a_id: WireId = "a".try_into ().unwrap ();
		let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
		let b_id: WireId = "b".try_into ().unwrap ();
		let resolved = resolve (& input, HashMap::from_iter ([ (b_id, a_val) ]));
		let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
		Ok (a_val)
	}

	fn resolve (
		input: & Input,
		mut resolved: HashMap <WireId, WireVal>,
	) -> HashMap <WireId, WireVal> {

		// setup data about each wire and their inter-dependencies

		#[ derive (Debug) ]
		struct Wire {
			id: WireId,
			input: WireInput,
			inputs: ArrayVec <WireId, 2>,
			outputs: Vec <WireId>,
		}

		let mut wires: HashMap <WireId, Wire> =
			input.iter ()
				.map (|wire| (wire.id.clone (), Wire {
					id: wire.id.clone (),
					input: wire.input.clone (),
					inputs: ArrayVec::from_iter (wire.input.inputs ().iter ().cloned ().cloned ()),
					outputs: default (),
				}))
				.collect ();

		// work out which wires we can handle now and later

		let mut queue = VecDeque::new ();

		for wire in input.iter () {

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
				}.expect ("Error resolving wire, should not be possible")
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

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn test_resolve () -> GenResult <()> {
			let wires = Input::parse (& [
				"123 -> x",
				"456 -> y",
				"x AND y -> d",
				"x OR y -> e",
				"x LSHIFT 2 -> f",
				"y RSHIFT 2 -> g",
				"NOT x -> h",
				"NOT y -> i",
			]) ?;
			let resolved = resolve (& wires, default ());
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

}

pub mod model {

	use super::*;
	use nums::IntConv;
	use parser::*;

	pub type WireVal = u16;

	#[ derive (Clone) ]
	pub struct Input (Vec <Wire>);

	impl Deref for Input {
		type Target = [Wire];
		fn deref (& self) -> & [Wire] { & self.0 }
	}

	impl Input {
		pub fn new (input: Vec <Wire>) -> GenResult <Input> {
			let all_wire_ids =
				input.iter ()
					.map (|wire| wire.id.clone ())
					.collect::<HashSet <_>> ();
			if all_wire_ids.len () < input.len () {
				Err (format! ("Duplicate wire ids")) ?;
			}
			if let Some ((wire, input)) =
				input.iter ()
					.flat_map (|wire| {
						let wire_id = wire.id.clone ();
						wire.input.inputs ().clone ().iter ()
							.map (move |& input| (wire_id.clone (), input.clone ()))
							.collect::<ArrayVec <_, 2>> ()
					})
					.find (|(_, input)| ! all_wire_ids.contains (input)) {
				Err (format! ("Wire {} refers to non-existant input {}", wire, input)) ?;
			}
			if let Some ((wire, msg)) =
				input.iter ()
					.filter_map (|wire| match wire.input {
						WireInput::LeftShift (_, val) if WireVal::BITS <= val.as_u32 () =>
							Some ((wire.id.clone (), format! ("Left shift by {} is invalid", val))),
						WireInput::RightShift (_, val) if WireVal::BITS <= val.as_u32 () =>
							Some ((wire.id.clone (), format! ("Right shift by {} is invalid", val))),
						_ => None,
					})
					.next () {
				Err (format! ("Wire {} has invalid input: {}", wire, msg)) ?;
			}
			Ok (Input (input))
		}
		pub fn parse (input: & [& str]) -> GenResult <Input> {
			let wires =
				input.iter ().enumerate ().map (|(line_idx, line)|
					Parser::wrap (line, Wire::parse_real)
						.map_parse_err (|col_idx| format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, col_idx + 1,
							& line [line.chars ().take (col_idx).map (char::len_utf8).sum () .. ]))
				).collect::<GenResult <_>> () ?;
			Self::new (wires)
		}
	}

	#[ derive (Clone, Eq, Hash, PartialEq) ]
	pub struct WireId (Rc <str>);

	impl Wire {
		pub fn parse (input: & str) -> GenResult <Wire> {
			Parser::wrap (input, Self::parse_real)
				.map_parse_err (|col_idx| format! ("Invalid input: col {}", col_idx + 1))
		}
		fn parse_real (parser: & mut Parser) -> ParseResult <Wire> {
			parser.set_ignore_whitespace (true);
			let valid_id = |word: & str| word.chars ().all (char::is_lowercase);
			parser.any ()
				.of (|parser| {
					let val = parser.int () ?;
					let id = parser.expect_word ("->") ?.confirm ().word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::Static (val)))
				})
				.of (|parser| {
					let arg = parser.word_if (valid_id) ?;
					let id = parser.expect_word ("->") ?.confirm ().word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::Wire (arg.try_into () ?)))
				})
				.of (|parser| {
					let arg_0 = parser.expect_word ("NOT") ?.confirm ().word_if (valid_id) ?;
					let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::Not (arg_0.try_into () ?)))
				})
				.of (|parser| {
					let arg_0 = parser.word_if (valid_id) ?;
					let arg_1 = parser.expect_word ("AND") ?.confirm ().word_if (valid_id) ?;
					let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::And (arg_0.try_into () ?, arg_1.try_into () ?)))
				})
				.of (|parser| {
					let arg_0 = parser.expect_word ("1") ?.expect_word ("AND") ?.confirm ().word_if (valid_id) ?;
					let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::AndOne (arg_0.try_into () ?)))
				})
				.of (|parser| {
					let arg_0 = parser.word_if (valid_id) ?;
					let arg_1 = parser.expect_word ("OR") ?.confirm ().word_if (valid_id) ?;
					let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::Or (arg_0.try_into () ?, arg_1.try_into () ?)))
				})
				.of (|parser| {
					let arg_0 = parser.word_if (valid_id) ?;
					let arg_1: u16 = parser.expect_word ("LSHIFT") ?.confirm ().int () ?;
					if WireVal::BITS <= arg_1.as_u32 () { Err (parser.err ()) ?; }
					let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::LeftShift (arg_0.try_into () ?, arg_1)))
				})
				.of (|parser| {
					let arg_0 = parser.word_if (valid_id) ?;
					let arg_1: u16 = parser.expect_word ("RSHIFT") ?.confirm ().int () ?;
					if WireVal::BITS <= arg_1.as_u32 () { Err (parser.err ()) ?; }
					let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
					parser.end () ?;
					Ok ((id.try_into () ?, WireInput::RightShift (arg_0.try_into () ?, arg_1)))
				})
				.done ()
				.map (|(id, input)| Wire { id, input })
		}
	}

	impl Deref for WireId {
		type Target = str;
		fn deref (& self) -> & str { self.0.deref () }
	}

	impl TryFrom <& str> for WireId {
		type Error = GenError;
		fn try_from (src: & str) -> GenResult <WireId> {
			if ! src.chars ().all (char::is_lowercase) {
				Err ("Wire ID must be lowercase") ?;
			}
			Ok (WireId (src.to_string ().into ()))
		}
	}

	impl Display for WireId {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			Display::fmt (& self.0, formatter)
		}
	}

	impl Debug for WireId {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "WireId ({:?})", self.0) ?;
			Ok (())
		}
	}

	impl PartialOrd for WireId {
		fn partial_cmp (& self, other: & WireId) -> Option <cmp::Ordering> {
			PartialOrd::partial_cmp (& self.0, & other.0)
		}
	}

	impl Ord for WireId {
		fn cmp (& self, other: & WireId) -> cmp::Ordering {
			Ord::cmp (& self.0, & other.0)
		}
	}

	#[ derive (Clone, Debug) ]
	pub struct Wire {
		pub id: WireId,
		pub input: WireInput,
	}

	#[ derive (Clone) ]
	pub enum WireInput {
		Static (WireVal),
		Wire (WireId),
		Not (WireId),
		And (WireId, WireId),
		AndOne (WireId),
		Or (WireId, WireId),
		LeftShift (WireId, WireVal),
		RightShift (WireId, WireVal),
	}

	impl Debug for WireInput {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			use WireInput::*;
			match self {
				Static (val) => write! (formatter, "Static ({:?})", val) ?,
				Wire (arg) => write! (formatter, "Wire ({:?})", arg) ?,
				Not (arg) => write! (formatter, "Not ({:?})", arg) ?,
				And (arg_0, arg_1) => write! (formatter, "And ({:?}, {:?})", arg_0, arg_1) ?,
				AndOne (arg) => write! (formatter, "AndOne ({:?})", arg) ?,
				Or (arg_0, arg_1) => write! (formatter, "Or ({:?}, {:?})", arg_0, arg_1) ?,
				LeftShift (arg_0, arg_1) => write! (formatter, "LeftShift ({:?}, {:?})", arg_0, arg_1) ?,
				RightShift (arg_0, arg_1) => write! (formatter, "RightShift ({:?}, {:?})", arg_0, arg_1) ?,
			}
			Ok (())
		}
	}

	impl WireInput {
		pub fn inputs (& self) -> ArrayVec <& WireId, 2> {
			match self {
				WireInput::Static (_) => array_vec! [],
				WireInput::Wire (arg) => array_vec! [ arg ],
				WireInput::Not (arg) => array_vec! [ arg ],
				WireInput::And (arg_0, arg_1) => array_vec! [ arg_0, arg_1 ],
				WireInput::AndOne (arg) => array_vec! [ arg ],
				WireInput::Or (arg_0, arg_1) => array_vec! [ arg_0, arg_1 ],
				WireInput::LeftShift (arg_0, _) => array_vec! [ arg_0 ],
				WireInput::RightShift (arg_0, _) => array_vec! [ arg_0 ],
			}
		}
	}

}
