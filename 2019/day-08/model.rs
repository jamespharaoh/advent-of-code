use super::*;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Pixel {
		Black = "0",
		White = "1",
		Transparent = "2",
	}
}
