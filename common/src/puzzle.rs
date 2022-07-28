use super::*;
use nums::IntConv;

fn puzzle_invoke_real (
	puzzle: & dyn Puzzle,
	args: & [OsString],
) -> GenResult <()> {
	let mut command = Command::new (format! ("aoc-{}-day-{}", puzzle.year (), puzzle.day ()));
	for part_num in 1 ..= puzzle.num_parts () {
		command = command.subcommand (
			Command::new (format! ("part-{}", part_num))
				.arg (
					clap::Arg::new ("repeat")
						.long ("repeat")
						.value_parser (clap::value_parser! (u64).range (1 .. ))
						.takes_value (true)
						.default_value ("1")
						.help ("Number of times to repeat the calculation"))
		);
	}
	for puzzle_command in puzzle.commands () {
		command = command.subcommand (
			puzzle_command.command ()
				.name (puzzle_command.name ())
		);
	}
	let matches = command.get_matches_from (args);
	let input_string = puzzle.load_input () ?;
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	use time::Instant;
	fn percentile (times: & [u64], num: u64, denom: u64) -> u64 {
		let size = times.len ().as_u64 () - 1;
		let idx: u64 = num * size / denom;
		let rem = num * size % denom;
		if rem == 0 { return times [idx.as_usize ()] }
		times [idx.as_usize ()] * (denom - rem) / denom
			+ times [idx.as_usize () + 1] * rem / denom
	}
	fn runner <InnerFn: Fn (u64) -> GenResult <()>> (repeat: u64, inner_fn: InnerFn) -> GenResult <()> {
		let times = {
			let mut times: Vec <_> = (0 .. repeat)
				.map (|idx| { inner_fn (idx) ?; Ok (Instant::now ()) })
				.scan (Instant::now (), |state, cur|
					Some (cur.map (|cur| cur - mem::replace (state, cur))))
				.map_ok (|duration| duration.as_micros ().as_u64 ())
				.collect::<GenResult <_>> () ?;
			times.sort ();
			times
		};
		if repeat == 1 { return Ok (()) }
		let total = times.iter ().map (|& val| val.as_u128 ()).sum::<u128> ();
		let mean = (total / repeat.as_u128 ()).as_u64 ();
		let disp_float = |val, ref_val|
			if ref_val >= 2_000_000.0 { format! ("{:.3}s", val / 1_000_000.0) }
			else if ref_val >= 2_000.0 { format! ("{:.3}ms", val / 1_000.0) }
			else { format! ("{:.0}µs", val) };
		let disp = |val: u128| disp_float (val.as_f64 (), val.as_f64 ());
		let disp_mean = |val: u64| disp_float (val.as_f64 (), mean.as_f64 ());
		let disp_pc = |pc| disp_float (percentile (& times, pc, 1000).as_f64 (), mean.as_f64 ());
		print! ("Statistics: total={} count={} mean={},", disp (total), repeat, disp_mean (mean));
		const PERCENTILE_OPTIONS: & [(u64, & [u64])] = & [
			(1000, & [0, 500, 900, 990, 999, 1000]),
			(100, & [0, 500, 900, 990, 1000]),
			(25, & [0, 500, 750, 900, 1000]),
			(10, & [0, 500, 900, 1000]),
			(0, & []),
		];
		for (min_repeat, percentiles) in PERCENTILE_OPTIONS.iter ().copied () {
			if repeat < min_repeat * 2 { continue }
			for percentile in percentiles.iter ().copied () {
				if percentile % 10 == 0 {
					print! (" p{}={}", percentile / 10, disp_pc (percentile));
				} else {
					print! (" p{}={}", percentile.as_f64 () / 10.0, disp_pc (percentile));
				}
			}
			if percentiles.is_empty () {
				print! (" min={} median={} max={}", disp_pc (0), disp_pc (500), disp_pc (1000));
			}
			break;
		}
		print! ("\n");
		Ok (())
	}
	match matches.subcommand () {
		None => {
			let result = puzzle.part_one (& input_lines) ?;
			println! ("Part one: {}", result);
			if puzzle.num_parts () >= 2 {
				let result = puzzle.part_two (& input_lines) ?;
				println! ("Part two: {}", result);
			}
		},
		Some (("part-1", matches)) => {
			let repeat: u64 = * matches.get_one ("repeat").unwrap ();
			runner (repeat, |idx| {
				let result = puzzle.part_one (& input_lines) ?;
				if idx == 0 { println! ("Result: {}", result); }
				Ok (())
			}) ?;
		},
		Some (("part-2", matches)) => {
			let repeat: u64 = * matches.get_one ("repeat").unwrap ();
			runner (repeat, |idx| {
				let result = puzzle.part_two (& input_lines) ?;
				if idx == 0 { println! ("Result: {}", result); }
				Ok (())
			}) ?;
		},
		Some ((name, matches)) => {
			for puzzle_command in puzzle.commands () {
				if puzzle_command.name () == name {
					return puzzle_command.invoke (matches);
				}
			}
			unreachable! ();
		},
	}
	Ok (())
}

pub trait Puzzle {

	fn dyn_puzzle (& self) -> & dyn Puzzle;

	fn name (& self) -> & 'static str;
	fn year (& self) -> u16;
	fn day (& self) -> u8;
	fn part_one (& self, _lines: & [& str]) -> GenResult <String> { unimplemented! () }
	fn part_two (& self, _lines: & [& str]) -> GenResult <String> { unimplemented! () }
	fn num_parts (& self) -> usize;

	fn commands (& self) -> Vec <PuzzleCommand> { Vec::new () }

	fn set_default_params (& mut self) { }
	fn set_param_real (& mut self, name: String, value: String);
	fn set_param (& mut self, name: & str, value: String) {
		self.set_param_real (name.to_string (), value);
	}

	fn invoke (& self, args: & [OsString]) -> GenResult <()> {
		puzzle_invoke_real (self.dyn_puzzle (), args)
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
macro_rules! puzzle_info {

	(
		name = $name:literal ;
		year = $year:literal ;
		day = $day:literal ;
		$($rest4:tt)*
	) => {
		pub fn puzzle_metadata () -> Box <dyn ::aoc_common::puzzle::Puzzle> {
			use ::aoc_common::puzzle::Puzzle;
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
		fn commands (& self) -> Vec <::aoc_common::puzzle::PuzzleCommand> {
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
		$self.set_param_real (stringify! ($name).to_string (), format! ("{}", $val as $type));
		puzzle_info! { @params $($rest)* }
	};

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

}
