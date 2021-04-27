use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

fn main () {

	let input_mem = load ();

	for noun in 0 .. 100 {
		for verb in 0 .. 100 {
			let mut mem = input_mem.clone ();
			mem [1] = noun;
			mem [2] = verb;
			run (& mut mem);
			let result = mem [0];
			if result != 19690720 { continue; }
			println! ("noun={} verb={}", noun, verb);
		}
	}

}

fn load () -> Vec <i64> {
	let mut mem: Vec <i64> = Vec::new ();
	let file = File::open ("input").unwrap ();
	let mut reader = BufReader::new (file);
	loop {
		let mut buf: Vec <u8> = Vec::new ();
		let num_read = reader.read_until (b',', & mut buf).unwrap ();
		if num_read == 0 { break; }
		while ! buf.is_empty () && ! buf.last ().unwrap ().is_ascii_digit () {
			buf.pop ().unwrap ();
		}
		let str = String::from_utf8 (buf).unwrap ();
		let num: i64 = str.parse ().unwrap ();
		mem.push (num);
	}
	mem
}

fn run (mem: & mut Vec <i64>) {
	let mut pos = 0;
	loop {
		match mem [pos] {
			1 => { // add
				let left = mem [pos + 1] as usize;
				let right = mem [pos + 2] as usize;
				let result = mem [pos + 3] as usize;
				mem [result] = mem [left] + mem [right];
				pos += 4;
			},
			2 => { // multiply
				let left = mem [pos + 1] as usize;
				let right = mem [pos + 2] as usize;
				let result = mem [pos + 3] as usize;
				mem [result] = mem [left] * mem [right];
				pos += 4;
			},
			99 => { // halt
				return;
			},
			opcode => {
				panic! ("invalid opcode: {}", opcode);
			},
		}
	}
}
