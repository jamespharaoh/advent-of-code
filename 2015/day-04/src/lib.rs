//! Advent of Code 2015: Day 4: The Ideal Stocking Stuffer
//!
//! [https://adventofcode.com/2015/day/4](https://adventofcode.com/2015/day/4)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "The Ideal Stocking Stuffer";
	year = 2015;
	day = 4;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "run"; method = cli::run; ),
		( name = "find-test-case"; method = cli::find_test_case; ),
	];
}

pub mod logic {

	use super::*;
	use md5::Output;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <usize> {
		let input = Input {
			seed: input.seed.clone (),
			num_zeros: Some (input.num_zeros.unwrap_or (5)),
			.. * input
		};
		calc_result (& input)
	}

	pub fn part_two (input: & Input) -> GenResult <usize> {
		let input = Input {
			seed: input.seed.clone (),
			num_zeros: Some (input.num_zeros.unwrap_or (6)),
			.. * input
		};
		calc_result (& input)
	}

	pub fn calc_result (input: & Input) -> GenResult <usize> {
		let num_threads = cmp::min (get_num_threads ().unwrap_or (1), input.max_threads);
		if num_threads < 2 {
			calc_result_serial (& input.seed, input.num_zeros.unwrap ())
		} else {
			calc_result_parallel (& input.seed, input.num_zeros.unwrap (), num_threads)
		}
	}

	fn calc_result_serial (input: & str, num_zeros: usize) -> GenResult <usize> {
		let check_fn = check_zeros_fn (num_zeros);
		for base in (0 .. ).step_by (100) {
			let input_buf =
				format! ("{}{}", input, if base == 0 { 1 } else { base })
					.as_bytes ().to_vec ();
			let (min_len, loops) = if base == 0 {
				(input_buf.len () - 1, 99)
			} else {
				(input_buf.len () - 2, 100)
			};
			if let Some (loop_idx) = search_range (input_buf, min_len, loops, & check_fn) {
				return Ok (if base == 0 { loop_idx + 1 } else { base + loop_idx });
			}
		}
		Err ("No solution found") ?;
		unreachable! ();
	}

	fn get_num_threads () -> GenResult <usize> {
		use std::fs::File;
		use std::io::BufRead as _;
		use std::io::BufReader;
		let file = File::open ("/proc/cpuinfo") ?;
		let reader = BufReader::new (file);
		let mut num_threads = 0;
		for line in reader.lines () {
			let line = line ?;
			if ! line.starts_with ("processor\t: ") { continue }
			num_threads += 1;
		}
		Ok (num_threads)
	}

	fn calc_result_parallel (input: & str, num_zeros: usize, num_threads: usize) -> GenResult <usize> {
		const BATCH_SIZE: usize = 1000;
		#[ derive (Debug, Default) ]
		struct State {
			queue: VecDeque <usize>,
			solutions: Vec <usize>,
		}
		let state_mutex = Arc::new (Mutex::new (default::<State> ()));
		let queue_pushed = Arc::new (Condvar::new ());
		let queue_pulled = Arc::new (Condvar::new ());
		#[ allow (clippy::needless_collect) ]
		let join_handles = (0 .. num_threads).map (|_| {
			let input = input.to_owned ();
			let state_mutex = Arc::clone (& state_mutex);
			let queue_pushed = Arc::clone (& queue_pushed);
			let queue_pulled = Arc::clone (& queue_pulled);
			let check_fn = check_zeros_fn (num_zeros);
			thread::spawn (move || {
				loop {
					let mut state = state_mutex.lock ().unwrap ();
					let base = loop {
						if ! state.solutions.is_empty () { return }
						if let Some (next) = state.queue.pop_front () {
							queue_pulled.notify_one ();
							break next;
						}
						state = queue_pushed.wait (state).unwrap ();
					};
					drop (state);
					let input_buf =
						format! ("{}{}", input, if base == 0 { 1 } else { base })
							.as_bytes ().to_vec ();
					let (min_len, loops) = if base == 0 {
						(input_buf.len () - 1, BATCH_SIZE - 1)
					} else {
						(input_buf.len () - (input_buf.len () - input.len ()), BATCH_SIZE)
					};
					if let Some (loop_idx) = search_range (input_buf, min_len, loops, & check_fn) {
						let mut state = state_mutex.lock ().unwrap ();
						state.solutions.push (
							if base == 0 { loop_idx + 1 } else { base + loop_idx });
						queue_pulled.notify_all ();
						return;
					}
				}
			})
		}).collect::<Vec <_>> ();
		let mut state = state_mutex.lock ().unwrap ();
		for base in (0 .. ).step_by (BATCH_SIZE) {
			while state.queue.len () == num_threads * 8 && state.solutions.is_empty () {
				state = queue_pulled.wait (state).unwrap ();
			}
			if ! state.solutions.is_empty () { break }
			state.queue.push_back (base);
			queue_pushed.notify_one ();
		}
		queue_pushed.notify_all ();
		drop (state);
		join_handles.into_iter ().for_each (|join_handle| join_handle.join ().unwrap ());
		let state_mutex = Arc::try_unwrap (state_mutex).unwrap ();
		let state = state_mutex.into_inner ().unwrap ();
		let mut solutions = state.solutions;
		solutions.sort ();
		Ok (solutions.into_iter ().next ().ok_or ("No solution found") ?)
	}

