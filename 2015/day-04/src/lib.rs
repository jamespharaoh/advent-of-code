//! Advent of Code 2015: Day 4: The Ideal Stocking Stuffer
//!
//! [https://adventofcode.com/2015/day/4](https://adventofcode.com/2015/day/4)

#![ allow (clippy::needless_collect) ]
#![ allow (clippy::needless_range_loop) ]

use aoc_common::*;

puzzle_info! {
	name = "The Ideal Stocking Stuffer";
	year = 2015;
	day = 4;
	part_one = |input| logic::part_one (input [0]);
	part_two = |input| logic::part_two (input [0]);
	commands = [
		( name = "run"; method = cli::run; ),
	];
}

pub mod md5;

pub mod logic {

	use super::*;
	use md5::Output;

	pub fn part_one (input: & str) -> GenResult <usize> {
		calc_result_serial (input, 5)
	}

	pub fn part_two (input: & str) -> GenResult <usize> {
		calc_result_parallel (input, 6)
	}

	pub fn calc_result_serial (input: & str, num_zeros: usize) -> GenResult <usize> {
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

	pub fn calc_result_parallel (input: & str, num_zeros: usize) -> GenResult <usize> {
		let num_threads = get_num_threads ().unwrap_or (1);
		if num_threads < 2 { return calc_result_serial (input, num_zeros) }
		const BATCH_SIZE: usize = 1000;
		#[ derive (Debug, Default) ]
		struct State {
			queue: VecDeque <usize>,
			solutions: Vec <usize>,
		}
		let state_mutex = Arc::new (Mutex::new (default::<State> ()));
		let queue_pushed = Arc::new (Condvar::new ());
		let queue_pulled = Arc::new (Condvar::new ());
		let join_handles = (0 .. num_threads).map (|_| {
			let input = input.to_string ();
			let state_mutex = state_mutex.clone ();
			let queue_pushed = queue_pushed.clone ();
			let queue_pulled = queue_pulled.clone ();
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

	pub fn search_range (
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

	pub fn check_zeros <const ZRS: usize> (hash: & Output) -> bool {
		for idx in 0 .. hash.len () {
			if ZRS == idx * 2 + 1 && hash [idx] & 0xf0 != 0 { return false }
			if ZRS >= idx * 2 + 2 && hash [idx] != 0 { return false }
		}
		true
	}

	pub fn check_zeros_fn (num_zeros: usize) -> Box <dyn Fn (& Output) -> bool + Send + Sync> {
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
			_ => panic! (),
		})
	}

}

mod cli {

	use super::*;

	#[ derive (clap::Parser) ]
	pub struct RunArgs {

		#[ clap (long, default_value = "inputs/day-04") ]
		input: String,

		#[ clap (long) ]
		threads: Option <usize>,

		#[ clap (long) ]
		zeros: usize,

	}

	pub fn run (args: RunArgs) -> GenResult <()> {
		let input_string = fs::read_to_string (& args.input) ?;
		let input_lines: Vec <_> = input_string.trim ().split ('\n').collect ();
		println! ("Using input file: {}", & args.input);
		println! ("Looking for {} zeros", args.zeros);
		let result = logic::calc_result_parallel (input_lines [0], args.zeros) ?;
		println! ("Result: {}", result);
		Ok (())
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (609043, logic::part_one ("abcdef") ?);
		Ok (())
	}

}
