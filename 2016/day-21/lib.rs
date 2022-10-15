//! Advent of Code 2016: Day 21: Scrambled Letters and Hash
//!
//! [https://adventofcode.com/2016/day/21](https://adventofcode.com/2016/day/21)
//!
//! # Input
//!
//! A series of operations, one per line, to perform on a password consisting of a series of
//! lowercase letters. The operations are as follows:
//!
//! * `swap position P0 with position P1`, where `P0` and `P1` are valid zero-based positions for
//!   the password length, swaps the letters at the named positions.
//! * `swap letter C0 with letter C1`, where `C0` and `C1` are lowercase letters, swaps the two
//!   mentioned charaters.
//! * `rotate left N steps` or `rotate right N steps`, where `N` is an integer between zero and
//!   the password length, rotates the characters as you would expect.
//! * `rotate based on position of letter C`, where `C` is a lowercase letter, rotates the string
//!   to the right a number of times equal to the zero-based index of the letter plus one.
//! * `reverse positions P0 through P1`, where `P0` and `P1` are valid zero-based positions for
//!   the password length, reverses the characters between those positions, inclusively.
//! * `move position P0 to position P1`, where `P0` and `P1` are valid zero-based positions for
//!   the password length, removes the character at the first position and reinserts it at the
//!   second.
//!
//! # Part one
//!
//! Apply the operations to the password `abcdefgh` and return the result.
//!
//! # Part two
//!
//! Determine the password to which you can apply the operations to get the result `fbgdceah`.
//!
//! # Algorithm
//!
//! We basically just apply the indicated operations as described. Reversing the operations for
//! part two is fairly simple. The operation to rotate based on the position of a character
//! requires a little analysis to do in reverse, but this turns out to be fairly simply as well.
//!
//! We do all of this very efficiently, without allocation, and this runs extremely quickly. Most
//! of the forward and reverse operations share code, for example all four of the rotation
//! operations share a single implementation with some logic to reverse the amount as appropriate.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

mod examples;
pub mod input;
pub mod logic;
pub mod ops;

puzzle_info! {
	name = "Scrambled Letters and Hash";
	year = 2016;
	day = 21;
	parse = |lines| input::Input::parse_from_lines (lines);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod model {

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Mode { Scramble, Unscramble }

	pub type State = Vec <char>;

}
