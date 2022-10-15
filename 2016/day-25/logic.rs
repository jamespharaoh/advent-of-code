use super::*;

use cpu::Cpu;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <i32> {
	for start in 1_i32 ..= input.params.limit {
		let mut cpu = Cpu {
			instrs: Rc::new (input.instrs.clone ()),
			reg_a: start,
			limit: input.params.ops_limit,
			.. default ()
		};
		let mut expect = 0_i32;
		let mut num = 0_u32;
		let mut states = HashSet::new ();
		while let Some (out) = cpu.exec () ? {
			if out != expect { break }
			expect = i32::from (expect == 0_i32);
			num += 1;
			let state = (expect, cpu.next, cpu.reg_a, cpu.reg_b, cpu.reg_c, cpu.reg_d);
			if num >= 2 && ! states.insert (state) {
				return Ok (start);
			}
		}
	}
	Err ("No solution found".into ())
}
