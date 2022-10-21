use std::fmt::{ self, Display };

use aoc_misc::prelude::*;
use aoc_nums as nums;
use nums::Int;

mod chars;

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
					.map (|(row, col)| (row.qck_usize (), col.qck_usize ()))
					.collect ();
			dots_temp.sort_by_key (|& (row, col)| (row, col));
			dots_temp.dedup ();
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
		let mut row = dots.iter ().map (|& (row, _)| row).min ().unwrap ();
		let start_col = dots.iter ().map (|& (_, col)| col).min ().unwrap ();
		let mut col = start_col;
		for (dot_row, dot_col) in dots {
			while first_row || row < dot_row {
				write! (formatter, "\n") ?;
				col = start_col;
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
	dots: impl IntoIterator <Item = (Val, Val)>,
) -> GenResult <String> {
	let dots: Vec <(Val, Val)> =
		dots.into_iter ()
			.map (|(row, col)| (col, row))
			.sorted ()
			.collect ();
	let base_row = some_or! (
		dots.iter ().copied ().map (|(_, row)| row).min (),
		return Ok (String::new ()));
	let mut result = String::new ();
	let mut dots_iter = dots.iter ().copied ().peekable ();
	while dots_iter.peek ().is_some () {
		let base_col = dots_iter.peek ().unwrap ().0;
		let mut encoded = 0_u128;
		let mut col = base_col;
		while let Some ((dot_col, dot_row)) = dots_iter.peek ().copied () {
			if dot_col == col + Val::ONE {
				col = dot_col;
			}
			if col < dot_col { break }
			if dot_col == col {
				let rel_col = dot_col - base_col;
				let rel_row = dot_row - base_row;
				if rel_col >= Val::from_usize (10).unwrap () { return Err ("Too wide".into ()) }
				if rel_row >= Val::from_usize (12).unwrap () { return Err ("Too tall".into ()) }
				let row_bit = (Val::from_usize (11).unwrap () - rel_row).pan_u32 ();
				let col_bit = (Val::from_usize (9).unwrap () - rel_col).pan_u32 ();
				let bit = row_bit * 10 + col_bit;
				encoded |= 1_u128 << bit;
			}
			dots_iter.next ().unwrap ();
		}
		if let Some (ch) = chars::decode (encoded) {
			result.push (ch);
			continue;
		}
		#[ allow (clippy::print_stderr) ]
		#[ cfg (all (debug_assertions, not (fuzzing))) ]
		{
			let mut drawn = String::new ();
			let mut val = encoded;
			for _ in 0_u32 .. 12 {
				for _ in 0_u32 .. 10 {
					drawn.push (if val & (1 << 119_u32) != 0 { '#' } else { ' ' });
					val = (val << 1_u128) & ! (0xff << 120_u32);
				}
				if val == 0 { break }
				drawn.push ('\n');
			}
			eprintln! ("{drawn}");
		}
		let mut shift = 0_u32;
		while encoded != 0 && encoded.trailing_zeros () >= 16 {
			shift += 16;
			encoded >>= 16_u32;
		}
		return Err (format! (
			"Unrecognised character: 0x{encoded:x} << {shift} in position {}",
			result.len () + 1,
		).into ());
	}
	Ok (result)
}
