use super::*;

parse_display_enum! {

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Dir { Left = "left", Right = "right" }

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Slot { Zero = "0", One = "1" }

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct State {
	pub id: char,
	pub false_write: Slot,
	pub false_dir: Dir,
	pub false_state: char,
	pub true_write: Slot,
	pub true_dir: Dir,
	pub true_state: char,
}

impl <'inp> FromParser <'inp> for State {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let parse_state_id = |parser: & mut Parser| {
			match parser.next () {
				Some (ch @ 'A' ..= 'Z') => Ok (ch),
				_ => Err (parser.err ()),
			}
		};
		parse! (parser,
			"In state ", id, ":\n",
			"  If the current value is 0:\n",
			"    - Write the value ", false_write, ".\n",
			"    - Move one slot to the ", false_dir, ".\n",
			"    - Continue with state ", false_state = parse_state_id, ".\n",
			"  If the current value is 1:\n",
			"    - Write the value ", true_write, ".\n",
			"    - Move one slot to the ", true_dir, ".\n",
			"    - Continue with state ", true_state = parse_state_id, ".",
		);
		Ok (Self { id, false_write, false_dir, false_state, true_write, true_dir, true_state })
	}
}

impl Display for State {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			concat! (
				"In state {id}:\n",
				"  If the current value is 0:\n",
				"    - Write the value {false_write}.\n",
				"    - Move one slot to the {false_dir}.\n",
				"    - Continue with state {false_state}.\n",
				"  If the current value is 1:\n",
				"    - Write the value {true_write}.\n",
				"    - Move one slot to the {true_dir}.\n",
				"    - Continue with state {true_state}.\n",
			),
			id = self.id,
			false_write = self.false_write,
			false_dir = self.false_dir,
			false_state = self.false_state,
			true_write = self.true_write,
			true_dir = self.true_dir,
			true_state = self.true_state,
		) ?;
		Ok (())
	}
}
