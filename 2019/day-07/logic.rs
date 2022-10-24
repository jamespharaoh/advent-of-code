//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Cpu;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	let mut todo: Vec <(TinyVec <Val, 5>, Val)> = Vec::new ();
	todo.push ((TinyVec::new (), Val::ZERO));
	let mut highest = Val::ZERO;
	while let Some ((phases, value)) = todo.pop () {
		if phases.len () == 5 {
			if value <= highest { continue }
			highest = value;
			continue;
		}
		for phase in Val::ZERO .. Val::FIVE {
			if phases.contains (& phase) { continue }
			let mut phases = phases.clone ();
			phases.push (phase);
			let mut cpu = Cpu::new (input.data.clone ());
			cpu.set_max_ops (input.params.max_ops_one);
			cpu.input (phase);
			cpu.input (value);
			let value = cpu.run ().output () ?.ok_or ("Program generated no output") ?;
			todo.push ((phases, value));
		}
	}
	Ok (highest)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	let mut highest = Val::ZERO;
	let mut todo: Vec <TinyVec <Val, 5>> = Vec::new ();
	todo.push (TinyVec::new ());
	while let Some (phases) = todo.pop () {
		if phases.len () < 5 {
			for phase in Val::FIVE ..= Val::NINE {
				if phases.contains (& phase) { continue }
				let mut phases = phases.clone ();
				phases.push (phase);
				todo.push (phases);
			}
			continue;
		}
		let mut cpus: Vec <Cpu> =
			phases.iter ().copied ()
				.map (|phase| {
					let mut cpu = Cpu::new (input.data.clone ());
					cpu.set_max_ops (input.params.max_ops_two);
					cpu.input (phase);
					cpu
				})
				.collect ();
		let mut value = Val::ZERO;
		'CALC: loop {
			for cpu in cpus.iter_mut () {
				cpu.input (value);
				value = some_or! (cpu.run ().output () ?, break 'CALC);
			}
		}
		highest = cmp::max (highest, value);
	}
	Ok (highest)
}
