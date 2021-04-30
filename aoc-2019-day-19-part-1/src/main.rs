use intcode::Machine;
use intcode::RunResult;
use std::fs;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let mut num_affected: u64 = 0;
	for x in 0 .. 50 {
		for y in 0 .. 50 {
			let mut machine = Machine::new (programme.clone ());
			machine.input (x);
			machine.input (y);
			match machine.run () {
				RunResult::Output (1) => num_affected += 1,
				RunResult::Output (0) => (),
				unexpected => panic! ("Unexpected result: {:?}", unexpected),
			}
		}
	}
	println! ("Number affected by beam: {}", num_affected);
}
