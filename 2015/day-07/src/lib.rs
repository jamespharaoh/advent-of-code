//! Advent of Code 2015: Day 7: Some Assembly Required
//!
//! [https://adventofcode.com/2015/day/7](https://adventofcode.com/2015/day/7)

use aoc_common::*;

puzzle_info! {
	name = "Some Assembly Required";
	year = 2015;
	day = 7;
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

mod logic {

	use super::*;
	use model::Input;
	use model::WireId;
	use model::WireInput;
	use model::WireVal;

	pub fn part_one (input: & [& str]) -> GenResult <u16> {
		let wires = model::parse_input (input) ?;
		let resolved = resolve (& wires, default ());
		let a_id: WireId = "a".try_into ().unwrap ();
		let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
		Ok (a_val)
	}

	pub fn part_two (input: & [& str]) -> GenResult <u16> {
		let wires = model::parse_input (input) ?;
		let resolved = resolve (& wires, default ());
		let a_id: WireId = "a".try_into ().unwrap ();
		let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
		let b_id: WireId = "b".try_into ().unwrap ();
		let resolved = resolve (& wires, HashMap::from_iter ([ (b_id, a_val) ]));
		let a_val = resolved.get (& a_id).copied ().ok_or ("No value found for a") ?;
		Ok (a_val)
	}

	pub fn resolve (
		input: & Input,
		mut resolved: HashMap <WireId, WireVal>,
	) -> HashMap <WireId, WireVal> {
		#[ derive (Debug) ]
		struct Wire {
			id: WireId,
			input: WireInput,
			inputs: ArrayVec <WireId, 2>,
			outputs: ArrayVec <WireId, 10>,
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
		while let Some (id) = queue.pop_front () {
			let wire = wires.get_mut (& id).unwrap ();
			let val = match wire.input.clone () {
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
			}.unwrap ();
			resolved.insert (wire.id.clone (), val);
			let wire_id = wire.id.clone ();
			let wire_outputs = mem::take (& mut wire.outputs);
			drop (wire);
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
			let wires = model::parse_input (& [
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

mod model {

	use super::*;

	pub type Input = Vec <Wire>;
	pub type WireVal = u16;

	#[ derive (Clone, Debug, Eq, Hash, PartialEq) ]
	pub struct WireId (Rc <str>);

	impl Deref for WireId {
		type Target = str;
		fn deref (& self) -> & str { self.0.deref () }
	}

	impl TryFrom <& str> for WireId {
		type Error = GenError;
		fn try_from (src: & str) -> GenResult <WireId> {
			if ! src.chars ().all (char::is_lowercase) {
				Err (format! ("Wire ID must be lowercase")) ?;
			}
			Ok (WireId (src.to_string ().into ()))
		}
	}

	impl Display for WireId {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			Display::fmt (& self.0, formatter)
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

	#[ derive (Clone, Debug) ]
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

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		let mut interned_set: RefCell <HashSet <Rc <str>>> = RefCell::new (HashSet::new ());
		let mut intern = |input: & str| {
			if ! input.chars ().all (char::is_lowercase) {
				return Err ("Wire ids must be all lowercase");
			}
			let interned_set = interned_set.get_mut ();
			Ok (WireId (
				interned_set.get (input).cloned ().unwrap_or_else (|| {
					let result: Rc <str> = input.into ();
					interned_set.insert (result.clone ());
					result
				})
			))
		};
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
				let valid_id = |word: & str| word.chars ().all (char::is_lowercase);
				parser.any ()
					.of (|mut parser| {
						let val = parser.int () ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::Static (val)))
					})
					.of (|mut parser| {
						let arg = parser.word_if (valid_id) ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::Wire (intern (arg) ?)))
					})
					.of (|mut parser| {
						let id_0 = parser.expect_word ("NOT") ?.word_if (valid_id) ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::Not (intern (id_0) ?)))
					})
					.of (|mut parser| {
						let id_0 = parser.word_if (valid_id) ?;
						let id_1 = parser.expect_word ("AND") ?.word_if (valid_id) ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::And (intern (id_0) ?, intern (id_1) ?)))
					})
					.of (|mut parser| {
						let arg_0 = parser.expect_word ("1") ?.expect_word ("AND") ?.word_if (valid_id) ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::AndOne (intern (arg_0) ?)))
					})
					.of (|mut parser| {
						let id_0 = parser.word_if (valid_id) ?;
						let id_1 = parser.expect_word ("OR") ?.word_if (valid_id) ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::Or (intern (id_0) ?, intern (id_1) ?)))
					})
					.of (|mut parser| {
						let arg_0 = parser.word_if (valid_id) ?;
						let arg_1 = parser.expect_word ("LSHIFT") ?.int () ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::LeftShift (intern (arg_0) ?, arg_1)))
					})
					.of (|mut parser| {
						let arg_0 = parser.word_if (valid_id) ?;
						let arg_1 = parser.expect_word ("RSHIFT") ?.int () ?;
						let id = parser.expect_word ("->") ?.word_if (valid_id) ?;
						parser.end () ?;
						Ok ((intern (id) ?, WireInput::RightShift (intern (arg_0) ?, arg_1)))
					})
					.done ()
					.map (|(id, input)| Wire { id, input })
			}).map_parse_err (|char_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, char_idx + 1, line))
		).collect::<GenResult <_>> ()
	}

}
