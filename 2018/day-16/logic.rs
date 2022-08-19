//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::ArgType;
use model::Op;
use model::Opcode;
use model::Regs;
use model::Sample;
use model::Val;

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
				opcode_to_num_arr [opcode.idx ()] [sample.instr.op.as_usize ()] = false;
				num_to_opcode_arr [sample.instr.op.as_usize ()] [opcode.idx ()] = false;
			}
		}
	}

	// resolve the data to get mapping from num to opcode

	let mut num_to_opcode = [None; 16];
	loop {
		let mut progress = false;
		for match_opcode in Opcode::VARIANTS.iter ().copied () {
			if let Ok (match_num) =
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
		let opcode = num_to_opcode [instr.op.as_usize ()];
		regs = apply (opcode, instr.arg_a, instr.arg_b, instr.arg_c, regs)
			.ok_or (format! ("Failed to execute instruction: {instr}")) ?;
	}

	// return the value from register zero

	Ok (regs.reg_0.as_u32 ())

}

fn test_sample (opcode: Opcode, sample: & Sample) -> bool {
	let instr = sample.instr;
	let result = some_or! (
		apply (opcode, instr.arg_a, instr.arg_b, instr.arg_c, sample.before),
		return false);
	result == sample.after
}

fn apply (
	opcode: Opcode,
	arg_a: Val,
	arg_b: Val,
	arg_c: Val,
	mut regs: Regs,
) -> Option <Regs> {

	let val_a = match opcode.arg_a () {
		ArgType::Reg => regs.get (arg_a),
		ArgType::Imm => Some (arg_a),
		ArgType::Ignore => None,
	};

	let val_b = match opcode.arg_b () {
		ArgType::Reg => regs.get (arg_b),
		ArgType::Imm => Some (arg_b),
		ArgType::Ignore => None,
	};

	let val_c = match opcode.op () {
		Op::Add => Val::add_2 (val_a ?, val_b ?).ok () ?,
		Op::Mul => Val::mul_2 (val_a ?, val_b ?).ok () ?,
		Op::Ban => val_a ? & val_b ?,
		Op::Bor => val_a ? | val_b ?,
		Op::Set => val_a ?,
		Op::Gt => if val_a ? > val_b ? { 1 } else { 0 },
		Op::Eq => if val_a ? == val_b ? { 1 } else { 0 },
	};

	regs.set (arg_c, val_c) ?;

	Some (regs)

}
