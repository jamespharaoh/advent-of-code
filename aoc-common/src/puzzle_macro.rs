#[ macro_export ]
macro_rules! puzzle_info {

	( @commands $commands:ident ) => {};
	( @commands $commands:ident (
		name = $name:literal ;
		method = $method:expr ;
	) ) => {
		$commands.push (::aoc_common::puzzle::PuzzleCommand::new ($name, $method));
	};
	( @commands $commands:ident (
		name = $name:literal ;
		method = $method:expr ;
	) , $($rest:tt)* ) => {
		$commands.push (::aoc_common::puzzle::PuzzleCommand::new ($name, $method));
		puzzle_info! { @commands $commands $($rest)* }
	};

	( @rest ) => {};
	( @rest commands = [ $($commands:tt)* ] ; $($rest:tt)* ) => {
		fn commands (& self) -> Vec <::aoc_common::puzzle::PuzzleCommand> {
			let mut commands = Vec::new ();
			puzzle_info! { @commands commands $($commands)* }
			commands
		}
		puzzle_info! { @rest $($rest)* }
	};

	(
		name = $name:literal ;
		year = $year:literal ;
		day = $day:literal ;
		part_one = |$part_one_lines:ident| $part_one:expr ;
		part_two = |$part_two_lines:ident| $part_two:expr ;
		$($rest:tt)*
	) => {
		pub fn puzzle_metadata () -> Box <dyn ::aoc_common::puzzle::Puzzle> {
			struct ThisPuzzle;
			impl ::aoc_common::puzzle::Puzzle for ThisPuzzle {
				fn name (& self) -> & 'static str { $name }
				fn year (& self) -> u16 { $year }
				fn day (& self) -> u8 { $day }
				fn part_one (& self, $part_one_lines: & [& str]) -> GenResult <String> {
					$part_one.map (|val| format! ("{}", val))
				}
				fn part_two (& self, $part_two_lines: & [& str]) -> GenResult <String> {
					$part_two.map (|val| format! ("{}", val))
				}
				puzzle_info! { @rest $($rest)* }
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
		pub const fn puzzle_metadata () -> Box <dyn ::aoc_common::puzzle::Puzzle> {
			struct ThisPuzzle;
			impl ::aoc_common::puzzle::Puzzle for ThisPuzzle {
				fn name (& self) -> & 'static str { $name }
				fn year (& self) -> u16 { $year }
				fn day (& self) -> u8 { $day }
				fn part_one (& self, $part_one_lines: & [& str]) -> GenResult <String> {
					$part_one.map (|val| format! ("{}", val))
				}
				fn part_two (& self, $part_two_lines: & [& str]) -> GenResult <String> {
					$part_two.map (|val| format! ("{}", val))
				}
				puzzle_info! { @rest $($rest)* }
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
		pub fn puzzle_metadata () -> Box <dyn ::aoc_common::puzzle::Puzzle> {
			struct ThisPuzzle;
			impl ::aoc_common::puzzle::Puzzle for ThisPuzzle {
				fn name (& self) -> & 'static str { $name }
				fn year (& self) -> u16 { $year }
				fn day (& self) -> u8 { $day }
				fn num_parts (& self) -> usize { 1 }
				fn part_one (& self, $part_one_lines: & [& str]) -> GenResult <String> {
					$part_one.map (|val| format! ("{}", val))
				}
				puzzle_info! { @rest $($rest)* }
			}
			Box::new (ThisPuzzle {})
		}
	};

}
