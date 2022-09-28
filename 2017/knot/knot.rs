use aoc_common::*;

#[ inline ]
#[ must_use ]
pub fn calculate (input: & [u8]) -> [u8; 16] {
	calculate_rounds (input, 64)
}

#[ inline ]
#[ must_use ]
pub fn calculate_rounds (input: & [u8], num_rounds: u32) -> [u8; 16] {
	let input_iter = input.iter ().copied ().chain ([ 17, 31, 73, 47, 23 ]);
	let sparse = calc_sparse (input_iter, num_rounds);
	calc_checksum (& sparse)
}

#[ inline ]
#[ must_use ]
pub fn calc_sparse (
	input_iter: impl Iterator <Item = u8> + Clone,
	num_rounds: u32,
) -> [u8; 256] {

	// initialise the mutable string data with ascending values

	let mut string: VecDeque <u8> = (u8::MIN ..= u8::MAX).collect ();

	// iterate over lengths, repeated `num_rounds` times

	let mut string_temp = Vec::new ();
	let mut skip_size = 0_usize;
	let mut moves = 0_usize;

	for length in
		iter::repeat (input_iter)
			.take (num_rounds.qck_usize ())
			.flatten () {

		// take `length` items from the front, them add them to the back in reverse order

		for _ in 0 .. length { string_temp.push (string.pop_front ().unwrap ()); }
		while let Some (mark) = string_temp.pop () { string.push_back (mark); }
		moves += length.qck_usize ();

		// advance by `skip_size`

		string.rotate_left (skip_size);
		moves += skip_size;

		// increase `skip_size` by 1

		skip_size += 1;
		if skip_size == string.len () { skip_size = 0; }

	}

	// return to start position and convert to array

	string.rotate_right (moves % string.len ());

	string.into_iter ().collect::<Vec <u8>> ().try_into ().unwrap ()

}

#[ inline ]
#[ must_use ]
pub fn calc_checksum (input: & [u8; 256]) -> [u8; 16] {
	array::from_fn (|out_idx| {
		let in_idx = out_idx << 4_u32;
		input [in_idx .. in_idx + 16].iter ().copied ().fold (0, |sum, item| sum ^ item)
	})
}
