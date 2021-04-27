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
			(phase_combined / 625) % 5 + 5,
			(phase_combined / 125) % 5 + 5,
			(phase_combined / 25) % 5 + 5,
			(phase_combined / 5) % 5 + 5,
			phase_combined % 5 + 5,
		];

		for i in 0 .. 4 {
			for j in i + 1 .. 5 {
				if phases [i] == phases [j] {
					continue 'LOOP;
				}
			}
		}

		let mut machines: Vec <intcode::Machine> = phases.iter ().cloned ().map (
			|phase| {
				let mut machine = intcode::Machine::new (programme.clone ());
				machine.queue_input (phase);
				machine
			},
		).collect ();

		let mut thrust = 0;

		'CYCLE: loop {
			for machine in machines.iter_mut () {
				machine.queue_input (thrust);
				thrust = match machine.run () {
					intcode::RunResult::Output (thrust) => thrust,
					intcode::RunResult::Halt => break 'CYCLE,
				};
			}
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
		(vec! [ 9, 8, 7, 6, 5 ], 139629729),
		calculate ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
	);
}

#[ test ]
fn test_1 () {
	assert_eq! (
		(vec! [ 9, 7, 8, 5, 6 ], 18216),
		calculate ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
	);
}
