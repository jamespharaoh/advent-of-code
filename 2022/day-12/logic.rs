use super::*;

use input::Input;
use model::Height;
use model::Pos;
use model::Square;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let start =
		input.grid.iter ()
			.filter (|& (_, square)| square == Square::Start)
			.map (|(pos, _)| pos)
			.exactly_one ()
			.ok_or ("Must have exactly one start") ?;
	Ok (
		iter_starts (input) ?
			.find (|& (pos, _, _)| pos == start)
			.map (|(_, _, dist)| dist)
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		iter_starts (input) ?
			.find (|& (_, height, _)| height == 0)
			.map (|(_, _, dist)| dist)
			.ok_or ("No solution found") ?
	)
}

fn iter_starts (input: & Input) -> GenResult <impl Iterator <Item = (Pos, Height, u32)> + '_> {
	let end =
		input.grid.iter ()
			.filter (|& (_, square)| square == Square::End)
			.map (|(pos, _)| pos)
			.exactly_one ()
			.ok_or ("Must have exactly one end") ?;
	let offsets: Vec <GridOffset <Pos, 2>> =
		Pos::ZERO.adjacent_4 ().into_iter ()
			.map (|pos| input.grid.offset (pos))
			.try_collect () ?;
	let mut search =
		PrioritySearch::with_grid_range (
			input.grid.start (),
			input.grid.end (),
			move |cur: GridCursor <Pos, 2>, dist, mut adder: PrioritySearchAdder <_, _, _>| {
				let square = cur.get (& input.grid);
				for & offset in & offsets {
					let Some (adj_cur) = chk! (cur + offset).ok () else { continue };
					let adj_square = adj_cur.get (& input.grid);
					if adj_square.height () + 1 < square.height () { continue }
					adder.add (adj_cur, dist + 1);
				}
				(cur.pos (), square.height (), dist)
			}) ?;
	search.push (input.grid.cursor (end).unwrap (), 0);
	Ok (search)
}
