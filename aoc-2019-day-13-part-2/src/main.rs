use console::Key;
use console::Term;
use intcode::RunResult;
use std::collections::VecDeque;
use std::fs;
use std::io::Write as _;
use std::thread;
use std::time::Duration;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let mut programme = intcode::from_str (& programme_source);
	programme [0] = 2;
	let mut machine = intcode::Machine::new (programme);
	let mut score: i64 = 0;
	let mut output_buf: VecDeque <i64> = VecDeque::new ();
	let mut term = Term::stdout ();
	let mut ball = 0;
	let mut paddle = 0;
	term.clear_screen ();
	loop {

		let mut needs_input = false;
		let mut halted = false;
		match machine.run () {
			RunResult::Output (val) => output_buf.push_back (val),
			RunResult::Input => needs_input = true,
			RunResult::Halt => halted = true,
		};

		if needs_input || halted {
			while output_buf.len () >= 3 {
				let x = output_buf.pop_front ().unwrap ();
				let y = output_buf.pop_front ().unwrap ();
				let sym = output_buf.pop_front ().unwrap ();
				if x == -1 && y == 0 {
					score = sym;
				} else {
					term.move_cursor_to (x as usize * 2, y as usize + 1);
					write! (& mut term, "{}", match sym {
						0 => "  ",
						1 => "██",
						2 => "📦",
						3 => "══",
						4 => "⚽",
						_ => panic! (),
					}).unwrap ();
					if sym == 3 { paddle = x }
					if sym == 4 { ball = x }
				}
			}
			term.move_cursor_to (0, 0);
			term.clear_line ();
			write! (& mut term, "SCORE: {}", score).unwrap ();
			if halted { break }
			thread::sleep (Duration::from_millis (1));
			machine.queue_input ((ball - paddle).signum ());
		}

	}

	term.clear_screen ();
	println! ("Final score: {}", score);

}
