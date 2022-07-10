use clap::ArgMatches;
use clap::Command;

pub use arrayvec::ArrayVec;
pub use clap;
pub use itertools::Itertools;
pub use std::borrow::Borrow;
pub use std::cell::RefCell;
pub use std::cmp;
pub use std::collections::BinaryHeap;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::collections::VecDeque;
pub use std::error::Error;
pub use std::ffi::OsString;
pub use std::fmt;
pub use std::fs;
pub use std::hash;
pub use std::iter;
pub use std::iter::Peekable;
pub use std::marker::PhantomData;
pub use std::mem;
pub use std::ops;
pub use std::ops::RangeInclusive;
pub use std::path::Path;
pub use std::rc::Rc;
pub use std::rc::Weak as RcWeak;
pub use std::slice::Iter as SliceIter;
pub use std::str::Chars;
pub use std::str::FromStr;

pub type GenError = Box <dyn Error>;
pub type GenResult <Ok> = Result <Ok, GenError>;

pub trait Puzzle {

	fn name (& self) -> & 'static str;
	fn year (& self) -> u16;
	fn day (& self) -> u8;
	fn part_one (& self, _lines: & [& str]) -> GenResult <String> { unimplemented! () }
	fn part_two (& self, _lines: & [& str]) -> GenResult <String> { unimplemented! () }
	fn num_parts (& self) -> usize { return 2 }

	fn commands (& self) -> Vec <PuzzleCommand> { Vec::new () }

	fn invoke (& self, args: & [OsString]) -> GenResult <()> {
		let mut command = Command::new (format! ("aoc-{}-day-{}", self.year (), self.day ()));
		for part_num in 1 ..= self.num_parts () {
			command = command.subcommand (Command::new (format! ("part-{}", part_num)));
		}
		for puzzle_command in self.commands () {
			command = command.subcommand (
				puzzle_command.command ()
					.name (puzzle_command.name ())
			);
		}
		let matches = command.get_matches_from (args);
		match matches.subcommand () {
			None => {
				let result = self.invoke_part_one () ?;
				println! ("Part one: {}", result);
				if self.num_parts () >= 2 {
					let result = self.invoke_part_two () ?;
					println! ("Part two: {}", result);
				}
				Ok (())
			},
			Some (("part-1", _)) => {
				let result = self.invoke_part_one () ?;
				println! ("Result: {}", result);
				Ok (())
			},
			Some (("part-2", _)) => {
				let result = self.invoke_part_two () ?;
				println! ("Result: {}", result);
				Ok (())
			},
			Some ((name, matches)) => {
				for puzzle_command in self.commands () {
					if puzzle_command.name () == name {
						return puzzle_command.invoke (& matches);
					}
				}
				unreachable! ();
			},
		}
	}

	fn invoke_part_one (& self) -> GenResult <String> {
		let input_string = fs::read_to_string (& format! ("inputs/day-{:02}", self.day ())) ?;
		let input_lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
		self.part_one (& input_lines)
	}

	fn invoke_part_two (& self) -> GenResult <String> {
		let input_string = fs::read_to_string (& format! ("inputs/day-{:02}", self.day ())) ?;
		let input_lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
		self.part_two (& input_lines)
	}

}

pub struct PuzzleCommand {
	name: & 'static str,
	invoke_fn: Box <dyn Fn (& ArgMatches) -> GenResult <()>>,
	magic: Box <dyn MagicTrait>,
}

trait MagicTrait {
	fn command <'help> (& self) -> Command <'help>;
}

struct MagicStruct <Args> {
	phantom: PhantomData <Args>,
}

impl <Args: clap::CommandFactory> MagicTrait for MagicStruct <Args> {
	fn command <'help> (& self) -> Command <'help> { Args::command () }
}

impl PuzzleCommand {

	pub fn new <
		Args: clap::Parser + 'static,
		InvokeFn: Fn (Args) -> GenResult <()> + 'static,
	> (
		name: & 'static str,
		invoke_fn: InvokeFn,
	) -> PuzzleCommand {

		let invoke_fn = Box::new (
			move |matches: & _| invoke_fn (Args::from_arg_matches (matches).unwrap ()),
		);

		let magic: Box <MagicStruct <Args>> = Box::new (MagicStruct {
			phantom: PhantomData,
		});

		PuzzleCommand { name, invoke_fn, magic }

	}

	pub fn name (& self) -> & str { self.name }
	pub fn command <'help> (& self) -> Command <'help> { self.magic.command () }

	pub fn invoke (& self, args: & ArgMatches) -> GenResult <()> {
		(self.invoke_fn) (args)
	}

}

