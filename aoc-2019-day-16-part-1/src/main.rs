use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fs;
use std::str::FromStr;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let mut signal: Signal = input_string.trim ().parse ().unwrap ();
	for _ in 0 .. 100 {
		signal.apply_fft ();
	}
	println! ("Result signal: {}", & signal.to_string () [0 .. 8]);
}

struct Signal { data: Vec <u8> }

impl Signal {
	fn apply_fft (& mut self) {
		let old_data = & self.data;
		let mut new_data: Vec <u8> = Vec::with_capacity (old_data.len ());
		for dst_index in 0 .. old_data.len () {
			let mut sum: i64 = 0;
			for src_index in 0 .. old_data.len () {
				let phase = (src_index + 1) / (dst_index + 1) % 4;
				let co = match phase {
					0 => 0,
					1 => 1,
					2 => 0,
					3 => -1,
					_ => unreachable! (),
				};
				sum += co * old_data [src_index] as i64;
				//println! ("dst: {}, srt: {}, phase: {}, coefficient: {}", dst_index, src_index, phase, co);
			}
			new_data.push ((sum.abs () % 10) as u8);
		}
		self.data = new_data;
	}
}

impl FromStr for Signal {
	type Err = String;
	fn from_str (input: & str) -> Result <Signal, String> {
		let mut data: Vec <u8> = Vec::new ();
		for ch in input.chars () {
			if ! ch.is_ascii_digit () {
				return Err (format! ("Invalid char: {}", ch));
			}
			data.push ((ch as u8) - ('0' as u8));
		}
		Ok (Signal { data })
	}
}

impl Display for Signal {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		for value in self.data.iter () {
			write! (formatter, "{}", value) ?;
		}
		Ok (())
	}
}

#[ test ]
fn test_0 () {
	let mut signal: Signal = "12345678".parse ().unwrap ();
	signal.apply_fft (); assert_eq! ("48226158", signal.to_string ());
	signal.apply_fft (); assert_eq! ("34040438", signal.to_string ());
	signal.apply_fft (); assert_eq! ("03415518", signal.to_string ());
	signal.apply_fft (); assert_eq! ("01029498", signal.to_string ());
}
