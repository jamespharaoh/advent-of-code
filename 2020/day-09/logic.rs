//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	Ok (find_invalid (& input.data).ok_or ("No solution found") ?)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let invalid = find_invalid (& input.data).ok_or ("No solution found") ?;
	let range = find_contiguous (& input.data, invalid).ok_or ("No solution found") ?;
	let min = input.data [range.clone ()].iter ().copied ().min ().unwrap ();
	let max = input.data [range].iter ().copied ().max ().unwrap ();
	Ok (min + max)
}

fn find_invalid (data: & [u64]) -> Option <u64> {
	let mut window = Vec::new ();
	for (idx, num) in data.iter ().copied ().enumerate () {
		if window.len () == 25 && ! contains_sum (window.iter ().map (|& (_, num)| num), num) {
			return Some (num);
		}
		if 25 <= idx { window.retain (|& (prev_idx, _)| prev_idx != idx - 25); }
		let insert_idx = window.binary_search_by_key (& num, |& (_, num)| num).either ();
		window.insert (insert_idx, (idx, num));
	}
	None
}

fn contains_sum (
	iter: impl Iterator <Item = u64> + Clone + ExactSizeIterator + DoubleEndedIterator,
	sum: u64,
) -> bool {
	let mut fwd_iter = iter.clone ().enumerate ().peekable ();
	let mut rev_iter = iter.enumerate ().rev ().peekable ();
	loop {
		let & (fwd_idx, fwd_val) = fwd_iter.peek ().unwrap ();
		let & (rev_idx, rev_val) = rev_iter.peek ().unwrap ();
		if fwd_idx == rev_idx { return false }
		match sum.cmp (& (fwd_val + rev_val)) {
			Ordering::Equal => return true,
			Ordering::Less => rev_iter.next ().unwrap (),
			Ordering::Greater => fwd_iter.next ().unwrap (),
		};
	}
}

fn find_contiguous (data: & [u64], target: u64) -> Option <Range <usize>> {
	for idx_0 in 0 .. data.len () - 2 {
		let mut sum = data [idx_0];
		for idx_1 in idx_0 + 1 .. data.len () {
			sum += data [idx_1];
			if target == sum { return Some (idx_0 .. idx_1 + 1) }
			if target < sum { break }
		}
	}
	None
}
