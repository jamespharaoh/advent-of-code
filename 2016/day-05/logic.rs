use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <String> {
	let mut password = String::new ();
	for hash in iter_hashes (input) {
		if password.len () == input.params.password_len.pan_usize () { break }
		password.push (char::from_digit ((hash [2] & 0xf).pan_u32 (), 16).unwrap ());
	}
	Ok (password)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let mut password: TinyVec <Option <char>, 8> =
		iter::repeat (None)
			.take (input.params.password_len.pan_usize ())
			.collect ();
	for hash in iter_hashes (input) {
		if ! password.iter ().any (Option::is_none) { break }
		let pos = hash [2] & 0xf;
		if input.params.password_len <= pos { continue }
		if password [pos.pan_usize ()].is_some () { continue }
		let ch = char::from_digit ((hash [3] >> 4_u32).pan_u32 (), 16).unwrap ();
		password [pos.pan_usize ()] = Some (ch);
	}
	Ok (
		password.iter ()
			.map (|& ch| ch.unwrap ())
			.collect ()
	)
}

fn iter_hashes (input: & Input) -> impl Iterator <Item = md5::Output> {
	let door_id: Arc <str> = Arc::from (input.door_id.as_str ());
	let num_zeros = input.params.num_zeros;
	let batch_size = input.params.batch_size;
	let map_fn = move |num_start| {
		let mut hashes = Vec::new ();
		let mut buffer = door_id.deref ().to_owned ();
		for num in (num_start .. ).take (batch_size) {
			buffer.truncate (door_id.len ());
			write! (buffer, "{}", num).unwrap ();
			let hash = md5_hash (buffer.as_bytes ());
			if hash.num_zeros () < num_zeros { continue }
			hashes.push (hash);
		}
		hashes
	};
	ThreadMap::start (
			(0_u32 .. ).step_by (input.params.batch_size),
			map_fn,
			parallel::num_cpus_max (input.params.max_threads).pan_usize ())
		.flatten ()
}
