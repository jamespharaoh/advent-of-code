use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let width = input_lines [0].len ();
	let height = input_lines.len ();
	let blocks: Vec <Option <bool>> = input_lines.iter ().map (
		|line| line.chars (),
	).flatten ().map (
		|ch| match ch {
			'.' => None,
			'L' => Some (false),
			_ => panic! (),
		},
	).collect ();
	let mut board = Board { width, height, blocks };
	let mut num_iterations = 0;
	loop {
		let last_board = board.clone ();
		board.update ();
		if board == last_board { break }
		num_iterations += 1
	}
	println! ("Number of iterations: {}", num_iterations);
	let num_occupied = board.blocks.iter ().filter (|block| ** block == Some (true)).count ();
	println! ("Number of occupied seats: {}", num_occupied);
}

#[ derive (Clone, Debug, Eq, PartialEq) ]
struct Board {
	width: usize,
	height: usize,
	blocks: Vec <Option <bool>>,
}

impl Board {
	fn get (& self, x: usize, y: usize) -> Option <bool> {
		self.blocks [y * self.width + x]
	}
	fn update (& mut self) {
		let mut new_blocks = self.blocks.clone ();
		for x in 0 .. self.width {
			for y in 0 .. self.height {
				let mut num_adj = 0;
				if x > 0 && y > 0 && self.get (x - 1, y - 1) == Some (true) { num_adj += 1 }
				if y > 0 && self.get (x, y - 1) == Some (true) { num_adj += 1 }
				if x < self.width - 1 && y > 0 && self.get (x + 1, y - 1) == Some (true) { num_adj += 1 }
				if x > 0 && self.get (x - 1, y) == Some (true) { num_adj += 1 }
				if x < self.width - 1 && self.get (x + 1, y) == Some (true) { num_adj += 1 }
				if x > 0 && y < self.height - 1 && self.get (x - 1, y + 1) == Some (true) { num_adj += 1 }
				if y < self.height - 1 && self.get (x, y + 1) == Some (true) { num_adj += 1 }
				if x < self.width - 1 && y < self.height - 1 && self.get (x + 1, y + 1) == Some (true) { num_adj += 1 }
				new_blocks [y * self.width + x] = match self.get (x, y) {
					None => None,
					Some (false) => if num_adj == 0 { Some (true) } else { Some (false) },
					Some (true) => if num_adj >= 4 { Some (false) } else { Some (true) },
				}
			}
		}
		self.blocks = new_blocks
	}
}
