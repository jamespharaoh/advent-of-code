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
					.map (|(row, col)| (row.qck_usize (), col.qck_usize ()))
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
		if let Some (ch) = decode_char (encoded) {
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

const fn decode_char (encoded: u128) -> Option <char> {
	if encoded.trailing_zeros () >= 60 {
		decode_six (encoded)
	} else if encoded.trailing_zeros () >= 20 {
		decode_ten (encoded)
	} else {
		None
	}
}

const fn decode_ten (encoded: u128) -> Option <char> {
	Some (match encoded >> 24_u32 {
		0x_3012_0842_1084_3f08_4210_8421 => 'A',
		0x_f821_0842_10f8_2108_4210_843e => 'B',
		0x_7821_0802_0080_2008_0200_841e => 'C',
		0x_8822_0883_e088_2208_8220_0000 => 'H',
		0x_e010_0401_0040_1004_0380_0000 => 'I',
		0x_1c02_0080_2008_0200_8220_881c => 'J',
		0x_f821_0842_10f8_2408_8220_8421 => 'R',
		0x_8421_0481_2030_0c04_8120_8421 => 'X',
		0x_fc01_0040_2010_0804_0200_803f => 'Z',
		_ => return None,
	})
}

const fn decode_six (encoded: u128) -> Option <char> {
	Some (match encoded >> 64_u32 {
		0x_0060_2409_03c0_9024 => 'A',
		0x_00e0_240e_0240_9038 => 'B',
		0x_0060_2408_0200_9018 => 'C',
		0x_00f0_200e_0200_803c => 'E',
		0x_00f0_200e_0200_8020 => 'F',
		0x_0060_2408_02c0_901c => 'G',
		0x_0090_240f_0240_9024 => 'H',
		0x_0030_0401_0040_9018 => 'J',
		0x_0090_280c_0280_a024 => 'K',
		0x_0080_2008_0200_803c => 'L',
		0x_0060_2409_0240_9018 => 'O',
		0x_00e0_2409_0380_8020 => 'P',
		0x_00e0_2409_0380_a024 => 'R',
		0x_0090_2409_0240_9018 => 'U',
		0x_0088_2205_0080_2008 => 'Y',
		0x_00f0_0402_0100_803c => 'Z',
		_ => return None,
	})
}
