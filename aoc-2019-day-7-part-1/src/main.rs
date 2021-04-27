use std::fs;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let (phases, thrust) = calculate (& programme_source);
	println! ("Phase: {:?}, thurst: {}", phases, thrust);
}

fn calculate (programme: & str) -> (Vec <i64>, i64) {

	let programme = intcode::from_str (programme);

	let mut max_thrust = i64::MIN;
	let mut max_phases = None;

	'LOOP: for phase_combined in 0 .. 3125 {

		let phases: Vec <i64> = vec! [
			(phase_combined / 625) % 5,
			(phase_combined / 125) % 5,
			(phase_combined / 25) % 5,
			(phase_combined / 5) % 5,
			phase_combined % 5,
		];

		for i in 0 .. 4 {
			for j in i + 1 .. 5 {
				if phases [i] == phases [j] {
					continue 'LOOP;
				}
			}
		}

		let mut thrust = 0;

		for phase in phases.iter ().cloned () {
			let mut machine = intcode::Machine::new (programme.clone ())
				.with_input_buffer (vec! [phase, thrust].into ())
				.with_output_buffer ();
			machine.run ();
			thrust = machine.output_buffer ().pop_front ().unwrap ();
		}

		if thrust > max_thrust {
			max_thrust = thrust;
			max_phases = Some (phases.clone ());
		}

	}

	(max_phases.unwrap (), max_thrust)

}

#[ test ]
fn test_0 () {
	assert_eq! (
		(vec! [ 4, 3, 2, 1, 0 ], 43210),
		calculate ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
	);
}

#[ test ]
fn test_1 () {
	assert_eq! (
		(vec! [ 0, 1, 2, 3, 4 ], 54321),
		calculate ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
	);
}

#[ test ]
fn test_2 () {
	assert_eq! (
		(vec! [ 1, 0, 4, 3, 2 ], 65210),
		calculate ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
	);
}

