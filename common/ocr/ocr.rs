use std::fmt::{ self, Display };

use aoc_misc::*;
use aoc_nums as nums;
use nums::Int;

pub struct DrawDots <Val, Iter> (pub Iter)
	where
		Val: Int,
		Iter: Iterator <Item = (Val, Val)> + Clone;

impl <Val, Iter> Display for DrawDots <Val, Iter>
	where
		Iter: Iterator <Item = (Val, Val)> + Clone,
		Val: Int {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let dots = {
			let mut dots_temp: Vec <(usize, usize)> =
				self.0.clone ()
					.map (|(row, col)| (row.as_usize (), col.as_usize ()))
					.collect ();
			dots_temp.sort_by_key (|& (row, col)| (row, col));
			dots_temp
		};
		Self::display_fmt_real (dots, formatter)
	}

}

impl <Val, Iter> DrawDots <Val, Iter>
	where
		Iter: Iterator <Item = (Val, Val)> + Clone,
		Val: Int {

	fn display_fmt_real (dots: Vec <(usize, usize)>, formatter: & mut fmt::Formatter) -> fmt::Result {
		let mut first_row = true;
		let mut first_col = true;
		let mut row = 0;
		let mut col = 0;
		for (dot_row, dot_col) in dots {
			while first_row || row < dot_row {
				write! (formatter, "\n") ?;
				col = 0;
				if ! first_row { row += 1; }
				first_row = false;
				first_col = true;
			}
			while first_col || col < dot_col {
				write! (formatter, "  ") ?;
				if ! first_col { col += 1; }
				first_col = false;
			}
			write! (formatter, "##") ?;
			col += 1;
			first_col = false;
		}
		write! (formatter, "\n\n") ?;
		Ok (())
	}

}

#[ allow (clippy::missing_inline_in_public_items) ]
pub fn read_dots <Val: Int> (
	dots: & dyn Fn (Val, Val) -> bool,
) -> GenResult <String> {
	let mut result = String::new ();
	for offset in (0 .. ).step_by (5) {
		let mut encoded: u32 = 0;
		for row in 0 .. 6 {
			for col in 0 .. 5 {
				encoded <<= 1_i32;
				if dots (Val::from_usize (row) ?, Val::from_usize (offset + col) ?) {
					encoded |= 1;
				}
			}
		}
		result.push (match encoded {
			0x_1929_7a52 => 'A',
			0x_392e_4a5c => 'B',
			0x_1928_424c => 'C',
			// unknown => 'D',
			0x_3d0e_421e => 'E',
			0x_3d0e_4210 => 'F',
			0x_1928_5a4e => 'G',
			0x_252f_4a52 => 'H',
			// unknown => 'I',
			0x_0c21_0a4c => 'J',
			0x_254c_5292 => 'K',
			0x_2108_421e => 'L',
			// unknown => 'M',
			// unknown => 'N',
			0x_1929_4a4c => 'O',
			0x_3929_7210 => 'P',
			// unknown => 'Q',
			0x_3929_7292 => 'R',
			// unknown => 'S',
			// unknown => 'T',
			0x_2529_4a4c => 'U',
			// unknown => 'V',
			// unknown => 'W',
			// unknown => 'X',
			// unknown => 'Y',
			0x_3c22_221e => 'Z',
			0x_0000_0000 => break,
			_ => Err (format! ("Unrecognised character: {:#08x} in position {}", encoded,
				result.len () + 1)) ?,
		});
	}
	Ok (result)
}
