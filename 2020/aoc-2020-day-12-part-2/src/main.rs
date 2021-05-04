use std::fs;

fn main () {
	let output_string = fs::read_to_string ("input").unwrap ();
	let output_lines: Vec <& str> = output_string.trim ().split ('\n').collect ();
	let (x, y) = calculate (& output_lines);
	println! ("Final coordinates: {}, {}", x, y);
	println! ("Final distance: {}", x.abs () + y.abs ());
}

fn calculate <LineRef: AsRef <str>> (lines: & [LineRef]) -> (i64, i64) {
	let ops: Vec <Op> = lines.iter ().map (|line| {
		let line = line.as_ref ();
		let op_ch = line.chars ().next ().unwrap ();
		let (_, op_arg_str) = line.split_at (1);
		let op_arg: i64 = op_arg_str.parse ().unwrap ();
		match op_ch {
			'N' => Op::North (op_arg),
			'S' => Op::South (op_arg),
			'W' => Op::West (op_arg),
			'E' => Op::East (op_arg),
			'L' => match op_arg {
				90 => Op::Left,
				180 => Op::Reverse,
				270 => Op::Right,
				_ => panic! (),
			},
			'R' => match op_arg {
				90 => Op::Right,
				180 => Op::Reverse,
				270 => Op::Left,
				_ => panic! (),
			},
			'F' => Op::Forwards (op_arg),
			_ => panic! (),
		}
	}).collect ();
	let mut x: i64 = 0;
	let mut y: i64 = 0;
	let mut rel_x: i64 = 10;
	let mut rel_y: i64 = -1;
	for op in ops.iter ().cloned () {
		match op {
			Op::North (arg) => rel_y -= arg,
			Op::South (arg) => rel_y += arg,
			Op::West (arg) => rel_x -= arg,
			Op::East (arg) => rel_x += arg,
			Op::Left => { let (new_x, new_y) = (rel_y, - rel_x); rel_x = new_x; rel_y = new_y },
			Op::Right => { let (new_x, new_y) = (- rel_y, rel_x); rel_x = new_x; rel_y = new_y },
			Op::Reverse => { rel_x = - rel_x; rel_y = - rel_y },
			Op::Forwards (arg) => { x += rel_x * arg; y += rel_y * arg },
		}
	}
	(x, y)
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Op {
	North (i64),
	South (i64),
	West (i64),
	East (i64),
	Left,
	Right,
	Reverse,
	Forwards (i64),
}

#[ test ]
fn test_0 () {
	let (x, y) = calculate (& vec! ["F10", "N3", "F7", "R90", "F11"]);
	assert_eq! (286, x.abs () + y.abs ());
}
