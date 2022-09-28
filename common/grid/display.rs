use super::*;

use aoc_pos::PosGeo;
use aoc_pos::PosGeoHexLat;
use aoc_pos::PosRowCol;
use aoc_pos::PosXY;
use aoc_pos::PosYX;

pub enum GridPosDisplayType {
	DownRight,
	RightUp,
	UpRight,
	UpRightSlant,
}

pub trait GridPosDisplay: GridPos <2> {
	fn grid_pos_display_rows_cols (pos: Self) -> (Self::Coord, Self::Coord);
	fn grid_pos_display_native (size: Self, row: Self::Coord, col: Self::Coord) -> Self;
	fn grid_pos_display_prefix (row: Self::Coord) -> Self::Coord;
}

pub trait GridPosDisplayAuto: GridPos <2> {
	const DISPLAY_TYPE: GridPosDisplayType;
}

impl <Pos: GridPosDisplayAuto> GridPosDisplay for Pos {

	#[ inline ]
	fn grid_pos_display_rows_cols (size: Self) -> (Self::Coord, Self::Coord) {
		use GridPosDisplayType::{ DownRight, RightUp, UpRight, UpRightSlant };
		let size_arr = size.to_array ();
		match Self::DISPLAY_TYPE {
			DownRight | UpRight | UpRightSlant => (size_arr [0], size_arr [1]),
			RightUp => (size_arr [1], size_arr [0]),
		}
	}

	#[ inline ]
	fn grid_pos_display_native (size: Self, row: Self::Coord, col: Self::Coord) -> Self {
		use GridPosDisplayType::{ DownRight, RightUp, UpRight, UpRightSlant };
		let size_arr = size.to_array ();
		Self::from_array (match Self::DISPLAY_TYPE {
			DownRight => [ row, col ],
			RightUp => [ col, size_arr [1] - row - Self::Coord::ONE ],
			UpRight | UpRightSlant => [ size_arr [0] - row - Self::Coord::ONE, col ],
		})
	}

	#[ inline ]
	fn grid_pos_display_prefix (row: Self::Coord) -> Self::Coord {
		use GridPosDisplayType::{ DownRight, RightUp, UpRight, UpRightSlant };
		match Self::DISPLAY_TYPE {
			DownRight | RightUp | UpRight => Self::Coord::ZERO,
			UpRightSlant => row,
		}
	}

}

impl <Val: Int> GridPosDisplayAuto for PosGeo <Val> {
	const DISPLAY_TYPE: GridPosDisplayType = GridPosDisplayType::UpRight;
}

impl <Val: Int> GridPosDisplayAuto for PosGeoHexLat <Val> {
	const DISPLAY_TYPE: GridPosDisplayType = GridPosDisplayType::UpRightSlant;
}

impl <Val: Int> GridPosDisplayAuto for PosXY <Val> {
	const DISPLAY_TYPE: GridPosDisplayType = GridPosDisplayType::RightUp;
}

impl <Val: Int> GridPosDisplayAuto for PosYX <Val> {
	const DISPLAY_TYPE: GridPosDisplayType = GridPosDisplayType::DownRight;
}

impl <Val: Int> GridPosDisplayAuto for PosRowCol <Val> {
	const DISPLAY_TYPE: GridPosDisplayType = GridPosDisplayType::DownRight;
}

impl <Storage, Pos> Display for GridBuf <Storage, Pos, 2>
	where
		Storage: GridStorage,
		Storage::Item: Display,
		Pos: GridPosDisplay {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let mut first = true;
		let (num_rows, num_cols) = Pos::grid_pos_display_rows_cols (self.size ());
		for row in 0 .. num_rows.qck_usize () {
			let row = Pos::Coord::from_usize (row).unwrap ();
			if first { first = false; } else { formatter.write_char ('\n') ?; }
			for _ in 0 .. Pos::grid_pos_display_prefix (row).qck_usize () {
				formatter.write_char (' ') ?;
			}
			for col in 0 .. num_cols.qck_usize () {
				let col = Pos::Coord::from_usize (col).unwrap ();
				let native = Pos::grid_pos_display_native (self.size (), row, col);
				let item = self.get_native (native).unwrap ();
				Display::fmt (& item, formatter) ?;
			}
		}
		Ok (())
	}

}

pub struct GridPrint <Grid, Pos, MapFn, Out>
	where
		Grid: GridView <Pos, 2>,
		Pos: GridPos <2>,
		MapFn: Fn (<Grid as GridView <Pos, 2>>::Item) -> Out,
		Out: Display {
	grid: Grid,
	map_fn: MapFn,
	phantom: PhantomData <Pos>,
}

impl <Grid, Pos, MapFn, Out> Display for GridPrint <Grid, Pos, MapFn, Out>
	where
		Grid: GridView <Pos, 2>,
		Pos: GridPos <2>,
		MapFn: Fn (<Grid as GridView <Pos, 2>>::Item) -> Out,
		Out: Display {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for row in 0 .. self.grid.size ().to_array () [0].qck_usize () {
			let row = Pos::Coord::from_usize (row).unwrap ();
			for col in 0 .. self.grid.size ().to_array () [1].qck_usize () {
				let col = Pos::Coord::from_usize (col).unwrap ();
				let pos = Pos::from_array ([ row, col ]);
				let item = self.grid.get_native (pos).unwrap ();
				write! (formatter, "{}", (self.map_fn) (item)) ?;
			}
			write! (formatter, "\n") ?;
		}
		Ok (())
	}

}

pub trait GridViewPrint <Pos>: GridView <Pos, 2>
	where Pos: GridPos <2> {

	#[ inline ]
	fn print <MapFn, Out> (self, map_fn: MapFn) -> GridPrint <Self, Pos, MapFn, Out>
		where
			MapFn: Fn (Self::Item) -> Out, Out: Display,
			Self: Sized {
		GridPrint {
			grid: self,
			map_fn,
			phantom: PhantomData,
		}
	}

}

impl <Grid, Pos> GridViewPrint <Pos> for Grid
	where
		Grid: GridView <Pos, 2>,
		Pos: GridPos <2> {
}
