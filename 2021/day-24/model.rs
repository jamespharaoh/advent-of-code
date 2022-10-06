use super::*;

pub type Input = [u8; 14];

#[ inline ]
#[ must_use ]
pub fn input_from_str (input_str: & str) -> Input {
	input_str.chars ()
		.map (|letter| letter.to_digit (10).unwrap ().pan_u8 ())
		.array ()
}

#[ inline ]
#[ must_use ]
pub fn input_to_str (input: Input) -> String {
	input.into_iter ()
		.map (|val| char::from_digit (val.pan_u32 (), 10).unwrap ())
		.collect ()
}
