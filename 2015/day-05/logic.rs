use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.strings.iter ()
			.filter (|line| is_nice_one (line))
			.count ()
			.to_u32 () ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.strings.iter ()
			.filter (|line| is_nice_two (line))
			.count ()
			.to_u32 () ?
	)
}

fn is_nice_one (input: & str) -> bool {
	if input.chars ()
			.filter (|ch| ['a', 'e', 'i', 'o', 'u'].contains (ch))
			.count () < 3 {
		return false;
	}
	if ! input.chars ()
			.array_windows ()
			.any (|[ch_0, ch_1]| ch_0 == ch_1) {
		return false;
	}
	if input.chars ()
			.array_windows ()
			.any (|chars| [ ['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y'] ]
				.contains (& chars)) {
		return false;
	}
	true
}

fn is_nice_two (input: & str) -> bool {
	if ! input.chars ()
			.array_windows::<2> ()
			.enumerate ()
			.any (|(idx, chars_0)|
				input.chars ().skip (idx + 2)
					.array_windows ()
					.any (|chars_1| chars_0 == chars_1)) {
		return false;
	}
	if ! input.chars ()
			.array_windows ()
			.any (|[ch_0, _, ch_1]| ch_0 == ch_1) {
		return false;
	}
	true
}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn is_nice_one () {
		assert_eq! (true, logic::is_nice_one ("ugknbfddgicrmopn"));
		assert_eq! (true, logic::is_nice_one ("aaa"));
		assert_eq! (false, logic::is_nice_one ("jchzalrnumimnmhp"));
		assert_eq! (false, logic::is_nice_one ("haegwjzuvuyypxyu"));
		assert_eq! (false, logic::is_nice_one ("dvszwmarrgswjxmb"));
	}

	#[ test ]
	fn is_nice_two () {
		assert_eq! (true, logic::is_nice_two ("qjhvhtzxzqqjkmpb"));
		assert_eq! (true, logic::is_nice_two ("xxyxx"));
		assert_eq! (false, logic::is_nice_two ("uurcxstgmygtbstg"));
		assert_eq! (false, logic::is_nice_two ("ieodomkazucvgmuy"));
	}

}
