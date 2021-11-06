use std::error::Error;

use crate::shared::GridPosIter;
use crate::shared::Pos;

pub fn aoc2018_day6_part2 (input: & str) -> Result <(), Box <dyn Error>> {

	// collect points and work out size
	let mut width: i64 = 0;
	let mut height: i64 = 0;
	let mut posns: Vec <Pos> = Vec::new ();
	for line in input.trim ().split ("\n") {
		let line_parts: Vec <& str> = line.split (", ").collect ();
		if line_parts.len () != 2 { panic! () }
		let pos = Pos { x: line_parts [0].parse () ?, y: line_parts [1].parse () ? };
		if pos.x < 0 || pos.y < 0 { panic! () }
		if pos.x >= width { width = pos.x + 1 }
		if pos.y >= height { height = pos.y + 1 }
		posns.push (pos);
	}
	println! ("Grid size: {} × {} = {}", width, height, width * height);
	let extend_size = (10_000 + posns.len () as i64 - 1) / posns.len () as i64;
	println! ("Extend by 10000 ÷ {} = {} in four directions", posns.len (), extend_size);
	let new_width = width + extend_size * 2;
	let new_height = height + extend_size * 2;
	println! ("New grid size: {} × {} = {}", new_width, new_height, new_width * new_height);

	let mut area: u64 = 0;
	for pos_0 in GridPosIter::new (
		Pos::new (-100, -100),
		Pos::new (width, height),
	) {
		let distance: i64 = posns.iter ().map (
			|pos_1| (pos_0.x - pos_1.x).abs () + (pos_0.y - pos_1.y).abs (),
		).sum ();
		if distance <= 10000 {
			area += 1;
		}
	}
	println! ("Number of nearby squares: {}", area);

	Ok (())

}
