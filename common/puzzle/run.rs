use super::*;

pub struct RunStats {
	num_correct: usize,
	num_incorrect: usize,
	num_unknown: usize,
}

#[ allow (clippy::missing_inline_in_public_items) ]
pub fn run_year_and_exit (
	puzzles: & [Box <dyn Puzzle>],
	require_answers: bool,
) -> GenResult <()> {
	let stats = run_year (puzzles) ?;
	let num_errors =
		if require_answers { stats.num_incorrect + stats.num_unknown }
		else { stats.num_incorrect };
	#[ allow (clippy::exit) ]
	process::exit (i32::from (num_errors != 0));
}

#[ allow (clippy::missing_inline_in_public_items) ]
#[ allow (clippy::print_stdout) ]
pub fn run_year (puzzles: & [Box <dyn Puzzle>]) -> GenResult <RunStats> {

	let flush = || io::stdout ().flush ().unwrap ();

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

	let answers_path = PathBuf::from (
		format! ("{}/inputs/answers", puzzles [0].year ()));

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

	let answer_lens =
		answers.iter ()
			.fold ([0, 0], |[mut part_0, mut part_1], (& (_day, part), answer)| {
				if part == 0 { part_0 = cmp::max (part_0, answer.len ()); }
				if part == 1 { part_1 = cmp::max (part_1, answer.len ()); }
				[part_0, part_1]
			});

	// iterate puzzles

	for puzzle in puzzles.iter () {

		// load input

		let input_string = puzzle.load_input () ?;
		let input_lines: Vec <& str> =
			input_string.trim_end ().split ('\n').collect ();

		// print day and puzzle name

		print! (
			"{day:02} {name:len$}",
			day = puzzle.day (),
			name = puzzle.name (),
			len = name_len + 2);

		// start timer

		let start_time = time::Instant::now ();

		// iterate over parts

		let mut errors = Vec::new ();
		for (part_idx, part_name) in [ "One", "Two" ].into_iter ().enumerate () {

			// handle missing part

			if puzzle.num_parts () < part_idx + 1 {
				print! ("{:len$} ", "", len = answer_lens [part_idx] + 6);
				continue;
			}

			// print part name

			print! ("{}: ", part_name);

			// calculate result

			flush ();

			let result =
				if part_idx == 0 { puzzle.part_one (& input_lines) ? }
				else { puzzle.part_two (& input_lines) ? };

			// check against answers

			if let Some (answer) = answers.get (& (puzzle.day (), part_idx.pan_u8 ())) {
				if & result == answer {
					stats.num_correct += 1;
				} else if & result != answer {
					stats.num_incorrect += 1;
					errors.push (format! (
						"Part {part}: Expected {answer:?}, but calculated {result:?}",
						part = part_idx + 1,
						answer = answer,
						result = result));
				} else {
					// no answer
				}
			} else { stats.num_unknown += 1; }

			// print result

			print! ("{result:len$} ", len = answer_lens [part_idx] + 1);

		}

		// print duration

		let end_time = time::Instant::now ();
		let duration = end_time - start_time;

		print! (
			"Time: {millis:>4}.{micros:02}ms\n",
			millis = duration.as_millis (),
			micros = (duration.as_micros () % 1000) / 10);

		// print errors

		for error in errors {
			print! ("  {}\n", error);
		}

	}

	Ok (stats)

}

use aoc_args::args_decl;
args_decl! {
	#[ derive (Debug) ]
	pub struct Args {
		pub input: Option <PathBuf>,
		pub repeat: Option <u64>,
	}
}

fn percentile (times: & [u64], num: u64, denom: u64) -> u64 {
	let size = times.len ().pan_u64 () - 1;
	let idx: u64 = num * size / denom;
	let rem = num * size % denom;
	if rem == 0 { return times [idx.pan_usize ()] }
	times [idx.pan_usize ()] * (denom - rem) / denom
		+ times [idx.pan_usize () + 1] * rem / denom
}

#[ allow (clippy::print_stdout) ]
pub (crate) fn runner (
	repeat: u64,
	mut inner_fn: impl FnMut (u64) -> GenResult <()>,
) -> GenResult <()> {
	let times = {
		let mut times: Vec <_> = (0 .. repeat)
			.map (|idx| { inner_fn (idx) ?; Ok (Instant::now ()) })
			.scan (Instant::now (), |state, cur|
				Some (cur.map (|cur| cur - mem::replace (state, cur))))
			.map_ok (|duration| duration.as_micros ().pan_u64 ())
			.collect::<GenResult <_>> () ?;
		times.sort_unstable ();
		times
	};
	if repeat == 1 { return Ok (()) }
	let total = times.iter ().map (|& val| val.pan_u128 ()).sum::<u128> ();
	let mean = (total / repeat.pan_u128 ()).pan_u64 ();
	let disp_float = |val, ref_val|
		if ref_val >= 2_000_000_f64 { format! ("{:.3}s", val / 1_000_000_f64) }
		else if ref_val >= 2_000_f64 { format! ("{:.3}ms", val / 1_000_f64) }
		else { format! ("{:.0}µs", val) };
	let disp = |val: u128| disp_float (val.pan_f64 (), val.pan_f64 ());
	let disp_mean = |val: u64| disp_float (val.pan_f64 (), mean.pan_f64 ());
	let disp_pc = |pc| disp_float (percentile (& times, pc, 1000).pan_f64 (), mean.pan_f64 ());
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
				print! (" p{}={}", percentile.pan_f64 () / 10.0_f64, disp_pc (percentile));
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