#[ macro_export ]
macro_rules! puzzle {

	( @commands $commands:ident ) => {};
	( @commands $commands:ident (
		name = $name:literal ;
		method = $method:expr ;
	) ) => {
		$commands.push (PuzzleCommand::new ($name, $method));
	};
	( @commands $commands:ident (
		name = $name:literal ;
		method = $method:expr ;
	) , $($rest:tt)* ) => {
		$commands.push (PuzzleCommand::new ($name, $method));
		puzzle! { @commands $commands $($rest)* }
	};

	( @rest ) => {};
	( @rest commands = [ $($commands:tt)* ] ; $($rest:tt)* ) => {
		fn commands (& self) -> Vec <PuzzleCommand> {
			let mut commands = Vec::new ();
			puzzle! { @commands commands $($commands)* }
			commands
		}
		puzzle! { @rest $($rest)* }
	};

	(
		name = $name:literal ;
		year = $year:literal ;
		day = $day:literal ;
		part_one = |$part_one_lines:ident| $part_one:expr ;
		part_two = |$part_two_lines:ident| $part_two:expr ;
		$($rest:tt)*
	) => {
		pub fn puzzle_metadata () -> Box <dyn Puzzle> {
			struct ThisPuzzle;
			impl Puzzle for ThisPuzzle {
				fn name (& self) -> & 'static str { $name }
				fn year (& self) -> u16 { $year }
				fn day (& self) -> u8 { $day }
				fn part_one (& self, $part_one_lines: & [& str]) -> GenResult <String> {
					$part_one.map (|val| format! ("{}", val))
				}
				fn part_two (& self, $part_two_lines: & [& str]) -> GenResult <String> {
					$part_two.map (|val| format! ("{}", val))
				}
				puzzle! { @rest $($rest)* }
			}
			Box::new (ThisPuzzle {})
		}
	};
	(
		name = $name:literal ;
		year = $year:literal ;
		day = $day:literal ;
		part_one = |$part_one_lines:ident| $part_one:expr ;
		part_two = |$part_two_lines:ident| $part_two:expr ;
		$($rest:tt)*
	) => {
		pub const fn puzzle_metadata () -> Box <dyn Puzzle> {
			struct ThisPuzzle;
			impl Puzzle for ThisPuzzle {
				fn name (& self) -> & 'static str { $name }
				fn year (& self) -> u16 { $year }
				fn day (& self) -> u8 { $day }
				fn part_one (& self, $part_one_lines: & [& str]) -> GenResult <String> {
					$part_one.map (|val| format! ("{}", val))
				}
				fn part_two (& self, $part_two_lines: & [& str]) -> GenResult <String> {
					$part_two.map (|val| format! ("{}", val))
				}
				puzzle! { @rest $($rest)* }
			}
			Box::new (ThisPuzzle {})
		}
	};
	(
		name = $name:literal ;
		year = $year:literal ;
		day = $day:literal ;
		part_one = |$part_one_lines:ident| $part_one:expr ;
		$($rest:tt)*
	) => {
		pub fn puzzle_metadata () -> Box <dyn Puzzle> {
			struct ThisPuzzle;
			impl Puzzle for ThisPuzzle {
				fn name (& self) -> & 'static str { $name }
				fn year (& self) -> u16 { $year }
				fn day (& self) -> u8 { $day }
				fn num_parts (& self) -> usize { 1 }
				fn part_one (& self, $part_one_lines: & [& str]) -> GenResult <String> {
					$part_one.map (|val| format! ("{}", val))
				}
				puzzle! { @rest $($rest)* }
			}
			Box::new (ThisPuzzle {})
		}
	};

}

#[ must_use ]
#[ inline ]
pub fn default <T: Default> () -> T {
	Default::default ()
}

pub struct Parser <'a, ErrFn> {
	input: & 'a str,
	pos: usize,
	err_fn: ErrFn,
}

impl <'a, ErrFn, ErrFnRet> Parser <'a, ErrFn>
	where ErrFn: Fn (usize) -> ErrFnRet, ErrFnRet: Into <GenError> {

	pub fn new (input: & 'a str, err_fn: ErrFn) -> Parser <'a, ErrFn> {
		Parser {
			input,
			pos: 0,
			err_fn,
		}
	}

	pub fn expect (& mut self, expect: & str) -> GenResult <& mut Self> {
		for expect_char in expect.chars () {
			if self.peek () != Some (expect_char) { Err (self.err ()) ? }
			self.next ();
		}
		Ok (self)
	}

	pub fn int <IntType> (& mut self) -> GenResult <IntType> where IntType: FromStr {
		let len = self.input.chars ().enumerate ()
			.take_while (|& (idx, letter)| letter.is_digit (10) || (idx == 0 && letter == '-'))
			.map (|(_, letter)| letter.len_utf8 ())
			.sum ();
		let val = self.input [0 .. len].parse ().map_err (|_| self.err ()) ?;
		self.input = & self.input [len .. ];
		Ok (val)
	}

	pub fn end (& mut self) -> GenResult <()> {
		if self.peek ().is_some () { Err (self.err ()) ? }
		Ok (())
	}

	pub fn peek (& mut self) -> Option <char> {
		self.input.chars ().next ()
	}

	pub fn next (& mut self) -> Option <char> {
		let letter_opt = self.input.chars ().next ();
		if let Some (letter) = letter_opt {
			self.input = & self.input [letter.len_utf8 () .. ];
			self.pos += 1;
		}
		letter_opt
	}

	pub fn err (& self) -> GenError {
		(self.err_fn) (self.pos).into ()
	}

}
