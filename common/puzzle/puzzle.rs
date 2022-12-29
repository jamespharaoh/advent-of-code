use super::*;

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use command::PuzzleCommand;

args_decl! {
	#[ derive (Debug) ]
	pub struct Args {
		pub input: Option <PathBuf>,
		pub repeat: Option <u64>,
	}
}

#[ allow (clippy::print_stdout) ]
pub (crate) fn puzzle_invoke_real (
	puzzle: & dyn Puzzle,
	args: & [OsString],
) -> GenResult <()> {

	let command = args.get (1)
		.and_then (|os_str| os_str.to_str ())
		.into_iter ()
		.find (|rust_str| ! rust_str.starts_with ('-'));
	let command_args = args.iter ().skip (if command.is_some () { 2 } else { 1 }).cloned ();

	use aoc_args::ArgsParse as _;

	let part = match command {
		None => None,
		Some ("part-1") => Some (1),
		Some ("part-2") => Some (2),
		Some (command) => {
			for puzzle_command in puzzle.commands () {
				if puzzle_command.name () == command {
					return puzzle_command.invoke (command_args);
				}
			}
			return Err (format! ("Command not recognised: {command}").into ());
		},
	};

	let args = Args::parse (command_args) ?;
	let input_path = args.input.unwrap_or_else (|| puzzle.find_input_or_default ());
	let input_string = fs::read_to_string (input_path) ?;
	let input_lines: Vec <& str> = input_string.trim_end ().split ('\n').collect ();
	let repeat = args.repeat.unwrap_or (1);

	if part.map_or (false, |part| puzzle.num_parts () < part) {
		return Err (format! ("No such part: {}", part.unwrap ()).into ());
	}

	if part.unwrap_or (1) == 1 && 1 <= puzzle.num_parts () {
		run::runner (repeat, |idx| {
			let result = puzzle.part_one (& input_lines) ?;
			if idx == 0 { println! ("Part one: {result}"); }
			Ok (())
		}) ?;
	}

	if part.unwrap_or (2) == 2 && 2 <= puzzle.num_parts () {
		run::runner (repeat, |idx| {
			let result = puzzle.part_two (& input_lines) ?;
			if idx == 0 { println! ("Part two: {result}"); }
			Ok (())
		}) ?;
	}

	Ok (())

}

#[ allow (clippy::missing_inline_in_public_items) ]
pub trait Puzzle {

	fn dyn_puzzle (& self) -> & dyn Puzzle;

	fn name (& self) -> & 'static str;
	fn year (& self) -> u16;
	fn day (& self) -> u8;

	#[ inline ]
	fn part_one (& self, _lines: & [& str]) -> GenResult <String> { unimplemented! () }

	#[ inline ]
	fn part_two (& self, _lines: & [& str]) -> GenResult <String> { unimplemented! () }

	fn num_parts (& self) -> usize;

	#[ inline ]
	fn commands (& self) -> Vec <PuzzleCommand> {
		Vec::new ()
	}

	#[ inline ]
	fn set_default_params (& mut self) { }

	fn set_param_real (& mut self, name: String, value: String);

	#[ inline ]
	fn set_param (& mut self, name: & str, value: String) {
		self.set_param_real (name.to_owned (), value);
	}

	#[ inline ]
	fn invoke (& self, args: & [OsString]) -> GenResult <()> {
		puzzle_invoke_real (self.dyn_puzzle (), args)
	}

	fn find_input_or_arg (& self, arg: & Option <PathBuf>) -> PathBuf {
		arg.as_ref ().map_or_else (|| self.find_input_or_default (), PathBuf::clone)
	}

	fn find_input_or_default (& self) -> PathBuf {
		self.find_input ()
			.unwrap_or_else (|_| format! (
				"{:04}/inputs/day-{:02}", self.year (), self.day ()).into ())
	}

	fn find_input (& self) -> GenResult <PathBuf> {
		Ok (
			[
				format! ("{:04}/inputs/day-{:02}", self.year (), self.day ()),
				format! ("inputs/day-{:02}", self.day ()),
				format! ("../inputs/day-{:02}", self.day ()),
			].into_iter ()
				.find (|path| Path::new (path).exists ())
				.ok_or_else (|| format! (
					"Unable to find inputs/day-{:02} in \"{:04}\", \".\" or \"..\"",
					self.day (),
					self.year ())) ?
				.into ()
		)
	}

	fn load_input (& self) -> GenResult <String> {
		let input_path = [
			format! ("{:04}/inputs/day-{:02}", self.year (), self.day ()),
			format! ("inputs/day-{:02}", self.day ()),
			format! ("../inputs/day-{:02}", self.day ()),
		].into_iter ()
			.find (|path| Path::new (path).exists ())
			.ok_or_else (|| format! (
				"Unable to find inputs/day-{:02} in \"{:04}\", \".\" or \"..\"",
				self.day (),
				self.year ())) ?;
		let input_string = fs::read_to_string (input_path) ?;
		Ok (input_string)
	}

}

