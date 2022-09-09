//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::ArgType;
use model::Opcode;
use model::Regs;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <u64> {
	sanity_check (input) ?;
	SolutionsIter::new (input).next ().ok_or ("No solution found") ?
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	sanity_check (input) ?;
	SolutionsIter::new (input).last ().ok_or ("No solution found") ?
}

fn sanity_check (input: & Input) -> GenResult <()> {
	let regs = Regs::default ();
	regs.get (input.ip).ok_or ("Instruction pointer register is not valid") ?;
	for (instr_idx, instr) in input.instrs.iter ().enumerate () {
		if instr.opcode.arg_a () == ArgType::Reg {
			regs.get (instr.arg_a).ok_or (format! (
				"Instruction {instr_idx} references non-existent register {}", instr.arg_a)) ?;
		}
		if instr.opcode.arg_b () == ArgType::Reg {
			regs.get (instr.arg_b).ok_or (format! (
				"Instruction {instr_idx} references non-existent register {}", instr.arg_b)) ?;
		}
		regs.get (instr.arg_c).ok_or (format! (
			"Instruction {instr_idx} references non-existent register {}", instr.arg_c)) ?;
	}
	Ok (())
}

struct SolutionsIter <'inp> {
	input: & 'inp Input,
	todo: Vec <(Regs, Option <Val>)>,
	seen: HashSet <Val>,
	remain: u64,
}

impl <'inp> SolutionsIter <'inp> {
	fn new (input: & 'inp Input) -> Self {
		let mut todo = Vec::new ();
		todo.push ((Regs::default (), None));
		let seen = HashSet::new ();
		let remain = input.params.max_instrs;
		SolutionsIter { input, todo, seen, remain }
	}
}

impl <'inp> Iterator for SolutionsIter <'inp> {
	type Item = GenResult <Val>;
	fn next (& mut self) -> Option <GenResult <Val>> {
		while let Some ((mut regs, reg_0)) = self.todo.pop () {
			loop {
				if self.remain == 0 {
					return Some (Err ("Instruction limit reached".into ()));
				}
				self.remain -= 1;
				let instr_idx = regs.get (self.input.ip).unwrap ().as_usize ();
				if self.input.instrs.len () <= instr_idx.as_usize () {
					return reg_0.map_or_else (
						|| Some (Err ("Halted with indeterminate value in reg 0".into ())),
						|reg_0| Some (Ok (reg_0)));
				}
				let instr = self.input.instrs [instr_idx];
				if instr.arg_c == 0 {
					return Some (Err ("Don't know how to handle write to reg 0".into ()));
				}
				if (instr.opcode.arg_a () == ArgType::Reg && instr.arg_a == 0)
						|| (instr.opcode.arg_b () == ArgType::Reg && instr.arg_b == 0) {
					if instr.opcode != Opcode::Eqrr || (instr.arg_a == 0 && instr.arg_b == 0) {
						return Some (Err (
							format! ("Don't know how to handle {:?}", instr).into ()));
					}
					if reg_0.is_some () {
						return Some (Err ("Reg 0 compared again after matching as equal".into ()));
					}
					let other_reg = if instr.arg_a == 0 { instr.arg_b } else { instr.arg_a };
					let other = regs.get (other_reg).unwrap ();
					if ! self.seen.insert (other) { break; }
					regs.set (self.input.ip, regs.get (self.input.ip).unwrap () + 1).unwrap ();
					regs.set (instr.arg_c, 0).unwrap ();
					self.todo.push ((regs, None));
					regs.set (instr.arg_c, 1).unwrap ();
					self.todo.push ((regs, Some (other)));
					break;
				}
				regs = instr.apply (regs).unwrap ();
				regs.set (self.input.ip, regs.get (self.input.ip).unwrap () + 1).unwrap ();
			}
		}
		None
	}
}
