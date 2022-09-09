#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2018::day_21::*;
use aoc_common::*;
use aoc_fuzz::*;

use input::Input;
use model::Instr;
use model::Opcode;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.max_instrs = cmp::min (input.params.max_instrs, 1_000_000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});

aoc_fuzz_mutator! {

	transform_lifetimes = <'inp>;
	input_type = Input;

	transform add (1000 * 1, 10 * 10) = |input, rng| {
		let instr = Instr {
			opcode: Opcode::VARIANTS.choose (rng).copied ().unwrap (),
			arg_a: rng.gen_range (0 .. 99),
			arg_b: rng.gen_range (0 .. 99),
			arg_c: rng.gen_range (0 .. 99),
		};
		let mut instrs = input.instrs.to_vec ();
		let idx = rng.gen_range (0 ..= instrs.len ());
		instrs.insert (idx, instr);
		input.instrs = Rc::from (instrs);
	}

	transform modify (1000 * 1, 10 * 10) = |input, rng| {
		if input.instrs.is_empty () { return Some (()) }
		let mut instrs = input.instrs.to_vec ();
		let idx = rng.gen_range (0 .. instrs.len ());
		let instr = & mut instrs [idx];
		match rng.gen_range (0 .. 4) {
			0 => instr.opcode = Opcode::VARIANTS.choose (rng).copied ().unwrap (),
			1 => instr.arg_a = rng.gen_range (0 ..= 99),
			2 => instr.arg_b = rng.gen_range (0 ..= 99),
			3 => instr.arg_c = rng.gen_range (0 ..= 99),
			_ => unreachable! (),
		}
		input.instrs = Rc::from (instrs);
	}

	pub transform remove (1000 * 1, 10 * 5) = |input, rng| {
		if input.instrs.is_empty () { return Some (()) }
		let mut instrs = input.instrs.to_vec ();
		let idx = rng.gen_range (0 .. instrs.len ());
		instrs.remove (idx);
		input.instrs = Rc::from (instrs);
	}

	transform shuffle (1) = |input, rng| {
		let mut instrs = input.instrs.to_vec ();
		instrs.shuffle (rng);
		input.instrs = Rc::from (instrs);
	}

	transform sort (1) = |input, _rng| {
		let mut instrs = input.instrs.to_vec ();
		instrs.sort ();
		input.instrs = Rc::from (instrs);
	}

}
