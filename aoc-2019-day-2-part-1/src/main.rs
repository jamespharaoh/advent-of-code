use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

fn main () {

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

	mem [1] = 12;
	mem [2] = 2;

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
				break;
			},
			opcode => {
				panic! ("invalid opcode: {}", opcode);
			},
		}
	}

	println! ("Result: {}", mem [0]);

}
