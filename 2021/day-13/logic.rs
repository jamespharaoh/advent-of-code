#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use input::Input;
use model::Axis;
use model::Fold;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let dots = fold_once (& input.folds [0], input.dots.iter ().copied ());
	Ok (dots.len ().pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let dots = fold_multi (& input.folds, input.dots.iter ().copied ());
	let result = ocr::read_dots (dots.iter ().map (|pos| (pos.y, pos.x))) ?;
	Ok (result)
}

#[ must_use ]
pub fn fold_multi (folds: & [Fold], dots: impl IntoIterator <Item = Pos>) -> HashSet <Pos> {
	folds.iter ().fold (dots.into_iter ().collect (), |dots, fold| fold_once (fold, dots))
}

#[ must_use ]
pub fn fold_once (fold: & Fold, dots: impl IntoIterator <Item = Pos>) -> HashSet <Pos> {
	dots.into_iter ()
		.map (|mut dot| {
			match fold.axis {
				Axis::X => if dot.x > fold.val { dot.x = fold.val - (dot.x - fold.val) },
				Axis::Y => if dot.y > fold.val { dot.y = fold.val - (dot.y - fold.val) },
			}
			dot
		})
		.collect ()
}
