#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

#[ inline ]
pub fn read_auto <Val: Int> (
	dots: impl IntoIterator <Item = (Val, Val)>,
) -> GenResult <String> {
	read_auto_real (
		dots.into_iter ()
			.map (|(row, col)| (row.pan_isize (), col.pan_isize ()))
			.sorted_by_key (|& (row, col)| (col, row))
			.dedup ()
			.collect ())
}

fn read_auto_real (
	dots: Vec <(isize, isize)>,
) -> GenResult <String> {
	let mut result = String::new ();
	for encoded in glyphs_iter_auto (& dots) {
		let encoded = encoded ?;
		if let Some (ch) = chars::decode (encoded) {
			result.push (ch);
		} else {
			return Err (read_error (encoded, result.len ()).into ());
		}
	}
	Ok (result)
}

pub fn read_fixed <Val: Int> (
	dots: impl IntoIterator <Item = (Val, Val)>,
	(rows, cols): (Val, Val),
) -> GenResult <String> {
	read_fixed_real (
		dots.into_iter ()
			.map (|(row, col)| (row.pan_isize (), col.pan_isize ()))
			.sorted_by_key (|& (row, col)| (col, row))
			.dedup ()
			.collect (),
		(rows.pan_isize (), cols.pan_isize ()))
}

pub fn read_fixed_real (
	dots: Vec <(isize, isize)>,
	size: (isize, isize),
) -> GenResult <String> {
	let mut result = String::new ();
	for encoded in glyphs_iter_fixed (& dots, size) {
		let encoded = encoded ?;
		if let Some (ch) = chars::decode (encoded) {
			result.push (ch);
		} else {
			return Err (read_error (encoded, result.len ()).into ());
		}
	}
	Ok (result)
}

fn read_error (mut encoded: u128, pos: usize) -> String {
	#[ allow (clippy::print_stderr) ]
	#[ cfg (all (debug_assertions, not (fuzzing))) ]
	{
		eprintln! ("{}", draw_glyph (encoded));
	}
	let mut shift = 0_u32;
	while encoded != 0 && encoded.trailing_zeros () >= 16 {
		shift += 16;
		encoded >>= 16_u32;
	}
	format! (
		"Unrecognised character: 0x{encoded:x} << {shift} in position {}",
		pos + 1)
}

fn glyphs_iter_auto (
	dots: & [(isize, isize)],
) -> impl Iterator <Item = GenResult <u128>> + '_ {
	let base_row =
		dots.iter ()
			.map (|& (row, _)| row)
			.min ()
			.unwrap_or (0);
	let mut dots_iter = dots.iter ().peekable ();
	iter::from_fn (move || {
		dots_iter.peek () ?;
		let base_col = dots_iter.peek ().unwrap ().1;
		let mut encoded = 0_u128;
		let mut col = base_col;
		while let Some (& (dot_row, dot_col)) = dots_iter.peek ().copied () {
			if dot_col == col + 1 {
				col = dot_col;
			}
			if col < dot_col { break }
			if dot_col == col {
				let rel_col = dot_col - base_col;
				let rel_row = dot_row - base_row;
				if 10 <= rel_col {
					return Some (Err ("Too wide".into ()));
				}
				if 12 <= rel_row {
					return Some (Err ("Too tall".into ()));
				}
				let row_bit = (11 - rel_row).pan_u32 ();
				let col_bit = (9 - rel_col).pan_u32 ();
				let bit = row_bit * 10 + col_bit;
				encoded |= 1_u128 << bit;
			}
			dots_iter.next ().unwrap ();
		}
		Some (Ok (encoded))
	})
}

fn glyphs_iter_fixed (
	dots: & [(isize, isize)],
	(glyph_rows, glyph_cols): (isize, isize),
) -> impl Iterator <Item = GenResult <u128>> + '_ {
	assert! (glyph_rows <= 12 && glyph_cols <= 10);
	let (base_row, mut base_col) =
		dots.iter ().fold ((isize::MAX, isize::MAX), |(base_row, base_col), & (dot_row, dot_col)|
			(cmp::min (base_row, dot_row), cmp::min (base_col, dot_col)));
	let mut dots_iter = dots.iter ().peekable ();
	iter::from_fn (move || {
		let glyph_col =
			if let Some (&& (_, dot_col)) = dots_iter.peek () { dot_col }
			else { return None };
		let mut encoded = 0_u128;
		while dots_iter.peek ()
				.map_or (false, |&& (_, dot_col)| dot_col < base_col + glyph_cols) {
			let & (dot_row, dot_col) = dots_iter.next ().unwrap ();
			let rel_col = dot_col - glyph_col;
			let rel_row = dot_row - base_row;
			if glyph_rows <= rel_row {
				return Some (Err ("Too tall".into ()));
			}
			let row_bit = (11 - rel_row).pan_u32 ();
			let col_bit = (9 - rel_col).pan_u32 ();
			let bit = row_bit * 10 + col_bit;
			encoded |= 1_u128 << bit;
		}
		base_col += glyph_cols;
		Some (Ok (encoded))
	})
}

#[ cfg (debug_assertions) ]
fn draw_glyph (encoded: u128) -> String {
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
	drawn
}
