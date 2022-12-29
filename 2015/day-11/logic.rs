use super::*;

use input::Input;

const ALL_CHARS: & [char] = & [
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v',
	'w', 'x', 'y', 'z',
];

pub fn part_one (input: & Input) -> GenResult <String> {
	let password = next_password (& input.initial) ?;
	Ok (password)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let password_0 = next_password (& input.initial) ?;
	let password_1 = next_password (& password_0) ?;
	Ok (password_1)
}

pub fn next_password (input: & str) -> GenResult <String> {
	if input.chars ().count () != 8 {
		return Err ("Password must have eight characters".into ());
	}
	let mut char_idxs: Vec <usize> =
		input.chars ()
			.map (|ch| ALL_CHARS.iter ().copied ()
				.position (|valid_ch| ch == valid_ch)
				.ok_or_else (|| format! ("Input contains invalid character: {ch}").into ()))
			.collect::<GenResult <_>> () ?;
	fn idxs_to_chars (char_idxs: & [usize]) -> impl Iterator <Item = char> + '_ + Clone {
		char_idxs.iter ().copied ().map (|idx| ALL_CHARS [idx])
	}
	let new_password = loop {
		for idx in (0 .. char_idxs.len ()).rev () {
			char_idxs [idx] += 1;
			if char_idxs [idx] < ALL_CHARS.len () && (idx < char_idxs.len () - 2
					|| find_pair (idxs_to_chars (& char_idxs [ .. char_idxs.len () - 2]))
						.is_some ())
				{ break }
			char_idxs [idx] = 0;
		}
		if is_valid (idxs_to_chars (& char_idxs)) {
			break char_idxs.iter ().copied ()
				.map (|idx| ALL_CHARS [idx])
				.collect::<String> ();
		}
	};
	Ok (new_password)
}

pub fn is_valid <Inpt> (input: Inpt) -> bool
		where Inpt: Iterator <Item = char> + Clone {
	if ! input.clone ()
		.array_windows ()
		.any (|[a, b, c]| a.pan_u32 () + 1 == b.pan_u32 () && b.pan_u32 () + 1 == c.pan_u32 ()) {
		return false;
	}
	let (num_repeats, _, _) =
		input.fold ((0_u32, None, None), |(sum, last, prev), next|
			if last == Some (next) && prev != Some (next) {
				(sum + 1, None, Some (next))
			} else {
				(sum, Some (next), prev)
			});
	if num_repeats < 2 { return false }
	true
}

pub fn find_pair <Inpt> (input: Inpt) -> Option <(usize, char)>
		where Inpt: Iterator <Item = char> + Clone {
	input
		.array_windows ()
		.filter (|& [a, b]| a == b)
		.map (|[a, _]| a)
		.enumerate ()
		.next ()
}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn is_valid () {
		assert! (! logic::is_valid ("hijklmmn".chars ()));
		assert! (! logic::is_valid ("abbceffg".chars ()));
		assert! (! logic::is_valid ("abbcegjk".chars ()));
	}

	#[ test ]
	fn next_password () {
		assert_eq_ok! ("abcdffaa", logic::next_password ("abcdefgh"));
		assert_eq_ok! ("ghjaabcc", logic::next_password ("ghhzzzzz"));
	}

}
