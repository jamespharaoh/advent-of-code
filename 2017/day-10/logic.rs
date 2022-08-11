use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {

	let string =
		calc_sparse (
			input.lengths.iter ().copied (),
			input.params.string_size,
			input.params.rounds_one) ?;

	Ok (string [0].as_u32 () * string [1].as_u32 ())

}

pub fn part_two (input: & Input) -> GenResult <String> {

	// convert back to text representation

	let data: Vec <u8> =
		input.lengths.to_string ().bytes ()
			.chain ([17, 31, 73, 47, 23].into_iter ())
			.collect ();

	// calculate sparse hash with 64 rounds

	let sparse =
		calc_sparse (
			data.iter ().copied (),
			input.params.string_size,
			input.params.rounds_two) ?;

	// calculate dense hash

	let dense: Vec <u8> =
		sparse.into_iter ()
			.chunks (16)
			.into_iter ()
			.map (|chunk| chunk.fold (0, |sum, item| sum ^ item))
			.collect ();

	// convert to hex and return

	let result =
		dense.iter_vals ()
			.map (|byte| format! ("{:02x}", byte))
			.collect ();

	Ok (result)

}

fn calc_sparse (
	lengths_iter: impl Iterator <Item = u8> + Clone,
	string_size: u32,
	num_rounds: u32,
) -> GenResult <Vec <u8>> {

	if lengths_iter.clone ().any (|length| string_size < length.as_u32 ()) {
		return Err ("Length must be less or equal to string size".into ());
	}

	// initialise the mutable string data with ascending values

	let mut string: VecDeque <u8> =
		iter::repeat (u8::MIN ..= u8::MAX)
			.flatten ()
			.take (string_size.as_usize ())
			.collect ();

	// iterate over lengths, repeated `num_rounds` times

	let mut string_temp = Vec::new ();
	let mut skip_size = 0_usize;
	let mut moves = 0_usize;

	for length in
		iter::repeat (lengths_iter)
			.take (num_rounds.as_usize ())
			.flatten () {

		// take `length` items from the front, them add them to the back in reverse order

		for _ in 0 .. length { string_temp.push (string.pop_front ().unwrap ()); }
		while let Some (mark) = string_temp.pop () { string.push_back (mark); }
		moves += length.as_usize ();

		// advance by `skip_size`

		string.rotate_left (skip_size);
		moves += skip_size;

		// increase `skip_size` by 1

		skip_size += 1;
		if skip_size == string.len () { skip_size = 0; }

	}

	// return to start position and return

	string.rotate_right (moves % string.len ());
	Ok (string.into_iter ().collect ())

}
