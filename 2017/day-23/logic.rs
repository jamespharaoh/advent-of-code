//! Logic for solving the puzzles

use super::*;
use cpu::Cpu;
use cpu::Instr;
use cpu::Val;
use cpu::cpu_optimise;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut cpu = Cpu::new (input.instrs.as_slice ());
	cpu.set_limit (100_000);
	let mut num_muls = 0;
	while cpu.can_step () {
		if matches! (cpu.next_instr (), Some (Instr::Mul (_, _))) { num_muls += 1; }
		cpu.step () ?;
	}
	Ok (num_muls)
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	let mut cpu = Cpu::new (input.instrs.as_slice ());
	cpu.set_limit (10_000);
	cpu.set_reg ('a', 1) ?;
	exec_optimised (& mut cpu, false) ?;
	Ok (cpu.get_reg ('h') ?)
}

fn exec_optimised (cpu: & mut Cpu, verbose: bool) -> GenResult <Option <Val>> {
	while cpu.can_step () {
		#[ allow (clippy::print_stdout) ]
		if verbose {
			println! (
				"{:4}  {:8} {:8} {:8} {:8} {:8} {:8} {:8} {:8}  {}",
				cpu.next () + 1,
				cpu.get_reg ('a') ?,
				cpu.get_reg ('b') ?,
				cpu.get_reg ('c') ?,
				cpu.get_reg ('d') ?,
				cpu.get_reg ('e') ?,
				cpu.get_reg ('f') ?,
				cpu.get_reg ('g') ?,
				cpu.get_reg ('h') ?,
				cpu.next_instr ().unwrap ());
		}
		cpu_optimise! {
			cpu,
			regs = [ target, out, temp, outer, inner ],
			instrs = [
				Set (dst outer, imm 2),
				Set (dst inner, imm 2),
				Set (dst temp, src outer),
				Mul (dst temp, src inner),
				Sub (dst temp, src target),
				Jnz (src temp, imm 2),
				Set (dst out, imm 0),
				Sub (dst inner, imm -1),
				Set (dst temp, src inner),
				Sub (dst temp, src target),
				Jnz (src temp, imm -8),
				Sub (dst outer, imm -1),
				Set (dst temp, src outer),
				Sub (dst temp, src target),
				Jnz (src temp, imm -13),
			],
			run = {
				let target_val = cpu.load_reg (target);
				cpu.store_reg (temp, 0);
				cpu.store_reg (outer, cpu.load_reg (target));
				cpu.store_reg (inner, cpu.load_reg (target));
				let prime = some_or! (is_prime (target_val), break);
				if ! prime { cpu.store_reg (out, 0); }
				cpu.set_next (cpu.next () + 15);
				cpu.set_limit (cpu.limit ().saturating_sub (1));
				if verbose {
					println! ("Optimised check prime (target={target_val}, prime={prime})");
				}
			},
		};
		if let Some (val) = cpu.step () ? { return Ok (Some (val)) }
	}
	Ok (None)
}

fn is_prime (val: Val) -> Option <bool> {
	if val < 2 { return None }
	for div in PRIMES {
		if val <= div * div { break }
		if val % div == 0 { return Some (false) }
	}
	for div in (1001 ..= val / 1000).step_by (2) {
		if val <= div * div { break }
		if val % div == 0 { return Some (false) }
	}
	Some (true)
}

const PRIMES: [Val; 168] = [
	2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
	101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
	197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
	311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
	431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
	557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
	661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
	809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
	937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];
