//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Instr;
use model::Opcode;
use model::Regs;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let regs: Regs= Regs::default ();
	calc_result (input, regs)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut regs: Regs = Regs::default ();
	regs.set (0, 1).unwrap ();
	calc_result (input, regs)
}

#[ allow (clippy::unneeded_field_pattern) ]
fn calc_result (input: & Input, mut regs: Regs) -> GenResult <u64> {
	let sum_of_factors_reg = get_sum_of_factors_reg (input);
	let mut num_instrs = 0;
	#[ allow (clippy::while_let_loop) ]
	loop {
		let instr_val = some_or! (regs.get (input.ip), break);
		let instr_idx = ok_or! (instr_val.to_usize (), break);
		if input.instrs.len () <= instr_idx { break }
		if instr_idx == 1 {
			if let Some (reg) = sum_of_factors_reg {
				let target = regs.get (reg).unwrap ();
				return Ok (calc_sum_of_factors (target));
			}
		}
		let instr = input.instrs [instr_idx];
		regs = instr.apply (regs) ?;
		let new_ip = regs.get (input.ip).unwrap () + 1;
		regs.set (input.ip, new_ip).unwrap ();
		num_instrs += 1;
		if num_instrs == input.params.max_instrs {
			return Err ("Giving up after max instrs".into ());
		}
	}
	Ok (regs.get (0).unwrap ())
}

fn get_sum_of_factors_reg (input: & Input) -> Option <Val> {
	if input.instrs.len () != 36 { return None }
	match <[Instr; 16]>::try_from (& input.instrs [1 .. 17]).unwrap () {
		[
			Instr { opcode: Opcode::Seti, arg_a: 1, arg_b: _, arg_c: reg_a_0 },
			Instr { opcode: Opcode::Seti, arg_a: 1, arg_b: _, arg_c: reg_b_0 },
			Instr { opcode: Opcode::Mulr, arg_a: reg_a_1, arg_b: reg_b_1, arg_c: reg_c_0 },
			Instr { opcode: Opcode::Eqrr, arg_a: reg_c_1, arg_b: reg_d_0, arg_c: reg_c_2 },
			Instr { opcode: Opcode::Addr, arg_a: reg_c_3, arg_b: reg_e_0, arg_c: reg_e_1 },
			Instr { opcode: Opcode::Addi, arg_a: reg_e_2, arg_b: 1, arg_c: reg_e_3 },
			Instr { opcode: Opcode::Addr, arg_a: reg_a_2, arg_b: 0, arg_c: 0 },
			Instr { opcode: Opcode::Addi, arg_a: reg_b_2, arg_b: 1, arg_c: reg_b_3 },
			Instr { opcode: Opcode::Gtrr, arg_a: reg_b_4, arg_b: reg_d_1, arg_c: reg_c_4 },
			Instr { opcode: Opcode::Addr, arg_a: reg_e_4, arg_b: reg_c_5, arg_c: reg_e_5 },
			Instr { opcode: Opcode::Seti, arg_a: 2, arg_b: _, arg_c: reg_e_6 },
			Instr { opcode: Opcode::Addi, arg_a: reg_a_3, arg_b: 1, arg_c: reg_a_4 },
			Instr { opcode: Opcode::Gtrr, arg_a: reg_a_5, arg_b: reg_d_2, arg_c: reg_c_6 },
			Instr { opcode: Opcode::Addr, arg_a: reg_c_7, arg_b: reg_e_7, arg_c: reg_e_8 },
			Instr { opcode: Opcode::Seti, arg_a: 1, arg_b: _, arg_c: reg_e_9 },
			Instr { opcode: Opcode::Mulr, arg_a: reg_e_10, arg_b: reg_e_11, arg_c: reg_e_12 },
		] if reg_a_0 != reg_b_0 && reg_a_0 != reg_c_0 && reg_a_0 != reg_d_0 && reg_a_0 != reg_e_0
				&& reg_b_0 != reg_c_0 && reg_b_0 != reg_d_0 && reg_b_0 != reg_e_0
				&& reg_c_0 != reg_d_0 && reg_c_0 != reg_e_0 && reg_d_0 != reg_e_0 && reg_a_0 != 0
				&& reg_b_0 != 0 && reg_c_0 != 0 && reg_d_0 != 0 && reg_e_0 != 0
				&& reg_e_0 == input.ip
				&& [ reg_a_0, reg_a_1, reg_a_2, reg_a_3, reg_a_4, reg_a_5 ].into_iter ()
					.all_equal ()
				&& [ reg_b_0, reg_b_1, reg_b_2, reg_b_3, reg_b_4 ].into_iter ().all_equal ()
				&& [ reg_c_0, reg_c_1, reg_c_2, reg_c_3, reg_c_4, reg_c_5, reg_c_6, reg_c_7 ]
					.into_iter ().all_equal ()
				&& [ reg_d_0, reg_d_1, reg_d_2 ].into_iter ().all_equal ()
				&& [ reg_e_0, reg_e_1, reg_e_2, reg_e_3, reg_e_4, reg_e_5, reg_e_6, reg_e_7,
					reg_e_8, reg_e_9, reg_e_10, reg_e_11, reg_e_12 ].into_iter ().all_equal ()
			=> Some (reg_d_0),
		_ => None,
	}
}

fn calc_sum_of_factors (target: u64) -> u64 {
	let mut result = target + 1;
	let mut div = 2_u64;
	loop {
		let co_div = target / div;
		let rem = target % div;
		if co_div < div { break }
		if rem == 0 {
			result += div;
			if div != co_div { result += co_div; }
		}
		div += 1;
	}
	result
}
