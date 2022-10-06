use super::*;

use input::Input;
use md5::Output;

const BATCH_SIZE: u32 = 10_000;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, input.params.num_zeros_one)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, input.params.num_zeros_two)
}

pub fn calc_result (input: & Input, num_zeros: u32) -> GenResult <u32> {
	let check_fn = check_zeros_fn (num_zeros);
	let mut base_iter = (1 .. ).step_by (BATCH_SIZE.pan_usize ());
	let num_threads = parallel::num_cpus_max (input.params.max_threads);
	let result = if 1 < num_threads {
		let seed = Arc::from (& * input.seed);
		base_iter
			.thread_map (
				num_threads.pan_usize (),
				move |base| search_range (& seed, base .. base + BATCH_SIZE, & check_fn))
			.flatten ()
			.next ()
	} else {
		base_iter
			.find_map (|base| search_range (& input.seed, base .. base + BATCH_SIZE, & check_fn))
	};
	Ok (result.ok_or ("No solution found") ?)
}

fn search_range (
	input: & str,
	range: Range <u32>,
	check_fn: & (dyn Fn (& Output) -> bool + Send + Sync),
) -> Option <u32> {
	let base = range.start;
	let mut input_buf = format! ("{input}{base}").as_bytes ().to_vec ();
	for num in range {
		let hash = md5::md5_hash (& input_buf);
		if check_fn (& hash) { return Some (num) }
		let mut num_zeros = 0;
		while input.len () < input_buf.len () {
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
		if input_buf.len () == input.len () { input_buf.push (b'1'); }
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

fn check_zeros_fn (num_zeros: u32) -> fn (& Output) -> bool {
	match num_zeros {
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
	}
}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn check_zeros_fn () {
		fn with_zeros (zeros: u32) -> Output {
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
