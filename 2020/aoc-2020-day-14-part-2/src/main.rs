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
			let mut floating: u64 = 0;
			for ch in captures.get (1).unwrap ().as_str ().chars () {
				keep <<= 1;
				set <<= 1;
				floating <<= 1;
				if ch == '0' { keep |= 1 }
				if ch == '1' { set |= 1 }
				if ch == 'X' { floating |= 1 }
			}
			let mut float_set: Vec <u64> = Vec::new ();
			float_set.push (set);
			for bit in 0 .. 36 {
				if floating & (1 << bit) != 0 {
					let mut float_set_new: Vec <u64> = Vec::with_capacity (float_set.len () * 2);
					for val in float_set.into_iter () {
						float_set_new.push (val);
						float_set_new.push (val | (1 << bit));
					}
					float_set = float_set_new;
				}
			}
			Op::Mask { keep, float_set }
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
	let mut float_set: & Vec <u64> = & Vec::new ();
	for op in ops.iter () {
		match op {
			Op::Mask { keep: new_keep, float_set: new_float_set } => {
				keep = * new_keep;
				float_set = new_float_set;
			},
			Op::Mem { mut addr, val } => {
				addr &= keep;
				for float_set_val in float_set.iter ().cloned () {
					let one_addr = addr | float_set_val;
					* mem.entry (one_addr).or_insert (0) = * val;
				}
			},
		}
	}
	let sum: u64 = mem.values ().sum ();
	println! ("Sum of memory values: {}", sum);
}

#[ derive (Clone, Debug, Eq, PartialEq) ]
enum Op {
	Mask { keep: u64, float_set: Vec <u64> },
	Mem { addr: u64, val: u64 },
}