	fn search_range (
		mut input_buf: Vec <u8>,
		min_len: usize,
		loops: usize,
		check_fn: & (dyn Fn (& Output) -> bool + Send + Sync),
	) -> Option <usize> {
		for loop_idx in 0 .. loops {
			let hash = md5::md5_hash (& input_buf);
			if check_fn (& hash) { return Some (loop_idx) }
			let mut num_zeros = 0;
			while input_buf.len () != min_len {
				let next_digit = match input_buf.pop ().unwrap () {
					b'0' => b'1', b'1' => b'2', b'2' => b'3',
					b'3' => b'4', b'4' => b'5', b'5' => b'6',
					b'6' => b'7', b'7' => b'8', b'8' => b'9',
					b'9' => {
						num_zeros += 1;
						continue;
					},
					_ => unreachable! (),
				};
				input_buf.push (next_digit);
				break;
			}
			if input_buf.len () == min_len { input_buf.push (b'1'); }
			input_buf.extend (iter::repeat (b'0').take (num_zeros));
		}
		None
	}

	fn check_zeros <const ZRS: usize> (hash: & Output) -> bool {
		for idx in 0 .. hash.len () {
			if ZRS == idx * 2 + 1 && hash [idx] & 0xf0 != 0 { return false }
			if ZRS >= idx * 2 + 2 && hash [idx] != 0 { return false }
		}
		true
	}

	fn check_zeros_fn (num_zeros: usize) -> Box <dyn Fn (& Output) -> bool + Send + Sync> {
		Box::new (match num_zeros {
			0 => check_zeros::<0>, 1 => check_zeros::<1>, 2 => check_zeros::<2>,
			3 => check_zeros::<3>, 4 => check_zeros::<4>, 5 => check_zeros::<5>,
			6 => check_zeros::<6>, 7 => check_zeros::<7>, 8 => check_zeros::<8>,
			9 => check_zeros::<9>, 10 => check_zeros::<10>, 11 => check_zeros::<11>,
			12 => check_zeros::<12>, 13 => check_zeros::<13>, 14 => check_zeros::<14>,
			15 => check_zeros::<15>, 16 => check_zeros::<16>, 17 => check_zeros::<17>,
			18 => check_zeros::<18>, 19 => check_zeros::<19>, 20 => check_zeros::<20>,
			21 => check_zeros::<21>, 22 => check_zeros::<22>, 23 => check_zeros::<23>,
			24 => check_zeros::<24>, 25 => check_zeros::<25>, 26 => check_zeros::<26>,
			27 => check_zeros::<27>, 28 => check_zeros::<28>, 29 => check_zeros::<29>,
			30 => check_zeros::<30>, 31 => check_zeros::<31>, 32 => check_zeros::<32>,
			_ => unreachable! (),
		})
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn check_zeros_fn () {
			fn with_zeros (zeros: usize) -> Output {
				let mut hex = String::new ();
				for _ in 0 .. zeros { hex.push ('0'); }
				while hex.len () < 32 { hex.push ('f'); }
				Output::from_hex (& hex).unwrap ()
			}
			for zeros in 0 ..= 32 {
				let check_fn = logic::check_zeros_fn (zeros);
				let should_pass = with_zeros (zeros);
				assert! (check_fn (& should_pass));
				if zeros > 0 {
					let should_fail = with_zeros (zeros - 1);
					assert! (! check_fn (& should_fail), "Should fail for {} zeros: {}", zeros, should_fail);
				}
			}
		}

		#[ test ]
		#[ should_panic ]
		fn check_zeros_fn_panic () {
			let _ = logic::check_zeros_fn (33);
		}

	}

}

