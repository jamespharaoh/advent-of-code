use super::*;

pub type Coord = i8;
pub type Dir = aoc_pos::Dir2d;
pub type Grid = aoc_grid::Grid <Vec <Pixel>, Pos>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Tag = u16;
pub type TileId = u16;
pub type Tiles = HashMap <TileId, Tile>;
pub type Turn = aoc_pos::Turn2d;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Pixel {
		#[ default ]
		Black = ".",
		White = "#",
	}
}

#[ derive (Clone, Debug) ]
pub struct Tile {
	pub id: TileId,
	pub grid: Grid,
	pub tags: [Tag; 8],
}

impl Tile {

	#[ must_use ]
	pub fn new (id: TileId, grid: Grid) -> Self {
		let mut cur = grid.cursor (Pos::ZERO).unwrap ();
		let tags = [
			Dir::Right, Dir::Down, Dir::Left, Dir::Up,
			Dir::Down, Dir::Right, Dir::Up, Dir::Left,
		].map (|dir| {
			let offset = grid.offset (Pos::ZERO.try_add ((dir, 1)).unwrap ());
			let mut tag = 0_u16;
			loop {
				tag <<= 1_u32;
				if matches! (cur.item (), Pixel::White) { tag |= 1; }
				if cur.try_add_assign (offset).is_err () { break }
			}
			tag
		});
		let grid = grid
			.extend ([ (-1, -1), (-1, -1) ]).unwrap ()
			.translate (Pos::new (-1, -1)).unwrap ();
		Self { id, grid, tags }
	}

	pub fn rotate_left (& mut self) {
		self.grid = self.grid.transform ([ Dir::Left, Dir::Down ]);
		self.tags = [
			self.tags [1], self.tags [2], self.tags [3], self.tags [0],
			self.tags [7], self.tags [4], self.tags [5], self.tags [6],
		];
	}

	pub fn rotate_right (& mut self) {
		self.grid = self.grid.transform ([ Dir::Right, Dir::Up ]);
		self.tags = [
			self.tags [3], self.tags [0], self.tags [1], self.tags [2],
			self.tags [5], self.tags [6], self.tags [7], self.tags [4],
		];
	}

	pub fn rotate_around (& mut self) {
		self.grid = self.grid.transform ([ Dir::Up, Dir::Left ]);
		self.tags = [
			self.tags [2], self.tags [3], self.tags [0], self.tags [1],
			self.tags [6], self.tags [7], self.tags [4], self.tags [5],
		];
	}

	pub fn flip (& mut self) {
		self.grid = self.grid.transform ([ Dir::Right, Dir::Down ]);
		self.tags = [
			self.tags [4], self.tags [5], self.tags [6], self.tags [7],
			self.tags [0], self.tags [1], self.tags [2], self.tags [3],
		];
	}

}

impl Eq for Tile {}

impl Hash for Tile {
	fn hash <State: Hasher> (& self, state: & mut State) {
		self.id.hash (state);
	}
}

impl PartialEq for Tile {
	#[ inline ]
	fn eq (& self, other: & Self) -> bool {
		self.id == other.id
	}
}
