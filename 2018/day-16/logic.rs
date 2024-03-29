//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Opcode;
use model::Regs;
use model::Sample;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut count = 0_u32;
	for sample in input.samples.iter () {
		let mut num_matches = 0_u32;
		for opcode in Opcode::VARIANTS.iter ().copied () {
			if ! test_sample (opcode, sample) { continue }
			num_matches += 1;
		}
		if num_matches >= 3 { count += 1; }
	}
	Ok (count)
}

pub fn part_two (input: & Input) -> GenResult <u32> {

	for sample in input.samples.iter () {
		if ! (0 .. 16).contains (& sample.instr.op) {
			return Err (format! ("Opcode number out of range: {}", sample.instr.op).into ());
		}
	}
	for instr in input.instrs.iter () {
		if ! (0 .. 16).contains (& instr.op) {
			return Err (format! ("Opcode number out of range: {}", instr.op).into ());
		}
	}

	// gather data from the samples

	let mut opcode_to_num_arr = [[true; 16]; 16];
	let mut num_to_opcode_arr = [[true; 16]; 16];
	for sample in input.samples.iter () {
		for opcode in Opcode::VARIANTS.iter ().copied () {
			if ! test_sample (opcode, sample) {
				opcode_to_num_arr [opcode.idx ()] [sample.instr.op.pan_usize ()] = false;
				num_to_opcode_arr [sample.instr.op.pan_usize ()] [opcode.idx ()] = false;
			}
		}
	}

	// resolve the data to get mapping from num to opcode

	let mut num_to_opcode = [None; 16];
	loop {
		let mut progress = false;
		for match_opcode in Opcode::VARIANTS.iter ().copied () {
			if let Some (match_num) =
				opcode_to_num_arr [match_opcode.idx ()]
					.iter ().copied ()
					.enumerate ()
					.filter (|& (_, val)| val)
					.map (|(idx, _)| idx)
					.exactly_one () {
				for opcode in Opcode::VARIANTS.iter ().copied () {
					#[ allow (clippy::needless_range_loop) ]
					for num in 0_usize .. 16 {
						if opcode == match_opcode || num == match_num {
							opcode_to_num_arr [opcode.idx ()] [num] = false;
							num_to_opcode_arr [num] [opcode.idx ()] = false;
						}
					}
				}
				num_to_opcode [match_num] = Some (match_opcode);
				progress = true;
			}
		}
		if ! progress { break }
	}
	if num_to_opcode.iter ().any (Option::is_none) {
		return Err ("Failed to decode all instructions".into ());
	}
	let num_to_opcode = num_to_opcode.map (Option::unwrap);

	// run the specified program

	let mut regs = Regs::default ();
	for instr in input.instrs.iter () {
		let opcode = num_to_opcode [instr.op.pan_usize ()];
		regs = opcode.apply (instr.arg_a, instr.arg_b, instr.arg_c, regs) ?;
	}

	// return the value from register zero

	Ok (regs [0].pan_u32 ())

}

fn test_sample (opcode: Opcode, sample: & Sample) -> bool {
	let instr = sample.instr;
	let result = ok_or! (
		opcode.apply (instr.arg_a, instr.arg_b, instr.arg_c, sample.before),
		return false);
	result == sample.after
}
