use std::collections::HashSet;

fn main () {
	let mut layouts: HashSet <u32> = HashSet::new ();
	let mut board: u32 = 0b_10011_00111_01011_00110_01111;
	loop {
		for y in 0 .. 5 {
			for x in 0 .. 5 {
				let bit = y * 5 + x;
				if get_bit (& board, bit) {
					print! ("#");
				} else {
					print! (".");
				}
			}
			print! ("\n");
		}
		println! ("----------");
		let mut new_board: u32 = board;
		for x in 0 .. 5 {
			for y in 0 .. 5 {
				let bit = y * 5 + x;
				let mut adj: u32 = 0;
				if y > 0 && get_bit (& board, bit - 5) { adj += 1 }
				if y < 4 && get_bit (& board, bit + 5) { adj += 1 }
				if x > 0 && get_bit (& board, bit - 1) { adj += 1 }
				if x < 4 && get_bit (& board, bit + 1) { adj += 1 }
				if get_bit (& board, bit) {
					if adj != 1 { clear_bit (& mut new_board, bit) }
				} else {
					if adj == 1 || adj == 2 { set_bit (& mut new_board, bit) }
				}
			}
		}
		board = new_board;
		if layouts.contains (& board) {
			println! ("Repeated layout: {}", board);
			return;
		}
		layouts.insert (board);
	}
}

fn get_bit (board: & u32, bit: u32) -> bool {
	* board & (1 << bit) != 0
}

fn set_bit (board: & mut u32, bit: u32) {
	* board |= 1 << bit;
}

fn clear_bit (board: & mut u32, bit: u32) {
	* board &= ! (1 << bit);
}
