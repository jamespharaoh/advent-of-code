use super::*;

use input::Input;
use model::Coord;
use model::OriginalScanner;
use model::PlacedScanner;
use model::Pos;
use model::ScannerHash;
use model::ScannerHasher;
use model::ScannerMatch;
use model::UnplacedScanner;
use rotation::Rotation;

pub fn part_one (input: & Input) -> GenResult <i64> {
	check_input (input) ?;
	let (_, beacons) = calc_result (input) ?;
	Ok (beacons.len ().pan_i64 ())
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	check_input (input) ?;
	let (scanners, _) = calc_result (input) ?;
	let scanners: Vec <Pos> = scanners.into_iter ().collect ();
	Ok (
		scanners.iter ()
			.combinations (2)
			.map (|scanners| {
				let (pos_0, pos_1) = (scanners [0], scanners [1]);
				Coord::abs_diff (pos_0.x, pos_1.x)
					+ Coord::abs_diff (pos_0.y, pos_1.y)
					+ Coord::abs_diff (pos_0.z, pos_1.z)
			})
			.max ()
			.unwrap ()
			.pan_i64 ()
	)
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.scanners.len () < 2 {
		return Err ("Must provide at least two scanners".into ());
	}
	for scanner in & input.scanners {
		if 40 < scanner.beacons.len () {
			return Err ("Max forty beacons per scanner".into ());
		}
	}
	Ok (())
}

fn calc_result (input: & Input) -> GenResult <(HashSet <Pos>, HashSet <Pos>)> {
	let mut arranger = ScannerArranger::new ();
	if ! arranger.init (input) ? { Err ("Failed to arrange scanners") ? }
	arranger.run () ?;
	Ok ((arranger.scanner_positions, arranger.beacon_positions))
}

#[ derive (Default) ]
struct ScannerArranger {
	hash_builder: RandomHasher,
	original_scanners: Vec <Rc <OriginalScanner>>,
	unplaced_scanners: Vec <Rc <UnplacedScanner>>,
	scanner_matches: BinaryHeap <ScannerMatch>,
	scanner_positions: HashSet <Pos>,
	beacon_positions: HashSet <Pos>,
}

impl ScannerArranger {

	fn new () -> Self { default () }

	fn init (& mut self, input: & Input) -> NumResult <bool> {
		self.original_scanners.extend (
			input.scanners.iter ().map (|scanner|
				Rc::new (OriginalScanner {
					beacons: scanner.beacons.iter ().copied ().sorted ().collect (),
					matched: Cell::new (false),
				})));
		if self.original_scanners.is_empty () { return Ok (false) }
		self.unplaced_scanners = self.original_scanners.iter ()
			.map (|scanner| Ok (Rc::new (UnplacedScanner {
				original: Rc::clone (scanner),
				hashes: self.calc_scanner_hashes (& scanner.beacons, UNPLACED_ROTATIONS) ?,
			})))
			.try_collect () ?;
		Ok (true)
	}

