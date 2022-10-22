#![ cfg (not (tarpaulin_include)) ]

use super::*;

use std::path::PathBuf;

use input::Input;
use std::io::Read as _;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

args_decl! {
	pub struct RunArgs {
		input: Option <PathBuf>,
		max_threads: Option <u32>,
		num_zeros: Option <u32>,
	}
}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_path = puzzle_metadata ().find_input_or_arg (& args.input);
	let input_string = fs::read_to_string (& input_path) ?;
	let input_lines: Vec <_> = input_string.trim ().split ('\n').collect ();
	println! ("Using input file: {}", input_path.display ());
	println! ("Looking for {} zeros", args.num_zeros.unwrap_or (5));
	let mut input = Input::parse_from_lines (& input_lines) ?;
	if let Some (max_threads) = args.max_threads { input.params.max_threads = max_threads; }
	let result = logic::calc_result (& input, args.num_zeros.unwrap_or (5)) ?;
	println! ("Result: {}", result);
	Ok (())
}

args_decl! {
	#[ derive (Clone, Debug) ]
	pub struct FindTestCaseArgs {
		len: Option <usize>,
		max: Option <usize>,
	}
}

#[ allow (clippy::needless_collect) ]
#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::unnecessary_wraps) ]
pub fn find_test_case (args: FindTestCaseArgs) -> GenResult <()> {
	assert! (args.len.unwrap_or (16) >= 4);
	assert! (args.max.unwrap_or (3999) < 0x_0100_0000);
	let complete = Arc::new (AtomicBool::new (false));
	let threads: Vec <_> = (0_u32 .. 4_u32)
		.map (|_| {
			let args = args.clone ();
			let complete = Arc::clone (& complete);
			std::thread::spawn (move || find_test_case_worker (args, complete))
		})
		.collect ();
	threads.into_iter ().for_each (|thread| thread.join ().unwrap ());
	Ok (())
}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
#[ allow (clippy::string_slice) ]
fn find_test_case_worker (args: FindTestCaseArgs, complete: Arc <AtomicBool>) {
	let mut rand = File::open ("/dev/urandom").unwrap ();
	'OUTER: loop {
		if complete.load (Ordering::Acquire) { break }
		let num = loop {
			let mut buf = [0; 3];
			assert_eq! (3, rand.read (& mut buf).unwrap ());
			let num =
				(buf [0].pan_usize () << 16_u32)
					| (buf [1].pan_usize () << 8_u32)
					| buf [2].pan_usize ();
			if num <= args.max.unwrap_or (3999) { break num }
		};
		let mut buffer = String::new ();
		for _ in 0 .. args.len.unwrap_or (16) - 4 {
			let ch = loop {
				let mut buf = [0; 1];
				assert_eq! (1, rand.read (& mut buf).unwrap ());
				let ch = buf [0].pan_char ();
				if ('a' ..= 'z').contains (& ch) { break ch }
			};
			buffer.push (ch);
		}
		for chars in ('a' ..= 'z').permutations (4) {
			buffer.truncate (args.len.unwrap_or (16) - 4);
			for ch in chars { buffer.push (ch); }
			write! (buffer, "{}", num).unwrap ();
			let hash = md5::md5_hash (buffer.as_bytes ());
			if hash [0] == 0 && hash [1] == 0 && hash [2] == 0 {
				for num in 1 .. num {
					let buffer = format! ("{}{}", & buffer [0 .. args.len.unwrap_or (16)], num);
					let hash = md5::md5_hash (buffer.as_bytes ());
					if hash [0] == 0 && hash [1] == 0 && hash [2] & 0xf0 == 0
							&& hash [2] & 0xf != 0 {
						println! ("{:?}", & buffer [0 .. args.len.unwrap_or (16)]);
						complete.store (true, Ordering::Release);
						break 'OUTER;
					}
				}
			}
		}
	}
}
