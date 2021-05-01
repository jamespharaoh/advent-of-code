use std::mem;

fn main () {
	let num_bugs = count_after (0b_10011_00111_01011_00110_01111, 200);
	println! ("Number of bugs after 200 iterations: {}", num_bugs);
}

fn count_after (initial: u32, times: u32) -> u32 {
'OUTER:
	for num_layers in (5 .. ).step_by (2) {
		let mut boards: Vec <u32> = vec! [0; num_layers];
		let middle = boards.len () / 2;
		boards [middle] = initial;
		let mut boards_temp = boards.clone ();
		for _ in 0 .. times {
			print_boards (& boards);
			for layer in 0 .. boards.len () {
				let outer = if layer > 0 { boards [layer - 1] } else { 0 };
				let current = boards [layer];
				let inner = boards.get (layer + 1).cloned ().unwrap_or (0);
				let mut temp = current;
				for x in 0 .. 5 {
					for y in 0 .. 5 {
						if x == 2 && y == 2 { continue }
						let mut adj: u32 = 0;

						if y == 0 {
							if get_bit (outer, 2, 1) { adj += 1 }
						} else if x == 2 && y == 3 {
							if get_bit (inner, 0, 4) { adj += 1 }
							if get_bit (inner, 1, 4) { adj += 1 }
							if get_bit (inner, 2, 4) { adj += 1 }
							if get_bit (inner, 3, 4) { adj += 1 }
							if get_bit (inner, 4, 4) { adj += 1 }
						} else if get_bit (current, x, y - 1) { adj += 1 }

						if y == 4 {
							if get_bit (outer, 2, 3) { adj += 1 }
						} else if x == 2 && y == 1 {
							if get_bit (inner, 0, 0) { adj += 1 }
							if get_bit (inner, 1, 0) { adj += 1 }
							if get_bit (inner, 2, 0) { adj += 1 }
							if get_bit (inner, 3, 0) { adj += 1 }
							if get_bit (inner, 4, 0) { adj += 1 }
						} else if get_bit (current, x, y + 1) { adj += 1 }

						if x == 0 {
							if get_bit (outer, 1, 2) { adj += 1 }
						} else if x == 3 && y == 2 {
							if get_bit (inner, 4, 0) { adj += 1 }
							if get_bit (inner, 4, 1) { adj += 1 }
							if get_bit (inner, 4, 2) { adj += 1 }
							if get_bit (inner, 4, 3) { adj += 1 }
							if get_bit (inner, 4, 4) { adj += 1 }
						} else if get_bit (current, x - 1, y) { adj += 1 }

						if x == 4 {
							if get_bit (outer, 3, 2) { adj += 1 }
						} else if x == 1 && y == 2 {
							if get_bit (inner, 0, 0) { adj += 1 }
							if get_bit (inner, 0, 1) { adj += 1 }
							if get_bit (inner, 0, 2) { adj += 1 }
							if get_bit (inner, 0, 3) { adj += 1 }
							if get_bit (inner, 0, 4) { adj += 1 }
						} else if get_bit (current, x + 1, y) { adj += 1 }

						if get_bit (current, x, y) {
							if adj != 1 { clear_bit (& mut temp, x, y) }
						} else {
							if adj == 1 || adj == 2 { set_bit (& mut temp, x, y) }
						}

					}
				}
				boards_temp [layer] = temp;
			}
			mem::swap (& mut boards, & mut boards_temp);
			if boards [0] != 0 || boards [boards.len () - 1] != 0 { continue 'OUTER }
		}
		let num_bugs: u32 = boards.iter ().cloned ().map (u32::count_ones).sum ();
		print_boards (& boards);
		return num_bugs;
	}
	unreachable! ();
}

fn print_boards (boards: & [u32]) {
	if cfg! (not (debug_assertions)) { return }
	for _ in 0 .. (boards.len () * 12 - 2) { print! ("=") }
	print! ("\n");
	for layer in 0 .. boards.len () {
		print! ("Layer {:03}   ", layer)
	}
	print! ("\n");
	for y in 0 .. 5 {
		for board in boards.iter ().cloned () {
			for x in 0 .. 5 {
				if x == 2 && y == 2 {
					print! ("??");
				}
				else if get_bit (board, x, y) {
					print! ("\x1b[48;5;237m🐛\x1b[0m");
				} else {
					print! ("\x1b[48;5;237m  \x1b[0m");
				}
			}
			print! ("  ");
		}
		print! ("\n");
	}
}

fn get_bit (board: u32, x: u32, y: u32) -> bool {
	board & (1 << (y * 5 + x)) != 0
}

fn set_bit (board: & mut u32, x: u32, y: u32) {
	* board |= 1 << (y * 5 + x);
}

fn clear_bit (board: & mut u32, x: u32, y: u32) {
	* board &= ! (1 << (y * 5 + x));
}

#[ test ]
fn test_0 () {
	assert_eq! (99, count_after (0b_00001_00100_11001_01001_10000, 10));
}
