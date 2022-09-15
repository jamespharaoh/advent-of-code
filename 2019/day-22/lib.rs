//! Advent of Code 2019: Day 22: Slam Shuffle
//!
//! [https://adventofcode.com/2019/day/22](https://adventofcode.com/2019/day/22)
//!
//! # Input
//!
//! A series of so-called shuffle operations, one per line, with three variants:
//!
//! - `deal into new stack`
//! - `cut n` where `n` is an integer
//! - `deal with increment n` where `n` is a positive integer
//!
//! See the puzzle description for details on what these mean.
//!
//! # Part one
//!
//! Apply the described operations once with a deck of size `10007`. Return the final position of
//! card `2019`.
//!
//! # Part two
//!
//! Apply the desired operations `101741582076661` times with a deck of size `119315717514047`.
//! Return the value of the card in position `2020`.
//!
//! # Algorithm
//!
//! The change in a card's position can be modelled as a combination of addition and multiplication
//! modulo the deck size. This relies on the following two things being true:
//!
//! - The deck size is a prime number
//! - The cards all start out with a position equal to their value
//!
//! The operations map as follows:
//!
//! - "Deal into new stack" is a negation followed by subtracting one, which can be converted to
//!   multiplication by the deck size minus one, followed by addition of the deck size minus one
//! - "Cut" is subtraction, which can be converted to addition by subtracting the argument from the
//!   deck size
//! - "Deal with increment" is simple multiplication
//!
//! We can combine two steps in sequence using some simple maths. We model an operation as a
//! multiplication followed by an addition:
//!
//! `f (pos, mul, add) = pos × mul + add`
//!
//! We can apply this function to itself and then extract a single copy of the original function as
//! follows. Here we use `mul_a` and `add_a` as the first step's arguments, and `mul_b` and `add_b`
//! as the second step's:
//!
//! ```text
//! f (f (pos, mul_a, add_a), mul_b, add_b)
//!     = (pos × mul_a + add_a) × mul_b + add_b
//!     = pos × mul_a × mul_b + add_a × mul_b + add_b
//!     = f (pos, mul_a × mul_b, add_a × mul_b + add_b)
//! ```
//!
//! Using this we can collapse the entire sequence of steps to a single step. This doesn't help
//! much for part one, but it is important for part two. We also need to be able to reverse the
//! operations. We can derive this from some basic maths identities:
//!
//! ```text
//! pos_1 = f (pos_0, mul, add)
//!       = pos_0 × mul + add
//! pos_0 × mul = pos_1 - add
//! pos_0 = (pos_1 - add) ÷ mul
//!       = pos_1 ÷ mul - add ÷ mul
//!       = pos_1 × (1 ÷ mul) + (- add ÷ mul)
//!       = f (pos_1, 1 ÷ mul, - add ÷ mul)
//! ```
//!
//! I am cheating a little here with the above representation, although the maths is sound. Instead
//! of dividing, we are actually multiplying by the
//! [multiplicative inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse), which
//! is possible because the modulo is a prime number.
//!
//! Now we can represent all the steps as `(mul, add)`, combine steps into the same form, and
//! find the reverse. We also need to be able to repeat steps many times. To do this, we repeatedly
//! combine a step with itself to get equivalent steps for repetition counts of increasing powers
//! of two. We apply these to the input zero or one times to get the right number. This is fairly
//! simple to do because these are the binary digits of the repetition count.
//!
//! With all these operations implemented the algorithm becomes very simple:
//!
//! - Convert the shuffles into `(mul, add)` operations
//! - Combine them into a single operation
//! - For part two, reverse the operation
//! - For part two, repeat the operation the specified number of times
//! - Apply the operation to the input
//!
//! Although the numbers involved fit into 64-bits, they are very high, and multiplication will
//! overflow easily. For this reason we use [`u64`] to store everything, but convert to [`u128`]
//! to perform operations. Once we modulo the result we can convert back to [`u64`] safely.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub mod input;
pub mod logic;
pub mod model;

puzzle_info! {
	name = "Slam Shuffle";
	year = 2019;
	day = 22;
	parse = |input| input::Input::parse_from_lines (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}
