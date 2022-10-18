use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub begin_state: char,
	pub num_steps: u32,
	pub states: Vec <State>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { begin_state, num_steps, states, params } = [
		params,
		"Begin in state ", begin_state = 'A' ..= 'Z', ".\n",
		"Perform a diagnostic checksum after ", num_steps, " steps.\n",
		"\n",
		@delim "\n\n" states,
	]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Dir {
		Left = [ "left" ],
		Right = [ "right" ],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Slot {
		Zero = [ "0" ],
		One = [ "1" ],
	}
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

struct_parser_display! {
	State { id, false_write, false_dir, false_state, true_write, true_dir, true_state } = [
		"In state ", id, ":\n",
		"  If the current value is 0:\n",
		"    - Write the value ", false_write, ".\n",
		"    - Move one slot to the ", false_dir, ".\n",
		"    - Continue with state ", false_state = 'A' ..= 'Z', ".\n",
		"  If the current value is 1:\n",
		"    - Write the value ", true_write, ".\n",
		"    - Move one slot to the ", true_dir, ".\n",
		"    - Continue with state ", true_state = 'A' ..= 'Z', ".",
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