	fn run (& mut self) -> GenResult <()> {
		self.place_scanner (
			& Rc::clone (& self.unplaced_scanners [0]),
			Rotation::None,
			Pos::ZERO) ?;
		let mut scanner_matches_buffer = Vec::new ();
		let mut num_failures = 0_u32;
		'OUTER: loop {
			if self.unplaced_scanners.is_empty () { break }
			self.scanner_matches.extend (scanner_matches_buffer.drain ( .. ));
			while let Some (scanner_match) = self.scanner_matches.pop () {
				if self.unplaced_scanners.is_empty () { break 'OUTER }
				if scanner_match.unplaced.original.matched.get () { continue }
				if self.process_match (& scanner_match) ? { continue 'OUTER }
				num_failures += 1;
				if num_failures == 10 {
					return Err ("Giving after ten failed placements".into ());
				}
				scanner_matches_buffer.push (scanner_match);
			}
			self.unplaced_scanners.retain (|scanner| ! scanner.original.matched.get ());
		}
		Ok (())
	}

	fn process_match (& mut self, scanner_match: & ScannerMatch) -> GenResult <bool> {
		let placed = Rc::clone (& scanner_match.placed);
		let unplaced = Rc::clone (& scanner_match.unplaced);
		let rotate = Rotation::combine (
			scanner_match.placed_rotate.rev (),
			scanner_match.unplaced_rotate);
		let offset = match Self::find_offset (& placed, & unplaced, rotate) ? {
			Some (value) => value,
			None => { return Ok (false) },
		};
		self.place_scanner (& unplaced, rotate, offset) ?;
		Ok (true)
	}

	fn find_offset (
		placed: & PlacedScanner,
		unplaced: & UnplacedScanner,
		rotate: Rotation,
	) -> NumResult <Option <Pos>> {
		let mut offsets_temp: Vec <_> =
			unplaced.original.beacons.iter ().copied ()
				.map (|beacon| rotate.apply (beacon))
				.cartesian_product (placed.beacons.iter ().copied ())
				.map (|(unplaced_beacon, placed_beacon)| chk! (placed_beacon - unplaced_beacon))
				.try_collect () ?;
		offsets_temp.sort ();
		let offsets_grouped = offsets_temp.into_iter ()
			.group_by (|& offset| offset);
		Ok (
			offsets_grouped.into_iter ()
				.map (|(offset, iter)| (offset, iter.count ()))
				.filter (|& (_, count)| count >= 12)
				.map (|(offset, _)| offset)
				.next ()
		)
	}

	fn place_scanner (
		& mut self,
		scanner: & Rc <UnplacedScanner>,
		rotate: Rotation,
		offset: Pos,
	) -> GenResult <()> {
		let beacons = scanner.original.beacons.iter ().copied ()
			.map (|beacon| rotate.apply (beacon) + offset)
			.sorted ()
			.collect::<Vec <_>> ();
		let hashes = self.calc_scanner_hashes (& beacons, PLACED_ROTATIONS) ?;
		let scanner = Rc::new (PlacedScanner {
			original: Rc::clone (& scanner.original),
			beacons,
			hashes,
		});
		scanner.original.matched.set (true);
		if ! self.scanner_positions.insert (offset) {
			return Err (format! ("Duplicated scanner position: {offset:?}").into ());
		}
		self.beacon_positions.extend (scanner.beacons.iter ().copied ());
		self.add_scanner_matches (& scanner);
		Ok (())
	}

	fn add_scanner_matches (& mut self, placed: & Rc <PlacedScanner>) {
		for unplaced in self.unplaced_scanners.iter () {
			if unplaced.original.matched.get () { continue }
			for (unplaced_rotate_idx, unplaced_rotate)
					in UNPLACED_ROTATIONS.iter ().copied ().enumerate () {
				let unplaced_hash = unplaced.hashes [unplaced_rotate_idx];
				for (placed_rotate_idx, placed_rotate)
						in PLACED_ROTATIONS.iter ().copied ().enumerate () {
					let placed_hash = placed.hashes [placed_rotate_idx];
					self.scanner_matches.push (ScannerMatch {
						placed: Rc::clone (placed),
						unplaced: Rc::clone (unplaced),
						priority: (placed_hash & unplaced_hash).bits ().pan_u32 (),
						placed_rotate,
						unplaced_rotate,
					});
				}
			}
		}
	}

	fn calc_scanner_hashes <const LEN: usize> (
		& self,
		beacons: & [Pos],
		rotates: & [Rotation; LEN],
	) -> NumResult <[ScannerHash; LEN]> {
		let mut hashers =
			[0_i32; LEN].map (|_| ScannerHasher::new_with_hasher (self.hash_builder.clone ()));
		for offset in beacons.iter ().copied ().enumerate ()
			.flat_map (|(beacon_0_idx, beacon_0)| beacons.iter ().copied ()
				.skip (beacon_0_idx + 1)
				.map (move |beacon_1| chk! (beacon_1 - beacon_0))) {
			let offset = offset ?;
			for idx in 0 .. rotates.len () {
				let offset = rotates [idx].apply (offset);
				hashers [idx].update (cmp::max (offset, - offset));
			}
		}
		Ok (hashers.map (|hasher| hasher.finish ()))
	}

}

const PLACED_ROTATIONS: & [Rotation; 4] = & [
	Rotation::None, Rotation::Clockwise,
	Rotation::UpsideDown, Rotation::CounterClockwise,
];

const UNPLACED_ROTATIONS: & [Rotation; 6] = & [
	Rotation::None, Rotation::Up, Rotation::Right,
	Rotation::Around, Rotation::Down, Rotation::Left,
];
