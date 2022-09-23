use super::*;

impl <Storage, Pos> Display for Grid <Storage, Pos, 2>
	where
		Storage: GridStorage + Clone,
		Storage::Item: Display,
		Pos: GridPosDisplay {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match Pos::ORDER {
			GridPosDisplayOrder::RightDown => {
				let mut first = true;
				for row in 0 .. self.size [0] {
					if first { first = false; } else { write! (formatter, "\n") ?; }
					for col in 0 .. self.size [1] {
						let item = self.get_native ([row, col]).unwrap ();
						Display::fmt (& item, formatter) ?;
					}
				}
			},
			GridPosDisplayOrder::RightUp => {
				let mut first = true;
				for row in (0 .. self.size [0]).rev () {
					if first { first = false; } else { write! (formatter, "\n") ?; }
					for col in 0 .. self.size [1] {
						let item = self.get_native ([row, col]).unwrap ();
						Display::fmt (& item, formatter) ?;
					}
				}
			},
			GridPosDisplayOrder::UpRight => {
				let mut first = true;
				for row in (0 .. self.size [1]).rev () {
					if first { first = false; } else { write! (formatter, "\n") ?; }
					for col in 0 .. self.size [0] {
						let item = self.get_native ([col, row]).unwrap ();
						Display::fmt (& item, formatter) ?;
					}
				}
			},
		}
		Ok (())
	}

}

pub struct GridPrint <'grd, Storage, Pos, MapFn, Out>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <2>,
		MapFn: Fn (Storage::Item) -> Out,
		Out: Display {
	grid: & 'grd Grid <Storage, Pos, 2>,
	map_fn: MapFn,
}

impl <'grd, Storage, Pos, MapFn, Out> Display for GridPrint <'grd, Storage, Pos, MapFn, Out>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <2>,
		MapFn: Fn (Storage::Item) -> Out,
		Out: Display {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for row in 0 .. self.grid.size [0] {
			for col in 0 .. self.grid.size [1] {
				let item = self.grid.get_native ([row, col]).unwrap ();
				write! (formatter, "{}", (self.map_fn) (item)) ?;
			}
			write! (formatter, "\n") ?;
		}
		Ok (())
	}

}

impl <Storage, Pos> Grid <Storage, Pos, 2>
	where Storage: GridStorage + Clone, Pos: GridPos <2> {

	#[ inline ]
	pub const fn print <MapFn, Out> (
		& self,
		map_fn: MapFn,
	) -> GridPrint <Storage, Pos, MapFn, Out>
			where MapFn: Fn (Storage::Item) -> Out, Out: Display {
		GridPrint { grid: self, map_fn }
	}

}
