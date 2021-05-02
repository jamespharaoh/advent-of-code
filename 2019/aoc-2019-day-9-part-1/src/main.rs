use std::fs;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let mut machine = intcode::Machine::new (programme);
	machine.queue_input (1);
	loop {
		match machine.run () {
			intcode::RunResult::Output (value) => println! ("Output: {}", value),
			intcode::RunResult::Halt => break,
		}
	}
}
