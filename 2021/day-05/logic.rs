use super::*;

use input::Input;
use model::Grid;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	calc_result (input, false)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	calc_result (input, true)
}

fn check_input (input: & Input) -> GenResult <()> {
	for & vent in & input.vents {
		if ! vent.is_valid () {
			return Err (format! ("Vent is at an invalid angle: {vent:?}").into ());
		}
	}
	Ok (())
}

fn calc_result (input: & Input, include_diagonal: bool) -> GenResult <u32> {
	let vents: Vec <_> =
		input.vents.iter ()
			.filter (|vent| include_diagonal || ! vent.is_diagonal ())
			.collect ();
	let (start, end) = vents.iter ()
		.flat_map (|vent| [ vent.start, vent.end ])
		.fold ((Pos::ZERO, Pos::new (1, 1)), |(min, max), pos| (
			Pos::new (cmp::min (min.y, pos.y), cmp::min (min.x, pos.x)),
			Pos::new (cmp::max (max.y, pos.y + 1), cmp::max (max.x, pos.x + 1))));
	let mut grid: Grid = Grid::new_range (start, end) ?;
	for vent in vents {
		let step = Pos::new (
			(vent.end.y - vent.start.y).signum (),
			(vent.end.x - vent.start.x).signum ());
		let mut pos = vent.start;
		loop {
			* grid.get_mut (pos).unwrap () += 1;
			if pos == vent.end { break }
			pos += step;
		}
	}
	Ok (
		grid.values ()
			.filter (|& num| 1 < num)
			.count ()
			.pan_u32 ()
	)
}
