//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Instr;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let mut memory: HashMap <u64, u64> = HashMap::new ();
	let mut keep = 0;
	let mut set = 0;
	for & instr in & input.instrs {
		match instr {
			Instr::Mask { mask, val } => (keep, set) = (! mask, val),
			Instr::Store { addr, val } => { memory.insert (addr, val & keep | set); },
		}
	}
	Ok (memory.values ().sum ())
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut stores: Vec <(u64, u64, u64)> = Vec::new ();
	let mut keep = 0;
	let mut set = 0;
	let mut fixed = 0;
	for & instr in & input.instrs {
		match instr {
			Instr::Mask { mask, val } => {
				keep = mask & ! val;
				set = mask & val;
				fixed = mask;
			},
			Instr::Store { addr, val } => {
				stores.push ((addr & keep | set, fixed, val));
			},
		}
	}
	let mut memory = HashSet::new ();
	let mut sum = 0;
	let mut num_iters = 0;
	for & (addr, fixed, val) in stores.iter ().rev () {
		let mut float = 0;
		while float < (1 << 36_u32) {
			if num_iters == 200_000_u32 {
				return Err ("Giving up due to max iterations".into ());
			}
			num_iters += 1;
			if memory.insert (addr | float) {
				if 100_000_usize < memory.len () {
					return Err ("Giving up due to memory limit".into ());
				}
				sum += val;
			}
			float = ((float | fixed) + 1) & ! fixed;
		}
	}
	Ok (sum)
}
