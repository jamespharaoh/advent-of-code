use super::*;

use input::Input;
use input::Token;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut stream_iter = input.stream.iter ().copied ();
	let mut count = 0_u32;
	while let Some (token) = stream_iter.next () {
		match token {
			Token::Letter (_) => count += 1,
			Token::Repeat (len, num) => {
				count += len.pan_u32 () * num.pan_u32 ();
				take_len (& mut stream_iter, len.pan_u32 ()) ?;
			},
		}
	}
	Ok (count)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut count = 0_u64;
	let mut pos = 0;
	let mut stack = Vec::new ();
	let mut mul = 1_u64;
	let mut next = u32::MAX;
	for & token in & input.stream {
		if matches! (token, Token::Letter (_)) { chk! (count += mul) ?; }
		pos += token.display_len ();
		while next <= pos {
			if next < pos { return Err ("Invalid token stream".into ()) }
			(mul, next) = stack.pop ().unwrap ();
		}
		if let Token::Repeat (len, num) = token {
			stack.push ((mul, next));
			chk! (mul *= num.pan_u64 ()) ?;
			next = pos + len.pan_u32 ();
		}
	}
	Ok (count)
}

fn take_len (mut stream_iter: impl Iterator <Item = Token>, mut len: u32) -> GenResult <()> {
	while 0 < len {
		if let Some (token) = stream_iter.next () {
			let token_len = token.display_len ();
			if len < token_len { return Err ("Invalid token stream".into ()) }
			len -= token_len;
		} else {
			return Err ("Invalid token stream".into ());
		}
	}
	Ok (())
}
