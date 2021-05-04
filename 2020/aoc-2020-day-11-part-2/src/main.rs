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
		let mut new_blocks = Vec::with_capacity (self.blocks.len ());
		for y in 0 .. self.height {
			for x in 0 .. self.width {
				let mut num_adj = 0;
				for dir in [
					Dir::UpLeft, Dir::Up, Dir::UpRight, Dir::Left, Dir::Right, Dir::DownLeft,
					Dir::Down, Dir::DownRight,
				].iter ().cloned () {
					let (mut adj_x, mut adj_y) = (x, y);
					loop {
						if let Some ((new_x, new_y)) = self.next_block ((adj_x, adj_y), dir) {
							adj_x = new_x; adj_y = new_y;
							if let Some (block) = self.get (adj_x, adj_y) {
								if block { num_adj += 1 }
								break;
							}
						} else { break }
					}
				}
				new_blocks.push (match self.get (x, y) {
					None => None,
					Some (false) => if num_adj == 0 { Some (true) } else { Some (false) },
					Some (true) => if num_adj >= 5 { Some (false) } else { Some (true) },
				});
			}
		}
		self.blocks = new_blocks
	}
	fn next_block (& self, (x, y): (usize, usize), dir: Dir) -> Option <(usize, usize)> {
		let up = y > 0;
		let down = y < self.height - 1;
		let left = x > 0;
		let right = x < self.width - 1;
		match dir {
			Dir::UpLeft => if up && left { Some ((x - 1, y - 1)) } else { None },
			Dir::Up => if up { Some ((x, y - 1)) } else { None },
			Dir::UpRight => if up && right { Some ((x + 1, y - 1)) } else { None },
			Dir::Left => if left { Some ((x - 1, y)) } else { None },
			Dir::Right => if right { Some ((x + 1, y)) } else { None },
			Dir::DownLeft => if down && left { Some ((x - 1, y + 1)) } else { None },
			Dir::Down => if down { Some ((x, y + 1)) } else { None },
			Dir::DownRight => if down && right { Some ((x + 1, y + 1)) } else { None },
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Dir { UpLeft, Up, UpRight, Left, Right, DownLeft, Down, DownRight }
