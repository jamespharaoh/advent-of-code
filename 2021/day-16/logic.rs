#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use input::Input;
use model::BitIter;
use model::Packet;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let mut input_iter = BitIter::new (& input.data);
	let packet = Packet::decode (& mut input_iter).ok_or ("Failed to decode packet") ?;
	Ok (packet.version_sum ())
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut input_iter = BitIter::new (& input.data);
	let packet = Packet::decode (& mut input_iter).ok_or ("Failed to decode packet") ?;
	Ok (packet_eval (& packet).ok_or ("Failed to evaluate packet") ?)
}

fn packet_eval (packet: & Packet) -> Option <u64> {
	let child_vals = || packet.children.iter ().map (packet_eval);
	let child_at = |pos| child_vals ().nth (pos) ?;
	match packet.packet_type {
		0 => child_vals ().try_fold (0, |sum, item| {
			let item = item ?;
			chk! (sum + item).ok ()
		}),
		1 => child_vals ().try_fold (1, |prod, item| {
			let item = item ?;
			chk! (prod * item).ok ()
		}),
		2 => child_vals ().try_fold (u64::MAX, |min, item| {
			Some (cmp::min (min, item ?))
		}),
		3 => child_vals ().try_fold (u64::MIN, |max, item| {
			Some (cmp::max (max, item ?))
		}),
		4 => Some (packet.value),
		5 => {
			if packet.children.len () != 2 { return None }
			Some (u64::from (child_at (0) > child_at (1)))
		},
		6 => {
			if packet.children.len () != 2 { return None }
			Some (u64::from (child_at (0) < child_at (1)))
		},
		7 => {
			if packet.children.len () != 2 { return None }
			Some (u64::from (child_at (0) == child_at (1)))
		},
		_ => panic! (),
	}
}
