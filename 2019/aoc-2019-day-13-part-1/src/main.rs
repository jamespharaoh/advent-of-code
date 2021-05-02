use std::fs;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let mut machine = intcode::Machine::new (programme);
	let mut board: Vec <Vec <Symbol>> = Vec::new ();
	loop {

		let x = match machine.next () {
			Some (val) => val,
			None => break,
		};
		let y = machine.next ().unwrap ();
		let sym = machine.next ().unwrap ();

		while board.len () < y as usize + 1 { board.push (Vec::new ()) } 
		let board_line = & mut board [y as usize];
		while board_line.len () < x as usize + 1 { board_line.push (Symbol::Empty) }
		board_line [x as usize] = match sym {
			0 => Symbol::Empty,
			1 => Symbol::Wall,
			2 => Symbol::Block,
			3 => Symbol::Paddle,
			4 => Symbol::Ball,
			_ => panic! (),
		};

	}

	let num_blocks: usize = board.iter ().map (
		|board_line| board_line.iter ().filter (
			|symbol| ** symbol == Symbol::Block,
		).count (),
	).sum ();

	for board_line in board.iter () {
		for symbol in board_line {
			print! ("{}", match symbol {
				Symbol::Empty => "  ",
				Symbol::Wall => "██",
				Symbol::Block => "[]",
				Symbol::Paddle => "==",
				Symbol::Ball => "()",
			});
		}
		print! ("\n");
	}

	println! ("Number of blocks: {}", num_blocks);

}

#[ derive (Eq, PartialEq) ]
enum Symbol { Empty, Wall, Block, Paddle, Ball }
