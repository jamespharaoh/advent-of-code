use super::*;

use input::Input;
use model::Grid;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, false)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, true)
}

#[ allow (clippy::let_underscore_must_use) ]
fn calc_result (input: & Input, floor: bool) -> GenResult <u32> {
	let (min_y, max_y) =
		input.traces.iter ()
			.flat_map (|trace| trace.points.iter ().copied ())
			.map (|pos| pos.y)
			.min_max ()
			.unwrap_or ((500, 500));
	if ! (1 <= min_y && max_y <= 200) {
		return Err ("Y coordinate range must be between 1 and 200".into ());
	}
	let mut grid = Grid::new_range (Pos::new (0, 500 - max_y - 2), Pos::new (max_y + 3, 500 + max_y + 3)) ?;
	for trace in & input.traces {
		let mut points_iter = trace.points.iter ().copied ();
		let Some (mut pos) = points_iter.next () else {
			return Err ("Traces must have at least one point".into ());
		};
		let _ = grid.try_set (pos, Tile::Rock);
		for next in points_iter {
			while pos != next {
				if pos.x == next.x {
					if pos.y < next.y { pos.y += 1; } else { pos.y -= 1; }
				} else if pos.y == next.y {
					if pos.x < next.x { pos.x += 1; } else { pos.x -= 1; }
				} else {
					return Err ("Adjacent points in traces must be vertically or horizontally aligned".into ());
				}
				let _ = grid.try_set (pos, Tile::Rock);
			}
		}
	}
	if floor {
		for x in grid.start ().x .. grid.end ().x {
			let _ = grid.try_set (Pos::new (grid.last_key ().y, x), Tile::Rock);
		}
	}
	let offsets = [
		grid.offset (Pos::new (1, 0)).unwrap (),
		grid.offset (Pos::new (1, -1)).unwrap (),
		grid.offset (Pos::new (1, 1)).unwrap (),
	];
	let mut num_resting = 0;
	let mut todo = vec! [ grid.cursor (Pos::new (0, 500)).unwrap () ];
	'OUTER: while let Some (mut cur) = todo.pop () {
		'DESCEND: loop {
			for offset in offsets {
				let Ok (new_cur) = chk! (cur + offset) else { break 'OUTER };
				if new_cur.get (& grid) != Tile::Air { continue }
				todo.push (cur);
				cur = new_cur;
				continue 'DESCEND;
			}
			grid.set (cur.pos (), Tile::Sand);
			num_resting += 1;
			break;
		}
	}
	Ok (num_resting)
}
