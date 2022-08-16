//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Claim;
use model::Square;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let claims = sanitise (input) ?;
	if claims.is_empty () { return Ok (0) }
	const NUM_SPLITS: u32 = 16;
	let calc_split = |low: u16, high: u16, idx: u32|
		(low.as_u32 () + (high - low).as_u32 () * idx.as_u32 () / NUM_SPLITS).as_u16 ();
	let bound = claims.iter ().map (|claim| claim.square).reduce (Square::bound).unwrap ();
	let mut num_overlaps = 0;
	let mut squares_x: Vec <Square> = Vec::new ();
	let mut squares_xy: Vec <Square> = Vec::new ();
	let mut count_overlaps = CountOverlaps::default ();
	for part_x in 0 .. NUM_SPLITS {
		let bound = some_or! (Square::new (
			calc_split (bound.left (), bound.right (), part_x),
			bound.top (),
			calc_split (bound.left (), bound.right (), part_x + 1),
			bound.bottom (),
		), continue);
		squares_x.clear ();
		squares_x.extend (claims.iter ()
			.filter_map (|claim| Square::overlap (bound, claim.square)));
		for part_y in 0 .. NUM_SPLITS {
			let bound = some_or! (Square::new (
				bound.left (),
				calc_split (bound.top (), bound.bottom (), part_y),
				bound.right (),
				calc_split (bound.top (), bound.bottom (), part_y + 1),
			), continue);
			squares_xy.clear ();
			squares_xy.extend (squares_x.iter ()
				.filter_map (|& square| Square::overlap (bound, square)));
			num_overlaps += count_overlaps.calc (& squares_xy);
		}
	}
	Ok (num_overlaps)		
}

#[ derive (Default) ]
struct CountOverlaps {
	overlaps: Vec <Square>,
	overlaps_temp: Vec <Square>,
}

impl CountOverlaps {
	fn calc (& mut self, squares: & [Square]) -> u32 {
		self.overlaps.clear ();
		for (idx_0, square_0) in squares.iter ().copied ().enumerate () {
			for square_1 in squares.iter ().skip (idx_0 + 1).copied () {
				let new_overlap =
					some_or! (Square::overlap (square_0, square_1), continue);
				for old_overlap in self.overlaps.drain ( .. ) {
					self.overlaps_temp.extend (Square::remove (old_overlap, new_overlap));
				}
				self.overlaps_temp.push (new_overlap);
				mem::swap (& mut self.overlaps, & mut self.overlaps_temp);
			}
		}
		self.overlaps.iter ()
			.map (|overlap| overlap.area ())
			.sum ()
	}
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut included = sanitise (input) ?;
	let mut included_temp = Vec::new ();
	let mut excluded: Vec <Claim> = Vec::new ();
	while let Some (claim_0) = included.pop () {
		let mut collision = false;
		for claim_1 in excluded.iter () {
			if Square::overlap (claim_0.square, claim_1.square).is_some () {
				collision = true;
			}
		}
		for claim_1 in included.drain ( .. ) {
			if Square::overlap (claim_0.square, claim_1.square).is_some () {
				excluded.push (claim_1);
				collision = true;
			} else {
				included_temp.push (claim_1);
			}
		}
		if ! collision { return Ok (claim_0.id) }
		mem::swap (& mut included, & mut included_temp);
	}
	Err ("No solution found".into ())
}

fn sanitise (input: & Input) -> GenResult <Vec <Claim>> {
	let mut claims = input.claims.clone ();
	claims.sort_by_key (|claim| claim.id);
	if let Some ((claim, _)) = claims.iter ()
			.tuple_windows::<(_, _)> ()
			.find (|& (left, right)| left.id == right.id) {
		return Err (format! ("Duplicated claim id: {}", claim.id).into ());
	}
	Ok (claims)
}
