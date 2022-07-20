//! Advent of Code 2021: Day 8: Seven Segment Search
//!
//! [https://adventofcode.com/2021/day/8](https://adventofcode.com/2021/day/8)

use aoc_common::*;

puzzle_info! {
	name = "Seven Segment Search";
	year = 2021;
	day = 8;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Digit;
	use model::Display;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let displays = model::parse_input (lines) ?;
		let mut count: u64 = 0;
		for display in displays.iter () {
			for digit in display.value.iter () {
				let num = digit.segments.iter ().cloned ().filter (|& segment| segment).count ();
				if [2, 3, 4, 7].contains (& num) { count += 1 }
			}
		}
		Ok (count)
	}

	#[ allow (clippy::identity_op) ]
	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let displays = model::parse_input (lines) ?;
		let mut sum: u64 = 0;
		for display in displays.iter () {
			let digits = get_digits (display);
			let digit_val = |digit| {
				digits.iter ().position (|& some_digit| some_digit == digit).unwrap () as u64
			};
			sum += [
				digit_val (display.value [0]) * 1000,
				digit_val (display.value [1]) * 100,
				digit_val (display.value [2]) * 10,
				digit_val (display.value [3]) * 1,
			].into_iter ().sum::<u64> ();
		}
		Ok (sum)
	}

	fn get_digits (display: & Display) -> Vec <Digit> {
		fn find_one <T, I: Iterator <Item = T>> (mut iter: I) -> T {
			let val = iter.next ().unwrap ();
			if iter.next ().is_some () { panic! () }
			val
		}
		let without_segment = |segment: usize| move |digit: & Digit|
			! digit.segments [segment];
		let has_num_segments = |num_segments: usize| move |digit: & Digit|
			digit.segments.iter ().cloned ().filter (|& segment| segment).count () == num_segments;
		let samples = || display.samples.iter ().cloned ();
		let samples_with_segments = |num_segments| samples ().filter (has_num_segments (num_segments));
		let digit_1 = find_one (samples_with_segments (2));
		let digit_4 = find_one (samples_with_segments (4));
		let digit_7 = find_one (samples_with_segments (3));
		let digit_8 = find_one (samples_with_segments (7));
		let segment_counts: Vec <usize> = (0 .. 7).map (
			|segment| display.samples.iter ().filter (
				|sample| sample.segments [segment],
			).count (),
		).collect ();
		let find_segment_freq = |segment_freq| segment_counts.iter ().cloned ().position (
			|some_freq| some_freq == segment_freq,
		).unwrap ();
		let segment_b = find_segment_freq (6);
		let segment_e = find_segment_freq (4);
		let segment_f = find_segment_freq (9);
		let digit_2 = find_one (samples_with_segments (5).filter (without_segment (segment_f)));
		let segment_c = find_one (
			digit_1.segments.iter ().cloned ().enumerate ()
				.filter (|& (index, value)| value && index != segment_f)
				.map (|(index, _value)| index));
		let digit_5 = find_one (samples_with_segments (5).filter (without_segment (segment_c)));
		let digit_6 = find_one (samples_with_segments (6).filter (without_segment (segment_c)));
		let digit_9 = find_one (samples_with_segments (6).filter (without_segment (segment_e)));
		let digit_3 = find_one (
			samples_with_segments (5)
				.filter (without_segment (segment_b))
				.filter (without_segment (segment_e)),
		);
		let digit_0 = find_one (
			samples_with_segments (6)
				.filter (|& sample| sample != digit_6)
				.filter (|& sample| sample != digit_9),
		);
		vec! [
			digit_0, digit_1, digit_2, digit_3, digit_4, digit_5, digit_6, digit_7, digit_8, digit_9,
		]
	}

	#[ test ]
	fn test_get_digits () {
		let display = model::parse_display (
			"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab \
			| cdfeb fcadb cdfeb cdbaf",
		).unwrap ();
		let digits = get_digits (& display);
		assert_eq! (display.samples [0], digits [8]);
		assert_eq! (display.samples [1], digits [5]);
		assert_eq! (display.samples [2], digits [2]);
		assert_eq! (display.samples [3], digits [3]);
		assert_eq! (display.samples [4], digits [7]);
		assert_eq! (display.samples [5], digits [9]);
		assert_eq! (display.samples [6], digits [6]);
		assert_eq! (display.samples [7], digits [4]);
		assert_eq! (display.samples [8], digits [0]);
		assert_eq! (display.samples [9], digits [1]);
	}

}

mod model {

	use super::*;

	pub fn parse_input (lines: & [& str]) -> GenResult <Vec <Display>> {
		let mut displays: Vec <Display> = Vec::new ();
		for line in lines {
			displays.push (parse_display (line) ?);
		}
		Ok (displays)
	}

	pub fn parse_display (input: & str) -> GenResult <Display> {
		let err = || format! ("Invalid input: {}", input);
		let line_parts: Vec <& str> = input.split (" | ").collect ();
		if line_parts.len () != 2 { Err (err ()) ? }
		let samples = parse_digits (line_parts [0]) ?;
		if samples.len () != 10 { Err (err ()) ? }
		let values = parse_digits (line_parts [1]) ?;
		if values.len () != 4 { Err (err ()) ? }
		Ok (Display {
			samples: samples.try_into ().map_err (|_| err ()) ?,
			value: values.try_into ().map_err (|_| err ()) ?,
		})
	}

	fn parse_digits (input: & str) -> GenResult <Vec <Digit>> {
		let input_parts: Vec <& str> = input.split (' ').collect ();
		let mut digits: Vec <Digit> = Vec::new ();
		for part in input_parts {
			digits.push (parse_digit (part) ?);
		}
		Ok (digits)
	}

	fn parse_digit (input: & str) -> GenResult <Digit> {
		let mut digit = Digit { segments: [false; 7] };
		for (index, letter) in ('a' ..= 'g').enumerate () {
			if input.contains (letter) {
				digit.segments [index] = true;
			}
		}
		Ok (digit)
	}

	#[ derive (Clone, Copy, Debug, PartialEq, Eq) ]
	pub struct Digit {
		pub segments: [bool; 7],
	}

	#[ derive (Clone, Copy, Debug) ]
	pub struct Display {
		pub samples: [Digit; 10],
		pub value: [Digit; 4],
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
		"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
		"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
		"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
		"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
		"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
		"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
		"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
		"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
		"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (26, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (61229, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