pub mod model {

	use super::*;

	pub struct Input {
		pub seed: String,
		pub num_zeros: Option <usize>,
		pub max_threads: usize,
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let num_zeros = parser::input_param_opt (& mut input, "NUM_ZEROS=") ?;
			let max_threads = parser::input_param (& mut input, "MAX_THREADS=", usize::MAX) ?;
			let seed = input [0].to_owned ();
			Ok (Self { seed, num_zeros, max_threads })
		}
	}

}

#[ cfg (not (tarpaulin_include)) ]
mod cli {

	use super::*;
	use model::Input;
	use std::io::Read as _;
	use std::fs::File;
	use std::sync::atomic::AtomicBool;
	use std::sync::atomic::Ordering;

	#[ derive (clap::Parser) ]
	pub struct RunArgs {

		#[ clap (long, default_value = "2015/inputs/day-04") ]
		input: String,

		#[ clap (long) ]
		max_threads: Option <usize>,

		#[ clap (long) ]
		zeros: usize,

	}

	#[ allow (clippy::needless_pass_by_value) ]
	#[ allow (clippy::print_stdout) ]
	pub fn run (args: RunArgs) -> GenResult <()> {
		let input_string = fs::read_to_string (& args.input) ?;
		let input_lines: Vec <_> = input_string.trim ().split ('\n').collect ();
		println! ("Using input file: {}", & args.input);
		println! ("Looking for {} zeros", args.zeros);
		let input = Input {
			seed: input_lines [0].to_owned (),
			num_zeros: Some (args.zeros),
			max_threads: args.max_threads.unwrap_or (usize::MAX),
		};
		let result = logic::calc_result (& input) ?;
		println! ("Result: {}", result);
		Ok (())
	}

	#[ derive (Clone, clap::Parser) ]
	pub struct FindTestCaseArgs {

		#[ clap (long, default_value = "16") ]
		len: usize,

		#[ clap (long, default_value = "3999") ]
		max: usize,

	}

	#[ allow (clippy::needless_collect) ]
	#[ allow (clippy::needless_pass_by_value) ]
	#[ allow (clippy::unnecessary_wraps) ]
	pub fn find_test_case (args: FindTestCaseArgs) -> GenResult <()> {
		assert! (args.len >= 4);
		assert! (args.max < 0x_0100_0000);
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
					(buf [0].as_usize () << 16_u32)
						| (buf [1].as_usize () << 8_u32)
						| buf [2].as_usize ();
				if num <= args.max { break num }
			};
			let mut buffer = String::new ();
			for _ in 0 .. args.len - 4 {
				let ch = loop {
					let mut buf = [0; 1];
					assert_eq! (1, rand.read (& mut buf).unwrap ());
					let ch = buf [0].as_char ();
					if ('a' ..= 'z').contains (& ch) { break ch }
				};
				buffer.push (ch);
			}
			for chars in ('a' ..= 'z').permutations (4) {
				buffer.truncate (args.len - 4);
				for ch in chars { buffer.push (ch); }
				write! (buffer, "{}", num).unwrap ();
				let hash = md5::md5_hash (buffer.as_bytes ());
				if hash [0] == 0 && hash [1] == 0 && hash [2] == 0 {
					for num in 1 .. num {
						let buffer = format! ("{}{}", & buffer [0 .. args.len], num);
						let hash = md5::md5_hash (buffer.as_bytes ());
						if hash [0] == 0 && hash [1] == 0 && hash [2] & 0xf0 == 0
								&& hash [2] & 0xf != 0 {
							println! ("{:?}", & buffer [0 .. args.len]);
							complete.store (true, Ordering::Release);
							break 'OUTER;
						}
					}
				}
			}
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"xxebvswgx",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3206", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4846", puzzle.part_two (EXAMPLE));
	}

}
