use super::*;

pub use dir::RouteDir;
pub use room::Room;

pub type Coord = i16;
pub type Dir = aoc_pos::DirGeo;
pub type Grid = GridBuf <Vec <Room>, Pos, 2>;
pub type Pos = aoc_pos::PosGeo <Coord>;

mod room {

	use super::*;

	enum_decl_parser_display! {
		#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub enum Room {
			#[ default ]
			None = [ "  " ],
			North = [ "┸ " ],
			South = [ "┰ " ],
			NorthSouth = [ "┃ " ],
			East = [ "━━" ],
			NorthEast = [ "┗━" ],
			SouthEast = [ "┏━" ],
			NorthSouthEast = [ "┣━" ],
			West = [ "━ " ],
			NorthWest = [ "┛ " ],
			SouthWest = [ "┓ " ],
			NorthSouthWest = [ "┫ " ],
			EastWest = [ "━━" ],
			NorthEastWest = [ "┻━" ],
			SouthEastWest = [ "┳━" ],
			NorthSouthEastWest = [ "╋━" ],
		}
	}

	impl Room {

		#[ inline ]
		#[ must_use ]
		pub const fn north (self) -> bool {
			matches! (self,
				Self::North | Self::NorthSouth | Self::NorthEast | Self::NorthSouthEast |
				Self::NorthWest | Self::NorthSouthWest | Self::NorthEastWest |
				Self::NorthSouthEastWest)
		}

		#[ inline ]
		#[ must_use ]
		pub const fn south (self) -> bool {
			matches! (self,
				Self::South | Self::NorthSouth | Self::SouthEast | Self::NorthSouthEast |
				Self::SouthWest | Self::NorthSouthWest | Self::SouthEastWest |
				Self::NorthSouthEastWest)
		}

		#[ inline ]
		#[ must_use ]
		pub const fn east (self) -> bool {
			matches! (self,
				Self::East | Self::NorthEast | Self::SouthEast | Self::NorthSouthEast |
				Self::EastWest | Self::NorthEastWest | Self::SouthEastWest | Self::NorthSouthEastWest)
		}

		#[ inline ]
		#[ must_use ]
		pub const fn west (self) -> bool {
			matches! (self,
				Self::West | Self::NorthWest | Self::SouthWest | Self::NorthSouthWest |
				Self::EastWest | Self::NorthEastWest | Self::SouthEastWest | Self::NorthSouthEastWest)
		}

		#[ inline ]
		#[ must_use ]
		pub fn doors (self) -> ArrayVec <Dir, 4> {
			let mut result = ArrayVec::new ();
			if self.north () { result.push (Dir::North); }
			if self.south () { result.push (Dir::South); }
			if self.east () { result.push (Dir::East); }
			if self.west () { result.push (Dir::West); }
			result
		}

	}

	impl BitOrAssign <RouteDir> for Room {
		fn bitor_assign (& mut self, dir: RouteDir) {
			* self = match (
				self.north () || matches! (dir, RouteDir::North),
				self.south () || matches! (dir, RouteDir::South),
				self.east () || matches! (dir, RouteDir::East),
				self.west () || matches! (dir, RouteDir::West),
			) {
				(false, false, false, false) => Self::None,
				(true, false, false, false) => Self::North,
				(false, true, false, false) => Self::South,
				(true, true, false, false) => Self::NorthSouth,
				(false, false, true, false) => Self::East,
				(true, false, true, false) => Self::NorthEast,
				(false, true, true, false) => Self::SouthEast,
				(true, true, true, false) => Self::NorthSouthEast,
				(false, false, false, true) => Self::West,
				(true, false, false, true) => Self::NorthWest,
				(false, true, false, true) => Self::SouthWest,
				(true, true, false, true) => Self::NorthSouthWest,
				(false, false, true, true) => Self::EastWest,
				(true, false, true, true) => Self::NorthEastWest,
				(false, true, true, true) => Self::SouthEastWest,
				(true, true, true, true) => Self::NorthSouthEastWest,
			}
		}
	}

}

mod dir {

	use super::*;

	enum_decl_parser_display! {

		#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
		pub enum RouteDir {
			North = [ "N" ],
			South = [ "S" ],
			East = [ "E" ],
			West = [ "W" ],
		}

	}

	impl RouteDir {

		#[ inline ]
		#[ must_use ]
		pub const fn rev (self) -> Self {
			match self {
				Self::North => Self::South,
				Self::South => Self::North,
				Self::East => Self::West,
				Self::West => Self::East,
			}
		}

	}

	impl Deref for RouteDir {

		type Target = Dir;

		#[ inline ]
		fn deref (& self) -> & Dir {
			match * self {
				Self::North => & Dir::North,
				Self::South => & Dir::South,
				Self::East => & Dir::East,
				Self::West => & Dir::West,
			}
		}

	}

}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct RouteRegex (RouteRegexString);

struct_parser_display! {
	RouteRegex (string) = [ "^", string, "$" ]
}

impl Deref for RouteRegex {
	type Target = RouteRegexString;
	fn deref (& self) -> & RouteRegexString {
		& self.0
	}
}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct RouteRegexString (Rc <[RouteRegexItem]>);

impl Deref for RouteRegexString {
	type Target = [RouteRegexItem];
	fn deref (& self) -> & [RouteRegexItem] {
		& self.0
	}
}

struct_parser_display! {
	RouteRegexString (items) = [ @collect items ]
}

enum_decl_parser_display! {

	#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum RouteRegexItem {
		Branch (branches: Rc <[RouteRegexString]>) = [
			"(", @confirm, @delim "|" branches, ")",
		],
		Span (dirs: Rc <[RouteDir]>) = [
			@confirm, @collect_some dirs,
		],
	}

}
