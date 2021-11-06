use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::iter;

use crate::shared::Grid;
use crate::shared::Pos;

pub fn aoc2018_day6_part1 (input: & str) -> Result <String, Box <dyn Error>> {

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
	println! ("Grid size: {} x {}", width, height);

	// initialize grid
	let mut grid: Grid <Option <u8>> = Grid::builder ()
		.width (width)
		.height (height)
		.unbounded ()
		.default (None)
		.build ();
	let mut id: u8 = 0;
	for pos in posns.iter ().cloned () {
		if id == u8::MAX { panic! () }
		grid.set (pos, Some (id));
		id += 1;
	}

	// spread out one square at a time
	loop {
		let mut grid_temp = grid.clone ();
		let mut progress = false;
		for pos in grid.posns () {
			if grid.get (pos).is_some () { continue }
			let mut neighbours: HashSet <u8> = HashSet::new ();
			for neighbour_pos in pos.four_neighbours () {
				if let Some (neighbour_val) = grid.get (neighbour_pos) {
					if neighbour_val == u8::MAX { continue }
					neighbours.insert (neighbour_val);
				}
			}
			if neighbours.len () == 1 {
				grid_temp.set (pos, Some (neighbours.into_iter ().next ().unwrap ()));
			}  else if neighbours.len () > 1 {
				grid_temp.set (pos, Some (u8::MAX));
			}
			progress = true;
		}
		grid = grid_temp;
		if ! progress { break }
	}

	// count areas
	let mut areas: Vec <u64> = iter::repeat (0).take (posns.len () + 1).collect ();
	for value in grid.values () {
		let value = value.unwrap ();
		if value == u8::MAX { continue }
		areas [value as usize] += 1;
	}
	let mut areas: HashMap <u8, u64> = areas.into_iter ().enumerate ().map (
		|(index, value)| (index as u8, value),
	).collect ();

	// remove areas which reach the edge
	for x in 0 .. width {
		if let Some (value) = grid.get (Pos::new (x, 0)) {
			areas.remove (& value);
		}
		if let Some (value) = grid.get (Pos::new (x, height - 1)) {
			areas.remove (& value);
		}
	}
	for y in 0 .. width {
		if let Some (value) = grid.get (Pos::new (0, y)) {
			areas.remove (& value);
		}
		if let Some (value) = grid.get (Pos::new (width - 1, y)) {
			areas.remove (& value);
		}
	}

	// find largest remaining area
	let (index, size) = areas.iter ().max_by_key (|(_, size)| * size).unwrap ();
	let index = * index;

	println! ("Identified largest finite area");
	println! ("Index: {}", index);
	println! ("Position: {}, {}", posns [index as usize].x, posns [index as usize].y);

	Ok (format! ("{}", size))

}
