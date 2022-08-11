//! Advent of Code 2016: Day 10: Balance Bots
//!
//! [https://adventofcode.com/2016/day/10](https://adventofcode.com/2016/day/10)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Balance Bots";
	year = 2016;
	day = 10;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Step;
	use model::Target;
	use model::Val;

	pub fn part_one (input: & Input) -> GenResult <Val> {
		for event in events (input) ? {
			if event.low_chip == input.low && event.high_chip == input.high {
				return Ok (event.from_bot);
			}
		}
		Err ("No solution found".into ())
	}

	pub fn part_two (input: & Input) -> GenResult <u64> {
		let mut output_0 = None;
		let mut output_1 = None;
		let mut output_2 = None;
		for event in events (input) ? {
			for (chip, target) in [
				(event.low_chip, event.low_target),
				(event.high_chip, event.high_target),
			] {
				if target == Target::Output (0) { output_0 = Some (chip); }
				if target == Target::Output (1) { output_1 = Some (chip); }
				if target == Target::Output (2) { output_2 = Some (chip); }
			}
		}
		match (output_0, output_1, output_2) {
			(Some (output_0), Some (output_1), Some (output_2)) =>
				Ok (output_0.as_u64 () * output_1.as_u64 () * output_2.as_u64 ()),
			_ => Err ("No solution found".into ()),
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	struct Event {
		from_bot: Val,
		low_chip: Val,
		high_chip: Val,
		low_target: Target,
		high_target: Target,
	}

	fn events (input: & Input) -> GenResult <Vec <Event>> {
		#[ derive (Debug, Default) ]
		struct BotState {
			chips: ArrayVec <Val, 2>,
			gives: Option <(Target, Target)>,
		}
		let mut bots: HashMap <Val, BotState> = HashMap::new ();
		let mut todo: Vec <Val> = Vec::new ();
		for step in input.steps.iter () {
			match * step {
				Step::Input { bot, val } => {
					let bot_state = bots.entry (bot).or_default ();
					if bot_state.chips.is_full () {
						Err (format! ("Bot {} has too many chips", bot)) ?;
					}
					bot_state.chips.push (val);
					if bot_state.chips.is_full () && bot_state.gives.is_some () {
						todo.push (bot);
					}
				},
				Step::Give { bot, low, high } => {
					let bot_state = bots.entry (bot).or_default ();
					if bot_state.gives.is_some () {
						Err (format! ("Bot {} gives more than once", bot)) ?;
					}
					bot_state.gives = Some ((low, high));
					if bot_state.chips.is_full () && bot_state.gives.is_some () {
						todo.push (bot);
					}
				},
			}
		}
		let mut events: Vec <Event> = Vec::new ();
		while let Some (from_bot) = todo.pop () {
			let from_bot_state = bots.get_mut (& from_bot).unwrap ();
			let low_chip = from_bot_state.chips.iter_vals ().min ().unwrap ();
			let high_chip = from_bot_state.chips.iter_vals ().max ().unwrap ();
			from_bot_state.chips.clear ();
			let (low_target, high_target) = from_bot_state.gives.unwrap ();
			for (chip, target) in [(low_chip, low_target), (high_chip, high_target)] {
				match target {
					Target::Bot (to_bot) => {
						let to_bot_state = some_or! (bots.get_mut (& to_bot),
							Err (format! ("Bot {} gives to unknown bot {}", from_bot, to_bot)) ?);
						if to_bot_state.chips.is_full () {
							Err (format! ("Bot {} has too many chips", to_bot)) ?;
						}
						to_bot_state.chips.push (chip);
						if to_bot_state.chips.is_full () && to_bot_state.gives.is_some () {
							todo.push (to_bot);
						}
					},
					Target::Output (_) => (),
				}
			}
			events.push (Event { from_bot, low_chip, high_chip, low_target, high_target });
		}
		Ok (events)
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn events () {
			assert_err! ("Bot 0 gives to unknown bot 1", logic::events (& Input {
				steps: vec! [
					Step::Input { val: 1, bot: 0 },
					Step::Input { val: 2, bot: 0 },
					Step::Give { bot: 0, low: Target::Bot (1), high: Target::Bot (2) },
				],
				.. default ()
			}));
			assert_err! ("Bot 0 has too many chips", logic::events (& Input {
				steps: vec! [
					Step::Input { val: 1, bot: 0 },
					Step::Input { val: 2, bot: 0 },
					Step::Input { val: 3, bot: 0 },
				],
				.. default ()
			}));
			assert_err! ("Bot 0 gives more than once", logic::events (& Input {
				steps: vec! [
					Step::Give { bot: 0, low: Target::Bot (1), high: Target::Bot (2) },
					Step::Give { bot: 0, low: Target::Bot (3), high: Target::Bot (4) },
				],
				.. default ()
			}));
		}

	}

}

pub mod model {

	use super::*;
	use parser::*;

	pub type Val = u16;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub steps: Vec <Step>,
		pub low: Val,
		pub high: Val,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Step {
		Input { val: Val, bot: Val },
		Give { bot: Val, low: Target, high: Target },
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Target {
		Bot (Val),
		Output (Val),
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let low = input_param (& mut input, "LOW=", 17_u16) ?;
			let high = input_param (& mut input, "HIGH=", 61_u16) ?;
			let steps = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						parser.item ()
					}).map_parse_err (|col_idx| format! ("Invalid input: line {}: col {}: {}",
						line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { steps, low, high })
		}
	}

	impl <'inp> FromParser <'inp> for Step {
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let val = parser.expect ("value ") ?.confirm ().int () ?;
					let bot = parser.expect (" goes to bot ") ?.int () ?;
					parser.end () ?;
					Ok (Self::Input { val, bot })
				})
				.of (|parser| {
					let bot = parser.expect ("bot ") ?.confirm ().int () ?;
					let low: Target = parser.expect (" gives low to ") ?.item () ?;
					let high: Target = parser.expect (" and high to ") ?.item () ?;
					parser.end () ?;
					Ok (Self::Give { bot, low, high })
				})
				.done ()
		}
	}

	impl <'inp> FromParser <'inp> for Target {
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let bot = parser.expect ("bot ") ?.confirm ().int () ?;
					Ok (Self::Bot (bot))
				})
				.of (|parser| {
					let output = parser.expect ("output ") ?.confirm ().int () ?;
					Ok (Self::Output (output))
				})
				.done ()
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"value 5 goes to bot 2",
		"bot 2 gives low to bot 1 and high to bot 0",
		"value 3 goes to bot 1",
		"bot 1 gives low to output 1 and high to bot 0",
		"bot 0 gives low to output 2 and high to output 0",
		"value 2 goes to bot 2",
	];

	#[ test ]
	fn part_one () {
		use parser::with_params;
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("0", puzzle.part_one (& with_params (["LOW=3", "HIGH=5"], EXAMPLE)));
		assert_eq_ok! ("1", puzzle.part_one (& with_params (["LOW=2", "HIGH=3"], EXAMPLE)));
		assert_eq_ok! ("2", puzzle.part_one (& with_params (["LOW=2", "HIGH=5"], EXAMPLE)));
		assert_err! ("No solution found", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("30", puzzle.part_two (EXAMPLE));
		assert_err! ("No solution found", puzzle.part_one (& EXAMPLE [1 .. ]));
	}

}
