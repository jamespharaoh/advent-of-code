use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <i64> {
	let mut nums: Vec <(i64, i64)> =
		input.nums.iter ()
			.enumerate ()
			.map (|(idx, & num)| (num.pan_i64 (), idx.pan_i64 ()))
			.collect ();
	move_all (& mut nums);
	Ok (calc_result (& nums))
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	let mut nums: Vec <(i64, i64)> =
		input.nums.iter ()
			.enumerate ()
			.map (|(idx, & num)| GenOk ((chk! (num.pan_i64 () * 811_589_153) ?, idx.pan_i64 ())))
			.try_collect () ?;
	for _ in 0_i32 .. 10_i32 {
		move_all (& mut nums);
	}
	Ok (calc_result (& nums))
}

fn calc_result (nums: & [(i64, i64)]) -> i64 {
	let pos = nums.iter ().position (|& (num_val, _)| num_val == 0).unwrap ().pan_i64 ();
	let (num_1k, _) = nums [(pos + 1000).rem_euclid (nums.len ().pan_i64 ()).pan_usize ()];
	let (num_2k, _) = nums [(pos + 2000).rem_euclid (nums.len ().pan_i64 ()).pan_usize ()];
	let (num_3k, _) = nums [(pos + 3000).rem_euclid (nums.len ().pan_i64 ()).pan_usize ()];
	num_1k + num_2k + num_3k
}

fn move_all (nums: & mut Vec <(i64, i64)>) {
	for idx in 0 .. nums.len ().pan_i64 () {
		move_one (nums, idx);
	}
}

fn move_one (nums: & mut Vec <(i64, i64)>, idx: i64) {
	let pos = nums.iter ().position (|& (_, num_idx)| num_idx == idx).unwrap ().pan_i64 ();
	let (num_val, num_idx) = nums.remove (pos.pan_usize ());
	let pos = (pos + num_val).rem_euclid (nums.len ().pan_i64 ());
	nums.insert (pos.pan_usize (), (num_val, num_idx));
}
