use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let trees: Vec <Vec <bool>> = input_lines.into_iter ().map (
		|input_line| input_line.chars ().map (
			|ch| match ch {
				'#' => true,
				'.' => false,
				_ => panic! (),
			},
		).collect::<Vec <bool>> (),
	).collect ();
	let mut num_trees_product: u64 = 1;
	for dir in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter () {
		let width = trees [0].len ();
		let height = trees.len ();
		let mut x: usize = 0;
		let mut y: usize = 0;
		let mut num_trees: u64 = 0;
		while y < height {
			if trees [y] [x] { num_trees += 1 }
			x = (x + dir.0) % width;
			y += dir.1;
		}
		println! ("Number of trees for ({}, {}): {}", dir.0, dir.1, num_trees);
		num_trees_product *= num_trees;
	}
	println! ("Product of numbers of trees: {}", num_trees_product);
}
