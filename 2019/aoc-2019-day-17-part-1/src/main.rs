use intcode::Machine;
use intcode::Mem;
use intcode::RunResult;
use std::fs;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let mut machine = Machine::new (programme);
	let mut output = String::new ();
	loop {
		match machine.run () {
			RunResult::Output (ch) => output.push (ch as u8 as char),
			RunResult::Input => panic! (),
			RunResult::Halt => break,
		}
	}
	let mut board: Vec <Vec <bool>> = Vec::new ();
	for line in output.split ('\n') {
		if line.trim ().len () == 0 { continue }
		let mut board_line = Vec::new ();
		for ch in line.chars () {
			board_line.push (ch != '.');
		}
		board.push (board_line);
	}
	let width = board [0].len ();
	let height = board.len ();
	let mut checksum: usize = 0;
	for x in 1 .. width - 1 {
		for y in 1 .. height - 1 {
			if ! board [y] [x] { continue }
			if ! board [y] [x - 1] { continue }
			if ! board [y] [x + 1] { continue }
			if ! board [y - 1] [x] { continue }
			if ! board [y + 1] [x] { continue }
			checksum += x * y;
		}
	}
	println! ("{}", output.trim ());
	println! ("Size: {}x{}", width, height);
	println! ("Checksum: {}", checksum);
}
