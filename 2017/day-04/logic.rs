use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.passphrases.iter ()
			.filter (|passphrase| ! passphrase.iter ()
				.tuple_combinations::<(_, _)> ()
				.any (|(left, right)| left == right))
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut buffer_pool: Vec <Vec <char>> = Vec::new ();
	let mut words: Vec <Vec <char>> = Vec::new ();
	Ok (
		input.passphrases.iter ()
			.filter (|passphrase| {
				words.extend (
					passphrase.iter ()
						.map (|word| {
							let mut buffer = buffer_pool.pop ().unwrap_or_default ();
							buffer.clear ();
							buffer.extend (word.chars ());
							buffer.sort ();
							buffer
						}));
				let result =
					! words.iter ()
						.tuple_combinations::<(_, _)> ()
						.any (|(left, right)| left == right);
				buffer_pool.append (& mut words);
				result
			})
			.count ()
			.pan_u32 ()
	)
}
