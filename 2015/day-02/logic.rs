use super::*;

use input::Input;
use model::Dim;

pub fn part_one (input: & Input) -> GenResult <Dim> {
	Ok (
		input.sizes.iter ()
			.map (|& (l, w, h)| {
				let sides = [ chk! (l * w) ?, chk! (w * h) ?, chk! (h * l) ? ];
				let total = sides.into_iter ().try_fold (0, |sum, val| chk! (sum + val)) ?;
				let smallest = sides.into_iter ().min ().unwrap ();
				chk! (2 * total + smallest)
			})
			.try_fold (0, |sum, val| { let val = val ?; chk! (sum + val) }) ?
	)
}

pub fn part_two (input: & Input) -> GenResult <Dim> {
	Ok (
		input.sizes.iter ()
			.map (|& (l, w, h)| {
				let pairs = [ chk! (l + w) ?, chk! (w + h) ?, chk! (h + l) ? ];
				let smallest = pairs.into_iter ().min ().unwrap ();
				let product = [l, w, h].into_iter ().try_fold (1, |prod, val| chk! (prod * val)) ?;
				chk! (2 * smallest + product)
			})
			.try_fold (0, |sum, val| { let val = val ?; chk! (sum + val) }) ?
	)
}

#[ cfg (test) ]
mod tests {

	use super::*;

	fn make_input (sizes: impl IntoIterator <Item = (Dim, Dim, Dim)>) -> Input {
		Input { sizes: sizes.into_iter ().collect (), params: default () }
	}

	#[ test ]
	fn part_one () {
		assert_eq_ok! (0, logic::part_one (& make_input ([ ])));
		assert_eq_ok! (101, logic::part_one (& make_input ([ (2, 3, 4), (1, 1, 10) ])));
		const BIG: u32 = 24770;
		assert_is_ok! (logic::part_one (& make_input ([ (BIG, BIG, BIG) ])));
		assert_err! ("Overflow", logic::part_one (& make_input ([ (BIG + 1, BIG + 1, BIG + 1) ])));
		assert_err! ("Overflow", logic::part_one (& make_input ([ (BIG, BIG, BIG), (BIG, BIG, BIG) ])));
	}

	#[ test ]
	fn part_two () {
		assert_eq_ok! (0, logic::part_two (& make_input ([ ])));
		assert_eq_ok! (48, logic::part_two (& make_input ([ (2, 3, 4), (1, 1, 10) ])));
		const BIG: u32 = 1625;
		assert_is_ok! (logic::part_two (& make_input ([ (BIG, BIG, BIG) ])));
		assert_err! ("Overflow", logic::part_two (& make_input ([ (BIG + 1, BIG + 1, BIG + 1) ])));
		assert_err! ("Overflow", logic::part_two (& make_input ([ (BIG, BIG, BIG), (BIG, BIG, BIG) ])));
	}

}
