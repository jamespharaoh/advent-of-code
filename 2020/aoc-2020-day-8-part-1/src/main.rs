use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let instrs: Result <Vec <Instr>, String> = input_lines.iter ().map (
		|instr_str| instr_str.parse (),
	).collect ();
	let instrs = instrs.unwrap ();
	let mut instr_idx: usize = 0;
	let mut prev_instrs: HashSet <usize> = HashSet::new ();
	let mut acc: i64 = 0;
	loop {
		if prev_instrs.contains (& instr_idx) { break }
		prev_instrs.insert (instr_idx);
		let instr = instrs [instr_idx];
		match instr.op {
			Op::Acc => { acc += instr.arg; instr_idx += 1 },
			Op::Jmp => instr_idx = (instr_idx as isize + instr.arg as isize) as usize,
			Op::Nop => instr_idx += 1,
		}
	}
	println! ("Accumlator value: {}", acc);
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Instr {
	op: Op,
	arg: i64,
}

impl FromStr for Instr {
	type Err = String;
	fn from_str (source: & str) -> Result <Instr, String> {
		let parts: Vec <& str> = source.split (' ').collect ();
		if parts.len () != 2 { return Err (format! ("Invalid instruction: {}", source)) }
		let op: Op = parts [0].parse () ?;
		let arg: i64 = parts [1].parse ().map_err (
			|error| format! ("Invalid argument: {}: {}", parts [1], error),
		) ?;
		Ok (Instr { op, arg })
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
enum Op { Acc, Jmp, Nop }

impl FromStr for Op {
	type Err = String;
	fn from_str (source: & str) -> Result <Op, String> {
		Ok (match source {
			"acc" => Op::Acc,
			"jmp" => Op::Jmp,
			"nop" => Op::Nop,
			_ => return Err (format! ("Invalid op: {}", source)),
		})
	}
}
