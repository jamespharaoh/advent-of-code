#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use input::Input;
use model::Cursor;
use model::Grid;
use model::Pos;
use model::Offset;
use model::Risks;
use search::PrioritySearch;
use search::PrioritySearchAdder;

pub fn part_one (input: & Input) -> GenResult <u16> {
	calc_result (& input.risks)
}

pub fn part_two (input: & Input) -> GenResult <u16> {

	// replicate the grid five times right and down

	let y_range = input.risks.first_key ().y ..= input.risks.last_key ().y;
	let x_range = input.risks.first_key ().x ..= input.risks.last_key ().x;

	let y_iter = (0 .. 5).flat_map (|y_rep| y_range.clone ().map (move |y| (y_rep, y)));
	let x_iter = (0 .. 5).flat_map (|x_rep| x_range.clone ().map (move |x| (x_rep, x)));

	let risks = Grid::wrap_size (
		y_iter.cartesian_product (x_iter)
			.map (|((y_rep, y), (x_rep, x))| {
				let orig_risk = input.risks.get (Pos { y, x }).unwrap ();
				(orig_risk + y_rep + x_rep - 1) % 9 + 1
			})
			.collect (),
		input.risks.size () * 5);

	// apply the algorithm

	calc_result (& risks)

}

fn calc_result (risks: & Risks) -> GenResult <u16> {

	if risks.size ().y < 2 || risks.size ().x < 2 {
		return Err ("Minimum grid size is 2×2".into ());
	}

	// prepare offsets for efficient grid operations

	let offsets: [Offset; 4] =
		Pos::ZERO.adjacent_4 ().into_iter ()
			.map (|pos| risks.offset (pos).unwrap ())
			.array ();

	// set up priority search, with a function that tries to walk in four directions

	let mut search = PrioritySearch::with_grid_size (
	    risks.size (),
		|cur: Cursor, path_risk, mut adder: PrioritySearchAdder <Cursor, u16, _>| {
			for & adj_off in offsets.iter () {
			    let adj_cur = ok_or! (chk! (cur + adj_off), continue);
				let adj_risk = adj_cur.get (risks);
				let adj_path_risk = path_risk + adj_risk.pan_u16 ();
				adder.add (adj_cur, adj_path_risk);
			}
			(cur.pos (), path_risk)
		},
	);

	// add the starting position with a cost of zero

	search.push (risks.cursor (risks.first_key ()).unwrap (), 0);

	// apply the search and return the first result which reaches the target

	Ok (
		search
			.filter (|& (pos, _)| pos == risks.last_key ())
			.map (|(_, score)| score)
			.next ()
			.unwrap ()
	)

}
