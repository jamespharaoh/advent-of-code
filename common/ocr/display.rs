use super::*;

pub struct DrawDots <Val, Iter> (pub Iter)
	where
		Val: Int,
		Iter: IntoIterator <Item = (Val, Val)> + Clone;

impl <Val, Iter> Display for DrawDots <Val, Iter>
	where
		Iter: IntoIterator <Item = (Val, Val)> + Clone,
		Val: Int {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let dots = {
			let mut dots_temp: Vec <(usize, usize)> =
				self.0.clone ().into_iter ()
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
		Iter: IntoIterator <Item = (Val, Val)> + Clone,
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
