//! Data representation and algorithms used to solve the puzzle

use super::*;
use input::Input;

enum_decl_parser_display! {
	/// Simple enum to represent the state of a pot
	///
	/// This represents a plant pot, and determines whether it contains a [`Plant`] or is [`Empty`].
	///
	#[ derive (Clone, Copy, Debug) ]
	pub enum Pot { Empty = [ "." ], Plant = [ "#" ] }
}

impl Pot {

	#[ inline ]
	#[ must_use ]
	fn as_usize (self) -> usize {
		self.as_u8 ().pan_usize ()
	}

	#[ inline ]
	#[ must_use ]
	const fn as_u8 (self) -> u8 {
		match self {
			Self::Empty => 0,
			Self::Plant => 1,
		}
	}

}

/// Efficiently apply rules to a [`State`]
///
/// This builds up a table to apply rules to eight pots at a time. It uses a table with `4096`
/// entries. This is enough to check `12` bits at the same time, which is sufficient to determine
/// the output of eight output bits at the same time. That includes the eight output bits
/// themselves, plus two bits to either side.
///
/// The main methods are [`build`](Generator::build) to create a new `Generator` from the puzzle
/// input, and `next` to apply the rules to a [`State`].
///
#[ derive (Clone, Debug) ]
pub struct Generator {
	bit_rules: [u8; 4096],
}

impl Generator {

	/// Build a [`Generator`] from an [`Input`]
	///
	/// This analyses the rules and creates a generator which can apply them quickly to a
	/// [`State`].
	///
	/// This returns an error if there are duplicate rules, or if any rules are missing. The check
	/// for missing rules is skipped in the input parameter `check_rules` is set to `false`, in
	/// which case any missing rules are assumed to result in no plant. This is used for testing,
	/// where the example given only includes rules which result in a plant.
	///
	pub fn build (input: & Input) -> GenResult <Self> {
		let mut rules = [None; 32];
		for input_rule in input.rules.iter () {
			let rule_idx = input_rule.from.iter ()
				.fold (0, |state, & item| (state << 1_u32) | item.as_usize ());
			let rule = & mut rules [rule_idx];
			if rule.is_some () { return Err ("Duplicated rule".into ()) }
			* rule = Some (input_rule.to);
		}
		if input.params.check_rules && rules.iter ().any (Option::is_none) {
			return Err ("Missing rule".into ());
		}
		let rules = rules.map (|rule| rule.unwrap_or (Pot::Empty));
		let bit_rules = array::from_fn (|mut idx| {
			let mut val = 0_u8;
			for _ in 0 .. 8_u32 {
				let from = (idx & 0xf80) >> 7_u32;
				idx <<= 1_u32;
				val = (val << 1_u32) | rules [from].as_u8 ();
			}
			val
		});
		Ok (Self { bit_rules })
	}

	pub fn next (& self, state: & State) -> NumResult <State> {
		let mut start = state.start - 2;
		let mut data = Vec::new ();
		let mut prev = 0_u8;
		for & byte in state.data.iter ().chain (iter::once (& 0)) {
			let idx = (prev.pan_usize () & 0x0f) << 8_u32 | byte.pan_usize ();
			assert! (idx < 4096);
			let next = self.bit_rules [idx];
			if next == 0 && data.is_empty () {
				start = chk! (start + 8) ?;
			} else {
				data.push (next);
			}
			prev = byte;
		}
		while data.last ().copied () == Some (0) { data.pop ().unwrap (); }
		Ok (State { start, data })
	}

}

/// Represents all occupied plant pots at a specific point in time
///
/// This uses an array of bytes, with each bit representing a single plant pot. We only cover the
/// range of occupied plant pots, so the first and last entries should never be `0`. The number of
/// plant represented by the first bit in the first byte is also stored.
///
#[ derive (Default) ]
pub struct State {
	pub start: i64,
	pub data: Vec <u8>,
}

impl State {

	/// Return an iterator over the numbers of all occupied pots, in order
	///
	pub fn iter (& self) -> impl Iterator <Item = (i64, Pot)> + '_ {
		self.data.iter ().enumerate ()
			.flat_map (|(byte_idx, byte)| {
				let start = self.start + (byte_idx.pan_i64 ()) * 8;
				[
					(start, if byte & 0x80 != 0 { Pot::Plant } else { Pot::Empty }),
					(start + 1, if byte & 0x40 != 0 { Pot::Plant } else { Pot::Empty }),
					(start + 2, if byte & 0x20 != 0 { Pot::Plant } else { Pot::Empty }),
					(start + 3, if byte & 0x10 != 0 { Pot::Plant } else { Pot::Empty }),
					(start + 4, if byte & 0x08 != 0 { Pot::Plant } else { Pot::Empty }),
					(start + 5, if byte & 0x04 != 0 { Pot::Plant } else { Pot::Empty }),
					(start + 6, if byte & 0x02 != 0 { Pot::Plant } else { Pot::Empty }),
					(start + 7, if byte & 0x01 != 0 { Pot::Plant } else { Pot::Empty }),
				]
			})
	}

}

impl From <& [Pot]> for State {

	#[ inline ]
	fn from (pots: & [Pot]) -> Self {
		let data =
			pots.iter ().copied ()
				.chain (iter::repeat (Pot::Empty).take (7))
				.tuples::<(_, _, _, _, _, _, _, _)> ()
				.map (|(a, b, c, d, e, f, g, h)|
					a.as_u8 () << 7_u32 | b.as_u8 () << 6_u32 |
					c.as_u8 () << 5_u32 | d.as_u8 () << 4_u32 |
					e.as_u8 () << 3_u32 | f.as_u8 () << 2_u32 |
					g.as_u8 () << 1_u32 | h.as_u8 ())
				.collect ();
		Self { start: 0, data }
	}

}
