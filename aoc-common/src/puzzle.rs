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
		for puzzle_command in self.commands () {
			command = command.subcommand (
				puzzle_command.command ()
					.name (puzzle_command.name ())
			);
		}
		let matches = command.get_matches_from (args);
		use time::Instant;
		fn percentile (times: & [u64], num: u64, denom: u64) -> u64 {
			let size = times.len () as u64 - 1;
			let idx = num * size / denom;
			let rem = num * size % denom;
			if rem == 0 { return times [idx as usize] }
			times [idx as usize] * (denom - rem) / denom + times [idx as usize + 1] * rem / denom
		}
		fn runner <InnerFn: Fn (u64) -> GenResult <()>> (repeat: u64, inner_fn: InnerFn) -> GenResult <()> {
			let times = {
				let mut times: Vec <_> = (0 .. repeat)
					.map (|idx| { inner_fn (idx) ?; Ok (Instant::now ()) })
					.scan (Instant::now (), |state, cur|
						Some (cur.map (|cur| cur - mem::replace (state, cur))))
					.map_ok (|duration| duration.as_micros () as u64)
					.collect::<GenResult <_>> () ?;
				times.sort ();
				times
			};
			if repeat == 1 { return Ok (()) }
			let total = times.iter ().map (|& val| val as u128).sum::<u128> ();
			let mean = (total / repeat as u128) as u64;
			let disp_float = |val, ref_val|
				if ref_val >= 2_000_000.0 { format! ("{:.3}s", val / 1_000_000.0) }
				else if ref_val >= 2_000.0 { format! ("{:.3}ms", val / 1_000.0) }
				else { format! ("{:.0}µs", val) };
			let disp = |val| disp_float (val as f32, val as f32);
			let disp_mean = |val| disp_float (val as f32, mean as f32);
			let disp_pc = |pc| disp_float (percentile (& times, pc, 1000) as f32, mean as f32);
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
						print! (" p{}={}", percentile as f32 / 10.0, disp_pc (percentile));
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
				let result = self.invoke_part_one () ?;
				println! ("Part one: {}", result);
				if self.num_parts () >= 2 {
					let result = self.invoke_part_two () ?;
					println! ("Part two: {}", result);
				}
			},
			Some (("part-1", matches)) => {
				let repeat: u64 = * matches.get_one ("repeat").unwrap ();
				runner (repeat, |idx| {
					let result = self.invoke_part_one () ?;
					if idx == 0 { println! ("Result: {}", result); }
					Ok (())
				}) ?;
			},
			Some (("part-2", matches)) => {
				let repeat: u64 = * matches.get_one ("repeat").unwrap ();
				runner (repeat, |idx| {
					let result = self.invoke_part_two () ?;
					if idx == 0 { println! ("Result: {}", result); }
					Ok (())
				}) ?;
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
		Ok (())
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
