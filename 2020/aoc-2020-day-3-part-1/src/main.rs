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
	let width = trees [0].len ();
	let height = trees.len ();
	let mut x: usize = 0;
	let mut y: usize = 0;
	let mut num_trees: u64 = 0;
	while y < height {
		if trees [y] [x] { num_trees += 1 }
		x = (x + 3) % width;
		y += 1;
	}
	println! ("Number of trees: {}", num_trees);
}
