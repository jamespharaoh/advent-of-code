use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	if input.readings.is_empty () { return Err ("No readings provided".into ()) }
	let (gamma, epsilon) = iter_bits (& input.readings)
		.try_fold ((0, 0), |(gamma, epsilon), bit| {
			match count_ones (& input.readings, bit) {
				Ordering::Less => Ok ((gamma, epsilon | bit)),
				Ordering::Equal => Err ("No solution found"),
				Ordering::Greater => Ok ((gamma | bit, epsilon)),
			}
		}) ?;
	Ok (gamma.pan_u32 () * epsilon.pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	use Ordering::{ Less, Equal, Greater };
	if input.readings.is_empty () { return Err ("No readings provided".into ()) }
	let oxygen = calc_rating (& input.readings, |ord| matches! (ord, Equal | Greater)) ?;
	let co2 = calc_rating (& input.readings, |ord| matches! (ord, Less)) ?;
	Ok (oxygen.pan_u32 () * co2.pan_u32 ())
}

fn calc_rating (readings: & [u16], keep_fn: fn (Ordering) -> bool) -> GenResult <u16> {
	let mut readings = readings.to_vec ();
	let mut bits_iter = iter_bits (& readings);
	loop {
		if readings.is_empty () { return Err ("No solution found".into ()) }
		if readings.len () == 1 { return Ok (readings [0]) }
		let bit = some_or! (bits_iter.next (), break);
		let keep = if keep_fn (count_ones (& readings, bit)) { bit } else { 0 };
		readings.retain (|& rdng| (rdng & bit) == keep);
	}
	Err ("No solution found".into ())
}

fn iter_bits (readings: & [u16]) -> impl Iterator <Item = u16> {
	let max = readings.iter ().copied ().max ().unwrap ();
	let high = if 0 < max { 0x8000 >> max.leading_zeros () } else { 0 };
	itertools::iterate (high, |& bit| bit >> 1_u32)
		.take_while (|& bit| bit != 0)
}

fn count_ones (readings: & [u16], bit: u16) -> Ordering {
	let num_ones =
		readings.iter ().copied ()
			.filter (|& rdng| rdng & bit != 0)
			.count ();
	(num_ones * 2).cmp (& readings.len ())
}
