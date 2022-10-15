use super::*;

use input::Input;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		iter_posns (input)
			.take_while (|& (_, dist)| dist <= input.params.max_dist)
			.find (|& (pos, _)| pos == Pos::new (input.params.end_x, input.params.end_y))
			.map (|(_, dist)| dist)
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		iter_posns (input)
			.take_while (|& (_, dist)| dist <= input.params.count_dist)
			.count ()
			.pan_u32 ()
	)
}

fn iter_posns (input: & Input) -> impl Iterator <Item = (Pos, u32)> + '_ {
	let mut todo = VecDeque::new ();
	todo.push_back ((Pos::new (input.params.start_x, input.params.start_y), 0_u32));
	let mut seen = HashSet::new ();
	seen.insert (Pos::new (input.params.start_x, input.params.start_y));
	iter::from_fn (move || {
		let (pos, dist) = some_or! (todo.pop_front (), return None);
		for adj_pos in pos.adjacent_4 () {
			if ! seen.insert (adj_pos) { continue }
			let Pos { x, y } = adj_pos;
			if (x*x + 3*x + 2*x*y + y + y*y + input.seed).count_ones () & 1 == 1 { continue }
			todo.push_back ((adj_pos, dist + 1));
		}
		Some ((pos, dist))
	})
}
