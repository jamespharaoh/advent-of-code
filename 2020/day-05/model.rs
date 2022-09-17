use super::*;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Seat {
	pub row: [SeatRow; 7],
	pub col: [SeatCol; 3],
}

struct_parser_display! {
	Seat { row, col } = [ @array row, @array col ]
}

impl Seat {
	#[ must_use ]
	pub fn id (& self) -> u16 {
		let row = self.row.iter ().fold (0, |sum, & row|
			sum << 1_u32 | u16::from (matches! (row, SeatRow::Back)));
		let col = self.col.iter ().fold (0, |sum, & col|
			sum << 1_u32 | u16::from (matches! (col, SeatCol::Right)));
		row << 3_u32 | col
	}
}

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum SeatRow { Back = "B", Front = "F" }
}

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum SeatCol { Left = "L", Right = "R" }
}
