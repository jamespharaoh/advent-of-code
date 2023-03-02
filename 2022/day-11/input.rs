use super::*;

use model::Item;
use model::MonkeyId;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub monkeys: Vec <Monkey>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { monkeys, params } = [ params, @delim "\n\n" monkeys ]
}

#[ derive (Clone, Debug) ]
pub struct Monkey {
	pub id: MonkeyId,
	pub start_items: Vec <Item>,
	pub operation: Operation,
	pub divisible_by: Item,
	pub throw_true: MonkeyId,
	pub throw_false: MonkeyId,
}

struct_parser_display! {
	Monkey { id, start_items, operation, divisible_by, throw_true, throw_false } = [
		"Monkey ", id, ":\n",
		"  Starting items: ", @delim ", " start_items, "\n",
		"  Operation: ", operation, "\n",
		"  Test: divisible by ", divisible_by = (1 .. ), "\n",
		"    If true: throw to monkey ", throw_true, "\n",
		"    If false: throw to monkey ", throw_false,
	]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum Operation {
		Add (arg: Item) = [ "new = old + ", arg ],
		Multiply (arg: Item) = [ "new = old * ", arg ],
		Square = [ "new = old * old" ],
	}
}

impl Operation {

	pub fn apply (self, item: Item) -> NumResult <Item> {
		match self {
			Self::Add (arg) => chk! (item + arg),
			Self::Multiply (arg) => chk! (item * arg),
			Self::Square => chk! (item * item),
		}
	}

}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub rounds_one: u32 = ("ROUNDS_ONE=", 20, 1 .. ),
		pub rounds_two: u32 = ("ROUNDS_TWO=", 10_000, 1 .. ),
		pub div_one: u64 = ("DIV_ONE=", 3, 1 .. ),
		pub div_two: u64 = ("DIV_TWO=", 1, 1 .. ),
	}
}
