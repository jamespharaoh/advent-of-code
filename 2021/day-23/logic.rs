use super::*;

use input::Input;
use model::Amph::{ self, Amber, Bronze, Copper, Desert };
use model::Place;
use model::State;
use model::StateCompact;
use search::PrioritySearch;
use search::PrioritySearchAdder;

pub fn part_one (input: & Input) -> GenResult <i64> {
	check_input (input) ?;
	let state = State::new_part_one (input);
	calc_result (state)
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	check_input (input) ?;
	let state = State::new_part_two (input);
	calc_result (state)
}

pub fn calc_result (initial_state: State) -> GenResult <i64> {
	Ok (
		iterator (initial_state)
			.filter (|& (ref state_compact, _)| state_compact.is_finished ())
			.map (|(_, score)| score)
			.next ()
			.ok_or ("Failed to find solution") ?
	)
}

pub fn iterator (initial_state: State) -> impl Iterator <Item = (StateCompact, i64)> {
	let mut search = PrioritySearch::with_hash_map (
		|state: StateCompact, score: i64, mut adder: PrioritySearchAdder <'_, _, _, _>| {
			for (next_state, next_cost) in calc_next_states (state) {
				let next_score = score + next_cost;
				adder.add (next_state, next_score);
			}
			(state, score)
		},
	);
	search.push (initial_state.compact (), 0);
	search
}

#[ must_use ]
pub fn calc_next_states (state_compact: StateCompact) -> ArrayVec <(StateCompact, i64), 28> {
	let state = state_compact.expand ();

	let out_cost = |room| state.room_size () - state.room (room).len () + 1;
	let in_cost = |room| state.room_size () - state.room (room).len ();
	let hall_cost = |room: Amph, hall: Place|
		usize::abs_diff (2 + room.idx () * 2, hall.idx ());

	let next_moves = calc_next_moves (& state);
	if next_moves.is_empty () { return ArrayVec::new () }

	let blocking = (state.hall () [3], state.hall () [5], state.hall () [7]);
	let sections = [
		! matches! (blocking,
			(Some (Copper | Desert), _, _) |
			(_, Some (Bronze | Copper | Desert), _) |
			(_, _, Some (Desert))),
		! matches! (blocking,
			(Some (Amber), _, _) |
			(_, Some (Copper | Desert), _) |
			(_, _, Some (Desert))),
		! matches! (blocking,
			(Some (Amber), _, _) |
			(_, Some (Amber | Bronze), _) |
			(_, _, Some (Desert))),
		! matches! (blocking,
			(Some (Amber), _, _) |
			(_, Some (Amber | Bronze), _) |
			(_, _, Some (Amber | Bronze | Copper))),
	];

	let mut next_states = ArrayVec::new ();
	for next_move in next_moves.iter ().copied () {
		match next_move {
			Move::Between (amph, from_room, to_room) => {
				if ! sections [from_room.idx ()] || ! sections [to_room.idx ()] { continue }
				let cost = amph.cost () * (out_cost (from_room) + in_cost (to_room)
					+ usize::abs_diff (from_room.idx (), to_room.idx ()) * 2).pan_i64 ();
				let next_state = state.move_between (from_room, to_room);
				return iter::once ((next_state.compact (), cost)).collect ();
			},
			Move::In (amph, from_hall, to_room) => {
				if ! sections [to_room.idx ()] { continue }
				let cost = amph.cost () * (in_cost (to_room)
					+ hall_cost (to_room, from_hall)).pan_i64 ();
				let next_state = state.move_in (from_hall, to_room);
				return iter::once ((next_state.compact (), cost)).collect ();
			},
			Move::Out (..) => (),
		}
	}
	for next_move in next_moves.iter ().copied () {
		if let Move::Out (amph, from_room, to_hall) = next_move {
			if ! sections [from_room.idx ()] { continue }
			let cost = amph.cost () * (out_cost (from_room)
				+ hall_cost (from_room, to_hall)).pan_i64 ();
			let next_state = state.move_out (from_room, to_hall);
			next_states.push ((next_state.compact (), cost));
		}
	}

	next_states

}

#[ derive (Clone, Copy) ]
pub enum Move {
	Out (Amph, Amph, Place),
	In (Amph, Place, Amph),
	Between (Amph, Amph, Amph),
}

#[ must_use ]
pub fn calc_next_moves (state: & State) -> ArrayVec <Move, 28> {
	let mut result = ArrayVec::new ();
	let room_entrance = |room: Amph| Place::for_idx (2 + room.idx () * 2);
	for (idx, amph) in state.hall ().iter ().enumerate ()
			.filter_map (|(idx, amph)| amph.map (|amph| (idx, amph))) {
		let to_room = amph;
		let hall = Place::for_idx (idx);
		if ! state.room_is_happy (to_room) { continue }
		if ! path_clear (state, hall, room_entrance (to_room)) { continue }
		result.clear ();
		result.push (Move::In (amph, hall, to_room));
		return result;
	}
	for (from_room, amphs) in [
		(Amber, state.room (Amber)),
		(Bronze, state.room (Bronze)),
		(Copper, state.room (Copper)),
		(Desert, state.room (Desert)),
	] {
		if let Some (& amph) = amphs.last () {
			let to_room = amph;
			if state.room_is_happy (from_room) { continue }
			if state.room_is_happy (to_room)
					&& path_clear (state, room_entrance (from_room), room_entrance (to_room)) {
				result.clear ();
				result.push (Move::Between (amph, from_room, to_room));
				return result;
			}
			for hall in
				iter::successors (
						Some (room_entrance (from_room)),
						|prev_hall| (prev_hall.idx () > 0).then (||
							Place::for_idx (prev_hall.idx () - 1)))
					.take_while (|& hall| state.get (hall).is_none ())
					.chain (
						iter::successors (
								Some (room_entrance (from_room)),
								|prev_hall| (prev_hall.idx () + 1 < 11).then_some (
									Place::for_idx (prev_hall.idx () + 1)))
							.take_while (|& hall| state.get (hall).is_none ()))
					.filter (|hall| ! hall.entrance ()) {
				if ! path_clear (state, room_entrance (from_room), hall) { continue }
				result.push (Move::Out (amph, from_room, hall));
			}
		}
	}
	result
}

fn path_clear (state: & State, from: Place, to: Place) -> bool {
	state.hall ().iter ().enumerate ()
		.skip (cmp::min (to.idx (), from.idx ()))
		.take (usize::abs_diff (from.idx (), to.idx ()) + 1)
		.map (|(idx, amph)| (Place::for_idx (idx), amph))
		.filter (|& (hall, _)| hall != from)
		.all (|(_, amph)| amph.is_none ())
}

fn check_input (input: & Input) -> GenResult <()> {
	let (num_amber, num_bronze, num_copper, num_desert) =
		input.amphs.iter ().flatten ()
			.fold ((0_u32, 0_u32, 0_u32, 0_u32), |(amber, bronze, copper, desert), & amph|
				match amph {
					Amber => (amber + 1, bronze, copper, desert),
					Bronze => (amber, bronze + 1, copper, desert),
					Copper => (amber, bronze, copper + 1, desert),
					Desert => (amber, bronze, copper, desert + 1),
				});
	if num_amber != 2 || num_bronze != 2 || num_copper != 2 || num_desert != 2 {
		return Err ("Must have exactly two each of amber, bronze, copper, desert".into ());
	}
	Ok (())
}
