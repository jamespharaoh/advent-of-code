use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::ops::RangeInclusive;

fn main () {
	lazy_static! {
		static ref FIELD_RE: Regex =
			Regex::new (r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap ();
	}
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input_groups: Vec <& [& str]> = input_lines.split (|line| * line == "").collect ();
	if input_groups.len () != 3 { panic! (); }
	let valid_ranges: Vec <RangeInclusive <u64>> = input_groups [0].iter ().map (|line| {
		let captures = FIELD_RE.captures (line).unwrap ();
		let range_0_min: u64 = captures.get (2).unwrap ().as_str ().parse ().unwrap ();
		let range_0_max: u64 = captures.get (3).unwrap ().as_str ().parse ().unwrap ();
		let range_1_min: u64 = captures.get (4).unwrap ().as_str ().parse ().unwrap ();
		let range_1_max: u64 = captures.get (5).unwrap ().as_str ().parse ().unwrap ();
		vec! [(range_0_min ..= range_0_max), (range_1_min ..= range_1_max)]
	}).flatten ().collect ();
	let values: Vec <u64> = input_groups [2].iter ().skip (1).map (
		|line| line.split (',').map (|field_str| field_str.parse ().unwrap ()),
	).flatten ().collect ();
	let invalid: Vec <u64> = values.iter ().cloned ().filter (
		|value| ! valid_ranges.iter ().any (|valid_range| valid_range.contains (value)),
	).collect ();
	let error_rate: u64 = invalid.iter ().sum ();
	println! ("Sum of invalid values: {}", error_rate);
}
