use super::*;

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
