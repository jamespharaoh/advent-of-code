use super::*;

use std::io::{ BufRead as _, BufReader, Write as _ };
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::ExitCode;

use puzzle::Puzzle;

pub struct RunStats {
	num_correct: usize,
	num_incorrect: usize,
	num_unknown: usize,
}

args_decl! {
	pub struct Args {
		pub input_dir: Option <PathBuf>,
		pub plain: bool,
	}
}

#[ allow (clippy::missing_inline_in_public_items) ]
#[ allow (clippy::print_stderr) ]
#[ must_use ]
pub fn main (
	puzzles: & [Box <dyn Puzzle>],
	require_answers: bool,
) -> ExitCode {

	let args = match Args::parse (env::args_os ().skip (1)) {
		Ok (args) => args,
		Err (err) => {
			eprintln! ("Error: {err}");
			return ExitCode::FAILURE;
		},
	};

	let stats = match run (puzzles, args) {
		Ok (stats) => stats,
		Err (err) => {
			eprintln! ("Error: {err}");
			return ExitCode::FAILURE;
		},
	};

	if stats.num_incorrect != 0 {
		eprintln! ("Number of incorrect answers: {}", stats.num_incorrect);
	}

	if stats.num_unknown != 0 {
		eprintln! ("Number of unknown answers: {}", stats.num_unknown);
	}

	let num_errors =
		if require_answers { stats.num_incorrect + stats.num_unknown }
		else { stats.num_incorrect };

	if num_errors != 0 { return ExitCode::FAILURE }

	ExitCode::SUCCESS

}

#[ allow (clippy::missing_inline_in_public_items) ]
pub fn run (puzzles: & [Box <dyn Puzzle>], args: Args) -> GenResult <RunStats> {

	let mut stats = RunStats {
		num_correct: 0,
		num_incorrect: 0,
		num_unknown: 0,
	};

	// work out max name length

	let name_len =
		puzzles.iter ()
			.map (|puzzle| puzzle.name ().len ())
			.max ()
			.unwrap ();

	// load answers

	let answers = Answers::load (puzzles [0].year (), & args) ?;

	// iterate puzzles

	for puzzle in puzzles.iter () {
		run_puzzle (& args, name_len, & answers, & ** puzzle, & mut stats) ?;
	}

	Ok (stats)

}

#[ allow (clippy::print_stdout) ]
fn run_puzzle (
	args: & Args,
	name_len: usize,
	answers: & Answers,
	puzzle: & dyn Puzzle,
	stats: & mut RunStats,
) -> GenResult <()> {

	// load input

	let input_string = load_input (args, puzzle) ?;

	let input_lines: Vec <& str> =
		input_string.trim_end ().split ('\n').collect ();

	// print day and puzzle name

	if args.plain {
		print! ("{:02}", puzzle.day ());
	} else {
		print! (
			"{day:02} {name:len$}",
			day = puzzle.day (),
			name = puzzle.name (),
			len = name_len + 2);
	}

	// start timer

	let start_time = time::Instant::now ();

	// iterate over parts

	let mut errors = Vec::new ();
	for (part_idx, part_name) in [ "One", "Two" ].into_iter ().enumerate () {

		// handle missing part

		if puzzle.num_parts () < part_idx + 1 {
			if ! args.plain {
				print! ("{:len$} ", "", len = answers.lens [part_idx] + 6);
			}
			continue;
		}

		// print part name

		if ! args.plain {
			print! ("{part_name}: ");
		}

		// calculate result

		io::stdout ().flush ().unwrap ();

		let result =
			if part_idx == 0 { puzzle.part_one (& input_lines) ? }
			else { puzzle.part_two (& input_lines) ? };

		// check against answers

		answers.check (puzzle, part_idx.pan_u8 (), & result, stats, & mut errors);

		// print result

		if args.plain {
			print! (" {result}");
		} else {
			print! ("{result:len$} ", len = answers.lens [part_idx] + 1);
		}

	}

	// print duration

	let end_time = time::Instant::now ();
	let duration = end_time - start_time;

	if args.plain {
		print! ("\n");
	} else {
		print! (
			"Time: {millis:>4}.{micros:02}ms\n",
			millis = duration.as_millis (),
			micros = (duration.as_micros () % 1000) / 10);
	}

	// print errors

	for error in errors {
		print! ("  {error}\n");
	}

	Ok (())

}

fn load_input (args: & Args, puzzle: & dyn Puzzle) -> GenResult <String> {
	if let Some (input_dir) = args.input_dir.as_ref () {
		let mut input_path = input_dir.clone ();
		input_path.push (format! ("day-{:02}", puzzle.day ()));
		Ok (fs::read_to_string (input_path) ?)
	} else {
		Ok (puzzle.load_input () ?)
	}
}

struct Answers {
	answers: HashMap <(u8, u8), String>,
	lens: [usize; 2],
}

impl Answers {

	fn load (year: u16, args: & Args) -> GenResult <Self> {

		let answers_path = if let Some (input_dir) = args.input_dir.as_ref () {
			let mut input_dir = input_dir.clone ();
			input_dir.push ("answers");
			input_dir
		} else {
			PathBuf::from (format! ("{year}/inputs/answers"))
		};

		let answers: HashMap <(u8, u8), String> =
			if answers_path.exists () {
				BufReader::new (File::open (answers_path) ?)
					.lines ()
					.map (move |line| {
						let line = line ?;
						let line_parts: Vec <String> =
							line.split (' ')
								.map (str::to_string)
								.collect ();
						let day: u8 = line_parts [0].parse::<u8> () ?;
						Ok (
							line_parts.into_iter ()
								.skip (1)
								.enumerate ()
								.map (move |(idx, val)|
									((day, idx.pan_u8 ()), val))
						)
					})
					.flatten_ok ()
					.collect::<GenResult <_>> () ?
			} else { HashMap::new () };

		let lens =
			answers.iter ()
				.fold ([0, 0], |[mut part_0, mut part_1], (& (_day, part), answer)| {
					if part == 0 { part_0 = cmp::max (part_0, answer.len ()); }
					if part == 1 { part_1 = cmp::max (part_1, answer.len ()); }
					[ part_0, part_1 ]
				});

		Ok (Self { answers, lens })

	}

	fn check (
		& self,
		puzzle: & dyn Puzzle,
		part_idx: u8,
		result: & str,
		stats: & mut RunStats,
		errors: & mut Vec <String>,
	) {
		if let Some (answer) = self.answers.get (& (puzzle.day (), part_idx.pan_u8 ())) {
			if result == answer {
				stats.num_correct += 1;
			} else {
				stats.num_incorrect += 1;
				errors.push (format! (
					"Part {part}: Expected {answer:?}, but calculated {result:?}",
					part = part_idx + 1,
					answer = answer,
				result = result));
			}
		} else { stats.num_unknown += 1; }
	}

}
