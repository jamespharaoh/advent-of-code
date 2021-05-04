use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	lazy_static! {
		static ref MASK_RE: Regex = Regex::new (r"^mask = ([01X]{36})$").unwrap ();
		static ref MEM_RE: Regex = Regex::new (r"^mem\[(\d+)\] = (\d+)$").unwrap ();
	}
	let ops: Vec <Op> = input_lines.iter ().map (|line| {
		if let Some (captures) = MASK_RE.captures (line) {
			let mut keep: u64 = 0;
			let mut set: u64 = 0;
			for ch in captures.get (1).unwrap ().as_str ().chars () {
				keep <<= 1;
				set <<= 1;
				if ch == 'X' { keep |= 1 }
				if ch == '1' { set |= 1 }
			}
			Op::Mask { keep, set }
		} else if let Some (captures) = MEM_RE.captures (line) {
			let addr = captures.get (1).unwrap ().as_str ().parse ().unwrap ();
			let val = captures.get (2).unwrap ().as_str ().parse ().unwrap ();
			Op::Mem { addr, val }
		} else {
			panic! ();
		}
	}).collect ();
	let mut mem: HashMap <u64, u64> = HashMap::new ();
	let mut keep: u64 = 0;
	let mut set: u64 = 0;
	for op in ops.iter ().cloned () {
		match op {
			Op::Mask { keep: new_keep, set: new_set } => {
				keep = new_keep;
				set = new_set;
			},
			Op::Mem { addr, val } => {
				let cell = mem.entry (addr).or_insert (0);
				* cell = (val & keep) | set;
			},
		}
	}
	let sum: u64 = mem.values ().sum ();
	println! ("Sum of memory values: {}", sum);
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
enum Op {
	Mask { keep: u64, set: u64 },
	Mem { addr: u64, val: u64 },
}
