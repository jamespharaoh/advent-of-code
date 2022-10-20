use super::*;

pub type Algorithm = [Pixel; 512];
pub type Pixels = GridBuf <Vec <Pixel>, Pos, 2>;
pub type Pos = pos::PosYX <i16>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Pixel {
		#[ default ]
		Dark = [ "." ],
		Light = [ "#" ],
	}
}

impl From <Pixel> for usize {
	fn from (pixel: Pixel) -> Self {
		match pixel {
			Pixel::Dark => 0,
			Pixel::Light => 1,
		}
	}
}

#[ derive (Debug) ]
pub struct Image {
	pixels: Pixels,
	default: Pixel,
}

impl Image {

	#[ must_use ]
	pub const fn new_from (pixels: Pixels, default: Pixel) -> Self {
		Self { pixels, default }
	}

	#[ must_use ]
	pub fn num_pixels (& self) -> usize {
		self.pixels.values ().filter (|& val| val != self.default).count ()
	}

	#[ must_use ]
	pub fn size (& self) -> Pos {
		self.pixels.size ()
	}

	#[ must_use ]
	pub const fn default (& self) -> Pixel {
		self.default
	}

	#[ must_use ]
	pub fn get (& self, pos: Pos) -> Pixel {
		self.pixels.get (pos).unwrap_or (self.default)
	}

	#[ must_use ]
	pub fn range (& self) -> (Pos, Pos) {
		(self.pixels.first_key (), self.pixels.last_key () + Pos { y: 1, x: 1 })
	}

}

impl Display for Image {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& self.pixels, formatter)
	}
}