#[ macro_export ]
macro_rules! puzzle_info {

	(
		name = $name:literal ;
		year = $year:literal ;
		day = $day:literal ;
		$($rest4:tt)*
	) => {
		pub fn puzzle_metadata () -> Box <dyn $crate::puzzle::Puzzle> {
			use $crate::puzzle::Puzzle;
			struct ThisPuzzle { params: HashMap <String, String> }
			impl Puzzle for ThisPuzzle {
				fn dyn_puzzle (& self) -> & dyn Puzzle { self }
				fn name (& self) -> & 'static str { $name }
				fn year (& self) -> u16 { $year }
				fn day (& self) -> u8 { $day }
				fn set_param_real (& mut self, name: String, value: String) {
					self.params.insert (name, value);
				}
				puzzle_info! { @rest (input, Ok::<_, Infallible> (input), 0) $($rest4)* }
			}
			let mut puzzle = ThisPuzzle { params: HashMap::new () };
			puzzle.set_default_params ();
			Box::new (puzzle)
		}
	};

	( @rest ($parse_input:ident, $parse_expr:expr, $num_parts:expr) ) => {
		fn num_parts (& self) -> usize { $num_parts }
	};
	( @rest ($parse_input_old:ident, $parse_expr_old:expr, $num_parts:expr)
		parse = |$parse_input:ident| $parse_expr:expr;
		$($rest1:tt)*
	) => {
		puzzle_info! { @rest ($parse_input, $parse_expr, $num_parts) $($rest1)* }
	};
	( @rest ($parse_input:ident, $parse_expr:expr, $num_parts:expr)
		part_one = |$part_input:ident $(, $param_name:ident : $param_type:ty)*| $part_expr:expr;
		$($rest0:tt)*
	) => {
		fn part_one (& self, $parse_input: & [& str]) -> GenResult <String> {
			$(
				let $param_name: $param_type =
					self.params [stringify! ($param_name)].parse ().unwrap ();
			)*
			let $part_input = $parse_expr ?;
			let result = $part_expr ?;
			Ok (format! ("{}", result))
		}
		puzzle_info! { @rest ($parse_input, $parse_expr, $num_parts + 1) $($rest0)* }
	};
	( @rest ($parse_input:ident, $parse_expr:expr, $num_parts:expr)
		part_two = |$part_input:ident $(, $param_name:ident : $param_type:ty)*| $part_expr:expr;
		$($rest0:tt)*
	) => {
		fn part_two (& self, $parse_input: & [& str]) -> GenResult <String> {
			$(
				let $param_name: $param_type =
					self.params [stringify! ($param_name)].parse ().unwrap ();
			)*
			let $part_input = $parse_expr ?;
			let result = $part_expr ?;
			Ok (format! ("{}", result))
		}
		puzzle_info! { @rest ($parse_input, $parse_expr, $num_parts + 1) $($rest0)* }
	};
	( @rest ($parse_input:ident, $parse_expr:expr, $num_parts:expr)
		commands = [ $($commands:tt)* ];
		$($rest3:tt)*
	) => {
		fn commands (& self) -> Vec <$crate::command::PuzzleCommand> {
			let mut commands = Vec::new ();
			puzzle_info! { @commands commands $($commands)* }
			commands
		}
		puzzle_info! { @rest ($parse_input, $parse_expr, $num_parts) $($rest3)* }
	};
	( @rest ($parse_input:ident, $parse_expr:expr, $num_parts:expr)
		params = [ $($params:tt)* ];
		$($rest:tt)*
	) => {
		fn set_default_params (& mut self) {
			puzzle_info! { @params self $($params)* }
		}
		puzzle_info! { @rest ($parse_input, $parse_expr, $num_parts) $($rest)* }
	};

	( @params ) => {};
	( @params $self:ident $name:ident : $type:ty = $val:expr ; $($rest:tt)* ) => {
		$self.set_param_real (stringify! ($name).to_owned (), format! ("{}", $val as $type));
		puzzle_info! { @params $($rest)* }
	};

	( @commands $commands:ident ) => {};
	( @commands $commands:ident (
		name = $name:literal ;
		method = $method:expr ;
	) ) => {
		$commands.push ($crate::command::PuzzleCommand::new ($name, $method));
	};
	( @commands $commands:ident (
		name = $name:literal ;
		method = $method:expr ;
	) , $($rest:tt)* ) => {
		$commands.push ($crate::command::PuzzleCommand::new ($name, $method));
		puzzle_info! { @commands $commands $($rest)* }
	};

}
