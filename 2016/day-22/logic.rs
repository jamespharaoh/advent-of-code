use super::*;

use input::Input;
use model::Coord;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <usize> {
	Ok (
		input.nodes.iter ()
			.filter (|node_a| node_a.used > 0)
			.map (|node_a| input.nodes.iter ()
				.filter (|node_b| node_a.pos != node_b.pos)
				.filter (|node_b| node_a.used <= node_b.avail)
				.count ())
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <usize> {

	// TODO this gets the right answer for the puzzles set, but it's not correct in other cases...
	// We can do better, or at the very least return an error if it doesn't meet the criteria.

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	struct State { data: Pos, empty: Pos }

	type Walls = GridBuf <Vec <bool>, Pos, 2>;

	let size =
		input.nodes.iter ()
			.try_fold (Pos::ZERO, |size, node| Ok::<_, Overflow> (Pos::new (
				cmp::max (size.x, chk! (node.pos.x + Coord::ONE) ?),
				cmp::max (size.y, chk! (node.pos.y + Coord::ONE) ?)))) ?;

	if 40 < size.x || 40 < size.y {
		return Err ("Max grid size is 40×40".into ());
	}

	let empty_start =
		input.nodes.iter ()
			.filter (|node| node.used == 0)
			.map (|node| node.pos)
			.exactly_one ()
			.ok ()
			.ok_or ("Must have exactly one empty node") ?;

	let lowest =
		input.nodes.iter ()
			.filter (|node| node.used > 0)
			.map (|node| node.used)
			.min ()
			.ok_or ("No nodes have used data") ?;

	let data_start = Pos::new (size.x - 1, 0);
	let data_end = Pos::ZERO;

	let mut walls = Walls::new_size (size);
	for node in input.nodes.iter () {
		walls.set (node.pos, node.used > lowest * 2);
	}

	let state = State { data: data_start, empty: empty_start };

	let mut seen: HashSet <State> = HashSet::new ();
	seen.insert (state);

	let mut todo: VecDeque <(usize, State)> = VecDeque::new ();
	todo.push_back ((0, state));

	while let Some ((dist, state)) = todo.pop_front () {
		if state.data == data_end { return Ok (dist) }
		for from_pos in state.empty.adjacent_4 () {
			if walls.get (from_pos).unwrap_or (true) { continue }
			let new_state = State {
				data: if from_pos == state.data { state.empty } else { state.data },
				empty: from_pos,
			};
			if seen.insert (new_state) {
				todo.push_back ((dist + 1, new_state));
			}
		}
	}

	Err ("No solution found".into ())

}
