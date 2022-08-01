//! Advent of Code 2015: Day 10: Elves Look, Elves Say
//!
//! [https://adventofcode.com/2015/day/10](https://adventofcode.com/2015/day/10)

#![ deny (bindings_with_variant_name) ]
#![ deny (non_camel_case_types) ]
#![ deny (non_snake_case) ]
#![ deny (non_upper_case_globals) ]

use aoc_common::*;

puzzle_info! {
	name = "Elves Look, Elves Say";
	year = 2015;
	day = 10;
	part_one = |input| logic::part_one (input [0]);
	part_two = |input| logic::part_two (input [0]);
	commands = [
		( name = "internals"; method = cli::internals; ),
		( name = "run"; method = cli::run; ),
		( name = "tracking"; method = tracking::run; ),
		( name = "cycles"; method = cycles::run; ),
	];
}

mod cycles;
mod tracking;

mod logic {

	use super::*;
	use model::State;
	use nums::IntConv;

	pub fn part_one (input: & str) -> GenResult <u32> {
		let input = State::parse (input) ?;
		Ok (
			iter::successors (
					Some (input),
					|state| Some (one_round (state)))
				.nth (40)
				.unwrap ()
				.len ().as_u32 ()
		)
	}

	pub fn part_two (input: & str) -> GenResult <u32> {
		let input = State::parse (input) ?;
		Ok (
			iter::successors (
					Some (input),
					|state| Some (one_round (state)))
				.nth (50)
				.unwrap ()
				.len ().as_u32 ()
		)
	}

	pub fn one_round (state: & State) -> State {
		let group_by =
			state.iter ().copied ()
				.group_by (|& val| val);
		group_by.into_iter ()
			.flat_map (|(val, group)| [ group.count ().as_u8 (), val ])
			.collect ()
	}

}

mod model {

	use super::*;
	use nums::IntConv;

	#[ derive (Clone, Eq, Hash, PartialEq) ]
	pub struct State (Vec <u8>);

	impl State {
		pub fn parse (input: & str) -> GenResult <Self> {
			input.chars ().map (|ch|
				Ok (ch.to_digit (10).ok_or ("Invalid input") ?.as_u8 ())
			).collect::<GenResult <_>> ()
		}
		pub fn iter (& self) -> SliceIter <'_, u8> {
			self.0.iter ()
		}
	}

	impl Borrow <[u8]> for State {
		fn borrow (& self) -> & [u8] {
			self.0.as_slice ()
		}
	}

	impl Deref for State {
		type Target = Vec <u8>;
		fn deref (& self) -> & Vec <u8> {
			& self.0
		}
	}

	impl FromIterator <u8> for State {
		fn from_iter <IntoIter> (iter: IntoIter) -> Self
				where IntoIter: IntoIterator <Item = u8> {
			Self (iter.into_iter ().collect ())
		}
	}

	impl TryFrom <Vec <u8>> for State {
		type Error = GenError;
		fn try_from (nums: Vec <u8>) -> GenResult <Self> {
			if nums.iter ().copied ().any (|num| (1 ..= 9).contains (& num)) {
				Err ("Digits must be 1-9") ?;
			}
			Ok (Self (nums))
		}
	}

	impl Debug for State {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "State (len={}, {})", self.0.len (), self) ?;
			Ok (())
		}
	}

	impl Display for State {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			for & val in self.0.iter () {
				write! (formatter, "{}", char::from_digit (val.as_u32 (), 10).unwrap ()) ?;
			}
			Ok (())
		}
	}

	impl PartialOrd for State {
		fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
			Some (Ord::cmp (self, other))
		}
	}

	impl Ord for State {
		fn cmp (& self, other: & Self) -> Ordering {
			Ord::cmp (& self.0.len (), & other.0.len ())
				.then (Ord::cmp (& self.0, & other.0))
		}
	}

}

mod cli {

	use super::*;
	use model::State;

	#[ derive (Debug, clap::Parser) ]
	pub struct RunArgs {

		#[ clap (long, default_value = "inputs/day-10") ]
		input: String,

		#[ clap (conflicts_with = "input") ]
		state: Option <String>,

		#[ clap (long) ]
		verbose: bool,

		#[ clap (long, default_value = "15") ]
		loops: u32,

		#[ clap (long, default_value = "0") ]
		keep_end: usize,

		#[ clap (long, default_value = "0") ]
		keep_start: usize,

	}

	#[ allow (clippy::needless_pass_by_value) ]
	#[ allow (clippy::print_stdout) ]
	pub fn run (args: RunArgs) -> GenResult <()> {
		let mut state = if let Some (state) = args.state.as_ref () {
			State::parse (state) ?
		} else {
			State::parse (
				fs::read_to_string (& args.input) ?
					.trim ()
					.split ('\n')
					.next ().unwrap ()
			) ?
		};
		for idx in 0 .. {
			println! ("{:2} {:4} {}", idx, state.len (), state);
			if idx == args.loops { break }
			state = logic::one_round (& state);
			if (args.keep_start > 0 || args.keep_end > 0)
					&& state.len () > (args.keep_start + args.keep_end) {
				state =
					state [ .. args.keep_start].iter ().copied ()
						.chain (state [state.len () - args.keep_end .. ].iter ().copied ())
						.collect ();
			}
		}
		Ok (())
	}

	#[ derive (Debug, clap::Parser) ]
	pub struct InternalsArgs;

	#[ allow (clippy::needless_pass_by_value) ]
	#[ allow (clippy::print_stdout) ]
	#[ allow (clippy::unnecessary_wraps) ]
	pub fn internals (_args: InternalsArgs) -> GenResult <()> {
		println! ("Data structures:");
		fn show_struct <Type> () {
			let name = std::any::type_name::<Type> ();
			let size = mem::size_of::<Type> ();
			let align = mem::align_of::<Type> ();
			println! (" - {} {} bytes (align = {})", name, size, align);
		}
		show_struct::<tracking::Item> ();
		Ok (())
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;
	use model::State;

	#[ test ]
	fn basic () -> GenResult <()> {
		let mut state = State::parse ("1") ?;
		for expect in [ "11", "21", "1211", "111221", "312211" ] {
			state = logic::one_round (& state);
			let expect = State::parse (expect) ?;
			assert_eq! (expect, state);
		}
		Ok (())
	}

}
