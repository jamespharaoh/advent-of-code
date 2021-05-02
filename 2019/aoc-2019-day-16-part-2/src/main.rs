use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io;
use std::io::Write as _;
use std::iter;
use std::fs;
use std::str;
use std::str::FromStr;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let message = decode (input_string.trim ());
	println! ("Message: {}", message);
}

fn decode (input_string: & str) -> String {
	let offset: usize = input_string [0 .. 7].parse ().unwrap ();
	let base_signal: Signal = input_string.parse ().unwrap ();
	let mut signal = Signal {
		data: iter::repeat_with (
			|| base_signal.data.iter (),
		).take (10000).flatten ().cloned ().collect (),
	};
	for index in 0 .. 100 {
		print! ("\rProgress: {}% ...\x1b[K", index);
		io::stdout ().flush ().unwrap ();
		signal.apply_fft ();
	}
	print! ("\rProgress: done\x1b[K\n");
	let mut message = String::new ();
	for datum in signal.data [offset .. offset + 8].iter () {
		message.push ((datum + ('0' as u8)) as char);
	}
	message
}

struct Signal { data: Vec <u8> }

impl Signal {
	fn apply_fft (& mut self) {
		let mut sums: Vec <i64> = self.data.iter ().rev ().scan (
			0,
			|sum, item| {
				* sum += (* item) as i64;
				Some (* sum)
			},
		).collect ();
		sums.reverse ();
		let mut new_data = Vec::new ();
		for dst_index in 0 .. self.data.len () {
			let mut phase: u8 = 1;
			let mut sum: i64 = 0;
			let mut offset = dst_index;
			while offset < self.data.len () {
				match phase {
					0 => sum += sums [offset],
					1 => sum += sums [offset],
					2 => sum -= sums [offset],
					3 => sum -= sums [offset],
					_ => unreachable! (),
				}
				offset += dst_index + 1;
				phase = (phase + 1) % 4;
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
	assert_eq! ("84462026", decode ("03036732577212944063491565474664"));
}

#[ test ]
fn test_1 () {
	assert_eq! ("78725270", decode ("02935109699940807407585447034323"));
}

#[ test ]
fn test_2 () {
	assert_eq! ("53553731", decode ("03081770884921959731165446850517"));
}
