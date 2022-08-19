#![ no_main ]

use aoc_fuzz::aoc_fuzz_mutator;
use libfuzzer_sys::fuzz_target;

use aoc_common::*;
use aoc_2018::day_16::*;
use input::Input;
use model::Instr;
use model::Regs;
use model::Sample;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (input) = Input::parse_from_lines (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});

aoc_fuzz_mutator! {

	transform_lifetimes = <'inp>;
	input_type = Input;

	pub transform remove (1) = |input, rng| {
		match rng.gen_range (0 .. 2) {
			0 => remove_sample (input, rng),
			1 => remove_instr (input, rng),
			_ => unreachable! (),
		};
	}

	transform modify_sample (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		let idx = rng.gen_range (0 .. input.samples.len ());
		let sample = & mut input.samples [idx];
		match rng.gen_range (0 .. 12) {
			0 => sample.before.reg_0 = rng.gen_range (0 .. 32),
			1 => sample.before.reg_1 = rng.gen_range (0 .. 32),
			2 => sample.before.reg_2 = rng.gen_range (0 .. 32),
			3 => sample.before.reg_3 = rng.gen_range (0 .. 32),
			4 => sample.instr.op = rng.gen_range (0 .. 16),
			5 => sample.instr.op = rng.gen_range (0 .. 32),
			6 => sample.instr.op = rng.gen_range (0 .. 32),
			7 => sample.instr.op = rng.gen_range (0 .. 32),
			8 => sample.after.reg_0 = rng.gen_range (0 .. 32),
			9 => sample.after.reg_1 = rng.gen_range (0 .. 32),
			10 => sample.after.reg_2 = rng.gen_range (0 .. 32),
			11 => sample.after.reg_3 = rng.gen_range (0 .. 32),
			_ => unreachable! (),
		}
	}

	transform add_sample (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		let sample = Sample {
			before: Regs {
				reg_0: rng.gen_range (0 .. 32),
				reg_1: rng.gen_range (0 .. 32),
				reg_2: rng.gen_range (0 .. 32),
				reg_3: rng.gen_range (0 .. 32),
			},
			instr: Instr {
				op: rng.gen_range (0 .. 16),
				arg_a: rng.gen_range (0 .. 32),
				arg_b: rng.gen_range (0 .. 32),
				arg_c: rng.gen_range (0 .. 32),
			},
			after: Regs {
				reg_0: rng.gen_range (0 .. 32),
				reg_1: rng.gen_range (0 .. 32),
				reg_2: rng.gen_range (0 .. 32),
				reg_3: rng.gen_range (0 .. 32),
			},
		};
		let new_idx = rng.gen_range (0 ..= input.samples.len ());
		input.samples.insert (new_idx, sample);
	}

	pub transform remove_sample (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		if input.samples.is_empty () { return Some (()) }
		let idx = rng.gen_range (0 .. input.samples.len ());
		input.samples.remove (idx);
	}

	transform sort_samples (1) = |input, _rng| {
		input.samples.sort ();
	}

	transform sort_instrs (1) = |input, _rng| {
		input.instrs.sort ();
	}

	transform modify_instr (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		let idx = rng.gen_range (0 .. input.instrs.len ());
		let instr = & mut input.instrs [idx];
		match rng.gen_range (0 .. 4) {
			0 => instr.op = rng.gen_range (0 .. 16),
			1 => instr.op = rng.gen_range (0 .. 32),
			2 => instr.op = rng.gen_range (0 .. 32),
			3 => instr.op = rng.gen_range (0 .. 32),
			_ => unreachable! (),
		}
	}

	transform add_instr (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		let instr = Instr {
			op: rng.gen_range (0 .. 16),
			arg_a: rng.gen_range (0 .. 32),
			arg_b: rng.gen_range (0 .. 32),
			arg_c: rng.gen_range (0 .. 32),
		};
		let new_idx = rng.gen_range (0 ..= input.instrs.len ());
		input.instrs.insert (new_idx, instr);
	}

	pub transform remove_instr (100 * 1, 10 * 10, 1 * 100) = |input, rng| {
		if input.instrs.is_empty () { return Some (()) }
		let idx = rng.gen_range (0 .. input.instrs.len ());
		input.instrs.remove (idx);
	}

	transform shuffle_samples (1) = |input, rng| {
		input.samples.shuffle (rng);
	}

	transform shuffle_instrs (1) = |input, rng| {
		input.instrs.shuffle (rng);
	}

}
