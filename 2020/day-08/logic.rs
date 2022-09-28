//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Instr;
use model::Op;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	let (_, acc) = run (& input.instrs) ?;
	Ok (acc)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	let mut instrs = input.instrs.clone ();
	let end = Val::from_usize (instrs.len ()).unwrap ();
	let mut result = None;
	for change_idx in 0 .. instrs.len () {
		let old_op = instrs [change_idx].op;
		let new_op = match old_op {
			Op::Acc => continue,
			Op::Jmp => Op::Nop,
			Op::Nop => Op::Jmp,
		};
		instrs [change_idx].op = new_op;
		let (pos, acc) = run (& instrs) ?;
		if pos == end {
			if result.is_some () { return Err ("Multiple solutions found".into ()) }
			result = Some (acc);
		}
		instrs [change_idx].op = old_op;
	}
	Ok (result.ok_or ("No solution found") ?)
}

/// Run the programme until termination or looping, returning final position and accumulator
///
fn run (instrs: & [Instr]) -> GenResult <(Val, Val)> {
	let mut acc = Val::ZERO;
	let mut seen: HashSet <usize> = HashSet::new ();
	let mut pos = Val::ZERO;
	loop {
		if pos < Val::ZERO || instrs.len () <= pos.pan_usize () { break }
		let instr_idx = pos.pan_usize ();
		if ! seen.insert (instr_idx) { break }
		let instr = instrs [instr_idx];
		if instr.op == Op::Acc { acc = Val::add_2 (acc, instr.arg) ?; }
		pos = Val::add_2 (pos, if instr.op == Op::Jmp { instr.arg } else { Val::ONE }) ?;
	}
	Ok ((pos, acc))
}
