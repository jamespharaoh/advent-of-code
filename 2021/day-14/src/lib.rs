//! Advent of Code 2021: Day 14: Extended Polymerization
//!
//! [https://adventofcode.com/2021/day/14](https://adventofcode.com/2021/day/14)

use aoc_common::*;

puzzle_info! {
	name = "Extended Polymerization";
	year = 2021;
	day = 14;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Input;
	use nums::IntConv;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		calc_result (10, lines)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		calc_result (40, lines)
	}

	pub fn calc_result (loops: usize, lines: & [& str]) -> GenResult <u64> {
		let input = Input::parse (lines) ?;
		let mut global_counts = {
			let mut global_counts = HashMap::new ();
			for letter in input.template.chars () {
				* global_counts.entry (letter).or_insert (0) += 1;
			}
			global_counts
		};
		struct Frame {
			counts: HashMap <char, usize>,
			letter_0: char,
			letter_1: char,
			loops: usize,
			pending: bool,
		}
		let mut stack = {
			let mut stack = Vec::new ();
			let mut letter_0 = input.template.chars ().next ().unwrap ();
			for letter_1 in input.template.chars ().skip (1) {
				stack.push (Frame {
					counts: HashMap::new (),
					letter_0,
					letter_1,
					loops,
					pending: true,
				});
				letter_0 = letter_1;
			}
			stack
		};
		let mut cache: HashMap <(char, char, usize), HashMap <char, usize>> = HashMap::new ();
		while let Some (frame) = stack.last_mut () {
			let cache_key = (frame.letter_0, frame.letter_1, frame.loops);
			if frame.pending && frame.loops > 0 {
				if let Some (cached_counts) = cache.get (& cache_key) {
					frame.counts = cached_counts.clone ();
					frame.pending = false;
					continue;
				}
				if let Some (& letter_insert) = input.rules.get (& (frame.letter_0, frame.letter_1)) {
					* frame.counts.entry (letter_insert).or_insert (0) += 1;
					frame.pending = false;
					for new_frame in vec! [
						Frame {
							counts: HashMap::new (),
							letter_0: frame.letter_0,
							letter_1: letter_insert,
							loops: frame.loops - 1,
							pending: true,
						},
						Frame {
							counts: HashMap::new (),
							letter_0: letter_insert,
							letter_1: frame.letter_1,
							loops: frame.loops - 1,
							pending: true,
						}
					] {
						stack.push (new_frame);
					}
					continue;
				}
				frame.pending = false;
				continue;
			}
			let frame = stack.pop ().unwrap ();
			let prev_counts =
				if let Some (prev_frame) =
					stack.iter_mut ().rev ()
						.find (|some_frame| some_frame.loops != frame.loops)
				{ & mut prev_frame.counts } else { & mut global_counts };
			for (& letter, count) in frame.counts.iter () {
				* prev_counts.entry (letter).or_insert (0) += count;
			}
			if frame.loops > 0 {
				cache.entry (cache_key).or_insert (frame.counts);
			}
		}
		let most = global_counts.values ().max ().unwrap ();
		let least = global_counts.values ().min ().unwrap ();
		Ok ((most - least).as_u64 ())
	}

}

mod model {

	use super::*;

	pub struct Input {
		pub template: String,
		pub rules: HashMap <(char, char), char>,
	}

	impl Input {
		pub fn parse (lines: & [& str]) -> GenResult <Input> {
			let template = lines [0].to_string ();
			let mut rules = HashMap::new ();
			for line in lines.iter ().skip (2) {
				let line_err = || format! ("Invalid input: {}", line);
				let mut line_iter = line.chars ();
				let letter_before = line_iter.next ().ok_or_else (line_err) ?;
				let letter_after = line_iter.next ().ok_or_else (line_err) ?;
				if iter::from_fn (|| line_iter.next ()).take (4).collect::<String> () != " -> " {
					Err (line_err ()) ?;
				}
				let letter_insert = line_iter.next ().ok_or_else (line_err) ?;
				rules.insert ((letter_before, letter_after), letter_insert);
			}
			Ok (Input { template, rules })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"NNCB",
		"",
		"CH -> B",
		"HH -> N",
		"CB -> H",
		"NH -> C",
		"HB -> C",
		"HC -> B",
		"HN -> C",
		"NN -> C",
		"BH -> H",
		"NC -> B",
		"NB -> B",
		"BN -> B",
		"BB -> N",
		"BC -> B",
		"CC -> N",
		"CN -> C",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (1588, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (2188189693529, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
