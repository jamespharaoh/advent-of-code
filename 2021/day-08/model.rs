use super::*;

use input::InputDisplay;

#[ derive (Clone, Copy, Debug) ]
pub struct Display {
	pub samples: [Digit; 10],
	pub value: [Digit; 4],
}

impl Display {
	pub fn build_vec (displays: & [InputDisplay]) -> GenResult <Vec <Self>> {
		displays.iter ()
			.map (|display| {
				Ok (Self {
					samples: display.samples.iter ()
						.map (|src| src.parse ())
						.try_array ()
						.unwrap (),
					value: display.value.iter ()
						.map (|src| src.parse ())
						.try_array ()
						.unwrap (),
				})
			})
			.collect ()
	}
}

#[ derive (Clone, Copy, Debug, Default) ]
pub struct Digit {
	pub segments: u8,
}

impl Digit {

	#[ inline ]
	#[ must_use ]
	pub const fn on (self) -> u8 {
		self.segments
	}

	#[ inline ]
	#[ must_use ]
	pub const fn off (self) -> u8 {
		self.segments ^ 0x7f
	}

	#[ inline ]
	#[ must_use ]
	pub const fn num_segments (self) -> u32 {
		self.segments.count_ones ()
	}

}

impl FromStr for Digit {
	type Err = ();
	fn from_str (src: & str) -> Result <Self, ()> {
		let mut segments = 0_u8;
		for ch in src.chars () {
			if ! ('a' ..= 'g').contains (& ch) { return Err (()) }
			segments.bit_set_assign (ch.pan_u32 () - 'a'.pan_u32 ());
		}
		Ok (Self { segments })
	}
}
