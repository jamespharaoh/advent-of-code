use super::*;

use input::Input;
use model::Step;
use model::Target;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	for event in events (input) ? {
		if event.low_chip == input.params.low && event.high_chip == input.params.high {
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
			Ok (output_0.pan_u64 () * output_1.pan_u64 () * output_2.pan_u64 ()),
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

	let (mut bots, mut todo) = get_initial_state (input) ?;

	let mut events: Vec <Event> = Vec::new ();
	while let Some (from_bot) = todo.pop () {
		let from_bot_state = bots.get_mut (& from_bot).unwrap ();

		let & low_chip = from_bot_state.chips.iter ().min ().unwrap ();
		let & high_chip = from_bot_state.chips.iter ().max ().unwrap ();
		from_bot_state.chips.clear ();

		let (low_target, high_target) = from_bot_state.gives.unwrap ();
		for (chip, target) in [(low_chip, low_target), (high_chip, high_target)] {

			if let Target::Bot (to_bot) = target {
				let to_bot_state = some_or! (
					bots.get_mut (& to_bot),
					return Err (format! ("Bot {from_bot} gives to unknown bot {to_bot}")
						.into ()));
				if to_bot_state.chips.is_full () {
					Err (format! ("Bot {to_bot} has too many chips")) ?;
				}
				to_bot_state.chips.push (chip);
				if to_bot_state.chips.is_full () && to_bot_state.gives.is_some () {
					todo.push (to_bot);
				}
			}

		}

		events.push (Event { from_bot, low_chip, high_chip, low_target, high_target });

	}

	Ok (events)

}

#[ derive (Debug, Default) ]
struct BotState {
	chips: TinyVec <Val, 2>,
	gives: Option <(Target, Target)>,
}

fn get_initial_state (input: & Input) -> GenResult <(HashMap <Val, BotState>, Vec <Val>)> {
	let mut bots: HashMap <Val, BotState> = HashMap::new ();
	let mut todo: Vec <Val> = Vec::new ();
	for step in input.steps.iter () {
		match * step {
			Step::Input { bot, val } => {
				let bot_state = bots.entry (bot).or_default ();
				if bot_state.chips.is_full () {
					return Err (format! ("Bot {bot} has too many chips").into ());
				}
				bot_state.chips.push (val);
				if bot_state.chips.is_full () && bot_state.gives.is_some () {
					todo.push (bot);
				}
			},
			Step::Give { bot, low, high } => {
				let bot_state = bots.entry (bot).or_default ();
				if bot_state.gives.is_some () {
					return Err (format! ("Bot {bot} gives more than once").into ());
				}
				bot_state.gives = Some ((low, high));
				if bot_state.chips.is_full () && bot_state.gives.is_some () {
					todo.push (bot);
				}
			},
		}
	}
	Ok ((bots, todo))
}

#[ cfg (test) ]
mod tests {

	use super::*;

	use input::InputParams;

	#[ test ]
	fn events () {
		assert_err! ("Bot 0 gives to unknown bot 1", logic::events (& Input {
			steps: vec! [
				Step::Input { val: 1, bot: 0 },
				Step::Input { val: 2, bot: 0 },
				Step::Give { bot: 0, low: Target::Bot (1), high: Target::Bot (2) },
			],
			params: InputParams::default (),
		}));
		assert_err! ("Bot 0 has too many chips", logic::events (& Input {
			steps: vec! [
				Step::Input { val: 1, bot: 0 },
				Step::Input { val: 2, bot: 0 },
				Step::Input { val: 3, bot: 0 },
			],
			params: InputParams::default (),
		}));
		assert_err! ("Bot 0 gives more than once", logic::events (& Input {
			steps: vec! [
				Step::Give { bot: 0, low: Target::Bot (1), high: Target::Bot (2) },
				Step::Give { bot: 0, low: Target::Bot (3), high: Target::Bot (4) },
			],
			params: InputParams::default (),
		}));
	}

}
