use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		key_indexes (input, 0)
			.nth (input.params.num_keys.pan_usize () - 1)
			.unwrap ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		key_indexes (input, input.params.hash_reps)
			.nth (input.params.num_keys.pan_usize () - 1)
			.unwrap ()
	)
}

struct KeyIndexIter <HashesIter: Iterator> {
	hashes: MultiPeek <HashesIter>,
	idx: u32,
	num_next: u32,
}

#[ inline ]
fn key_indexes (
	input: & Input,
	hash_reps: u32,
) -> KeyIndexIter <impl Iterator <Item = [u8; 32]>> {
	KeyIndexIter {
		hashes: hashes_iter (input, hash_reps).multipeek (),
		idx: 0,
		num_next: input.params.num_next,
	}
}

impl <HashesIter> Iterator for KeyIndexIter <HashesIter>
	where HashesIter: Iterator <Item = [u8; 32]> {

	type Item = u32;

	#[ inline ]
	fn next (& mut self) -> Option <u32> {
		loop {
			let hash_0 = self.hashes.next ().unwrap ();
			if let Some (ch_0) = find_triple (& hash_0) {
				for _ in 0 .. self.num_next.pan_usize () {
					if ! has_quintuple (self.hashes.peek ().unwrap (), ch_0) { continue }
					self.idx += 1;
					return Some (self.idx - 1);
				}
			}
			self.idx += 1;
		}
	}

}

fn find_triple (hash: & [u8; 32]) -> Option <u8> {
	hash.iter ()
		.array_windows ()
		.filter (|& [& a, & b, & c]| a == b && a == c)
		.map (|[& a, _, _]| a)
		.next ()
}

fn has_quintuple (hash: & [u8; 32], ch_0: u8) -> bool {
	hash.iter ()
		.fold ((false, 0_u32), |(matched, count), & ch_1|
			if matched || (ch_0 == ch_1 && count == 4) { (true, 0) }
			else if ch_0 == ch_1 { (false, count + 1) }
			else { (false, 0) }
		).0
}

fn hashes_iter (
	input: & Input,
	hash_reps: u32,
) -> impl Iterator <Item = [u8; 32]> {
	let salt = input.salt.to_owned ();
	let batch_size = input.params.batch_size;
	let map_fn = move |num_start| {
		let mut buffer = salt.clone ();
		(num_start .. )
			.take (batch_size)
			.map (|num| {
				buffer.truncate (salt.len ());
				write! (& mut buffer, "{}", num).unwrap ();
				stretched_hash (& buffer, hash_reps)
			})
			.collect::<Vec <_>> ()
	};
	ThreadMap::start (
			(0_i32 .. ).step_by (batch_size),
			map_fn,
			parallel::num_cpus_max (input.params.max_threads))
		.flatten ()
}

#[ inline ]
fn stretched_hash (input: & str, hash_reps: u32) -> [u8; 32] {
	let mut hasher = md5::MD5::new ();
	hasher.update (input.as_bytes ());
	let mut hash = hasher.finish_reset ();
	for _ in 0 .. hash_reps {
		hasher.update (& hash.as_hex_bytes ());
		hash = hasher.finish_reset ();
	}
	hash.as_hex_bytes ()
}
