use super::*;

use input::Component;
use input::Input;
use model::State;

pub fn part_one (input: & Input) -> GenResult <usize> {
	calc_result (input)
}

pub fn part_two (input: & Input) -> GenResult <usize> {
	let mut input = input.clone ();
	input.floors [0].push (Component::Generator (InpStr::borrow ("elerium")));
	input.floors [0].push (Component::Microchip (InpStr::borrow ("elerium")));
	input.floors [0].push (Component::Generator (InpStr::borrow ("dilithium")));
	input.floors [0].push (Component::Microchip (InpStr::borrow ("dilithium")));
	calc_result (& input)
}

fn calc_result (input: & Input) -> GenResult <usize> {
	let (mut start_state, _names) = State::from_input (input) ?;
	start_state.comps [ .. start_state.comps_len.pan_usize ()].sort ();
	let mut seen = HashSet::new ();
	seen.insert (start_state.compact ());
	let mut todo = VecDeque::new ();
	todo.push_back ((start_state.compact (), 0));
	let mut next_states = Vec::new ();
	while let Some ((state_compact, steps)) = todo.pop_front () {
		let state = state_compact.expand (start_state.comps_len);
		if state.is_done () { return Ok (steps) }
		next_states.clear ();
		state.next_states (& mut next_states);
		for & next_state_compact in next_states.iter () {
			if ! seen.insert (next_state_compact) { continue }
			todo.push_back ((next_state_compact, steps + 1));
		}
	}
	Err ("No solution found".into ())
}
