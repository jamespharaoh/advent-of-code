//! Advent of Code 2021: Day 19: Beacon Scanner
//!
//! [https://adventofcode.com/2021/day/19](https://adventofcode.com/2021/day/19)
//!
//! This algorithm uses bloom filters to allow it to scale better. For each scanner we generate a
//! set of bits which have bits set according to the arrangement of the beacons they contain. If
//! there are two beacons with a specific offset between them, then a number of bits are guaranteed
//! to be set. Once we have this information for every scanner, we can prioritise the slower
//! matching process to pairs of scanners which share a large number of bits.
//!
//! Generating these hashes for each scanner is slow, so there is some further optimisation going
//! on as well. Firstly, we have to rotate scanners so that they will match. Instead of rotating
//! each scanner in every direction, we rotate scanners which we have placed in one set of
//! directions and the ones we haven't placed in another. We choose a set of directions in each
//! case to guarntee a match. Specifically we rotate placed scanners around the Z axis only, giving
//! four hashes for each scanner. We rotate unplaced scanners to move its Z axis into one of the
//! six other positions. since almost all scanners will first be unplaced and later placed, this
//! means we do a total of ten hashes for each scanner, instead of the twenty four we would have to
//! with a more naïve algorithm.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_bithash::*;
use aoc_pos as pos;

pub mod rotation;

puzzle_info! {
	name = "Beacon Scanner";
	year = 2021;
	day = 19;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Coord;
	use model::Input;
	use model::Pos;
	use nums::IntConv;
	use rotation::Rotation;

	const SCANNER_HASH_U64S: usize = 64; // 4096 bits per hash
	const SCANNER_HASH_BITS: usize = 3;  // 2 bits per entry (pair of scanners)

	type ScannerHash = BitHash <SCANNER_HASH_U64S>;
	type ScannerHasher = BitHasher <RandomHasher, SCANNER_HASH_U64S, SCANNER_HASH_BITS>;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		let (_, beacons) = calc_result (& input) ?;
		Ok (beacons.len ().as_i64 ())
	}

	pub fn part_two (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		let (scanners, _) = calc_result (& input) ?;
		let scanners: Vec <Pos> = scanners.into_iter ().collect ();
		Ok (
			scanners.iter ().combinations (2).map (|scanners| {
				let (pos_0, pos_1) = (scanners [0], scanners [1]);
				Coord::abs_diff (pos_0.x, pos_1.x)
					+ Coord::abs_diff (pos_0.y, pos_1.y)
					+ Coord::abs_diff (pos_0.z, pos_1.z)
			}).max ().unwrap ().as_i64 ()
		)
	}

	fn calc_result (input: & Input) -> GenResult <(HashSet <Pos>, HashSet <Pos>)> {
		let mut arranger = ScannerArranger::new ();
		if ! arranger.init (input) { Err ("Failed to arrange scanners") ? }
		if ! arranger.run () { Err ("Failed to arrange scanners") ? }
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

		#[ must_use ]
		fn init (& mut self, input: & Input) -> bool {
			self.original_scanners.extend (
				input.scanners.iter ().map (|scanner|
					Rc::new (OriginalScanner {
						beacons: scanner.beacons.iter ().copied ().sorted ().collect (),
						matched: Cell::new (false),
					})));
			if self.original_scanners.is_empty () { return false }
			self.unplaced_scanners = self.original_scanners.iter ()
				.map (|scanner|
					Rc::new (UnplacedScanner {
						original: Rc::clone (scanner),
						hashes: self.calc_scanner_hashes (& scanner.beacons, UNPLACED_ROTATIONS),
					}))
				.collect::<Vec <_>> ();
			true
		}

		fn run (& mut self) -> bool {
			self.place_scanner (
				& Rc::clone (& self.unplaced_scanners [0]),
				Rotation::None,
				Pos::ZERO);
			let mut scanner_matches_buffer = Vec::new ();
			'OUTER: loop {
				if self.unplaced_scanners.is_empty () { break }
				self.scanner_matches.extend (scanner_matches_buffer.drain ( .. ));
				while let Some (scanner_match) = self.scanner_matches.pop () {
					if self.unplaced_scanners.is_empty () { break 'OUTER }
					if scanner_match.unplaced.original.matched.get () { continue }
					if self.process_match (& scanner_match) { continue 'OUTER }
					scanner_matches_buffer.push (scanner_match);
				}
				self.unplaced_scanners.retain (|scanner| ! scanner.original.matched.get ());
			}
			true
		}

		fn process_match (& mut self, scanner_match: & ScannerMatch) -> bool {
			let placed = Rc::clone (& scanner_match.placed);
			let unplaced = Rc::clone (& scanner_match.unplaced);
			let rotate = Rotation::combine (
				scanner_match.placed_rotate.rev (),
				scanner_match.unplaced_rotate);
			let offset = match Self::find_offset (& placed, & unplaced, rotate) {
				Some (value) => value,
				None => { return false },
			};
			self.place_scanner (& unplaced, rotate, offset);
			true
		}

		fn find_offset (
			placed: & PlacedScanner,
			unplaced: & UnplacedScanner,
			rotate: Rotation,
		) -> Option <Pos> {
			let offsets_grouped =
				unplaced.original.beacons.iter ().copied ()
					.map (|beacon| rotate.apply (beacon))
					.cartesian_product (placed.beacons.iter ().copied ())
					.map (|(unplaced_beacon, placed_beacon)| placed_beacon - unplaced_beacon)
					.sorted ()
					.group_by (|& offset| offset);
			offsets_grouped.into_iter ()
				.map (|(offset, iter)| (offset, iter.count ()))
				.filter (|& (_, count)| count >= 12)
				.map (|(offset, _)| offset)
				.next ()
		}

		fn place_scanner (& mut self, scanner: & Rc <UnplacedScanner>, rotate: Rotation, offset: Pos) {
			let beacons = scanner.original.beacons.iter ().copied ()
				.map (|beacon| rotate.apply (beacon) + offset)
				.sorted ()
				.collect::<Vec <_>> ();
			let hashes = self.calc_scanner_hashes (& beacons, PLACED_ROTATIONS);
			let scanner = Rc::new (PlacedScanner {
				original: Rc::clone (& scanner.original),
				beacons,
				hashes,
			});
			scanner.original.matched.set (true);
			self.scanner_positions.insert (offset);
			self.beacon_positions.extend (scanner.beacons.iter ().copied ());
			self.add_scanner_matches (& scanner);
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
							priority: (placed_hash & unplaced_hash).bits ().as_u32 (),
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
		) -> [ScannerHash; LEN] {
			let mut hashers =
				[0_i32; LEN].map (|_| ScannerHasher::new_with_hasher (self.hash_builder.clone ()));
			for offset in beacons.iter ().copied ().enumerate ()
				.flat_map (|(beacon_0_idx, beacon_0)| beacons.iter ().copied ()
					.skip (beacon_0_idx + 1)
					.map (move |beacon_1| beacon_1 - beacon_0)) {
				for idx in 0 .. rotates.len () {
					let offset = rotates [idx].apply (offset);
					hashers [idx].update (cmp::max (offset, - offset));
				}
			}
			hashers.map (|hasher| hasher.finish ())
		}

	}

	struct OriginalScanner {
		beacons: Vec <Pos>,
		matched: Cell <bool>,
	}

	struct UnplacedScanner {
		original: Rc <OriginalScanner>,
		hashes: [ScannerHash; 6],
	}

	struct PlacedScanner {
		original: Rc <OriginalScanner>,
		hashes: [ScannerHash; 4],
		beacons: Vec <Pos>,
	}

	#[ derive (Clone) ]
	struct ScannerMatch {
		placed: Rc <PlacedScanner>,
		unplaced: Rc <UnplacedScanner>,
		priority: u32,
		placed_rotate: Rotation,
		unplaced_rotate: Rotation,
	}

	impl PartialEq for ScannerMatch {
		fn eq (& self, other: & Self) -> bool {
			self.priority == other.priority
		}
	}

	impl Eq for ScannerMatch {}

	impl PartialOrd for ScannerMatch {
		fn partial_cmp (& self, other: & Self) -> Option <cmp::Ordering> {
			u32::partial_cmp (& self.priority, & other.priority)
		}
	}

	impl Ord for ScannerMatch {
		fn cmp (& self, other: & Self) -> cmp::Ordering {
			u32::cmp (& self.priority, & other.priority)
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

}

mod model {

	use super::*;

	pub type Coord = i16;
	pub type Pos = pos::PosXYZ <Coord>;

	pub struct Input {
		pub scanners: Vec <InputScanner>,
	}

	#[ derive (Debug) ]
	pub struct InputScanner {
		pub name: Rc <str>,
		pub beacons: Vec <Pos>,
	}

	impl Input {
		#[ allow (clippy::string_slice) ]
		pub fn parse (lines: & [& str]) -> GenResult <Self> {
			use parser::*;
			let mut scanners = Vec::new ();
			let mut lines_iter = lines.iter ().enumerate ();
			let err = |line_idx, line| format! ("Invalid input: line {}: {}", line_idx + 1, line);
			while let Some ((line_idx, line)) = lines_iter.next () {
				if ! line.starts_with ("--- ") { Err (err (line_idx, line)) ? }
				if ! line.ends_with (" ---") { Err (err (line_idx, line)) ? }
				let name = Rc::from (& line [4 .. line.len () - 4]);
				let mut beacons = Vec::new ();
				for (line_idx, line) in lines_iter.by_ref () {
					if line.is_empty () { break }
					Parser::wrap (line, |parser| {
						beacons.push (Pos {
							x: parser.int () ?,
							y: parser.expect (",") ?.int () ?,
							z: parser.expect (",") ?.int () ?,
						});
						parser.end () ?;
						Ok (())
					}).map_parse_err (|_, _| err (line_idx, line)) ?;
				}
				scanners.push (InputScanner { name, beacons });
			}
			Ok (Self { scanners })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use aoc_common::*;
	use crate::logic;

	const EXAMPLE: & [& str] = & [
		"--- scanner 0 ---",
		"404,-588,-901", "528,-643,409", "-838,591,734", "390,-675,-793", "-537,-823,-458",
		"-485,-357,347", "-345,-311,381", "-661,-816,-575", "-876,649,763", "-618,-824,-621",
		"553,345,-567", "474,580,667", "-447,-329,318", "-584,868,-557", "544,-627,-890",
		"564,392,-477", "455,729,728", "-892,524,684", "-689,845,-530", "423,-701,434",
		"7,-33,-71", "630,319,-379", "443,580,662", "-789,900,-551", "459,-707,401",
		"",
		"--- scanner 1 ---",
		"686,422,578", "605,423,415", "515,917,-361", "-336,658,858", "95,138,22", "-476,619,847",
		"-340,-569,-846", "567,-361,727", "-460,603,-452", "669,-402,600", "729,430,532",
		"-500,-761,534", "-322,571,750", "-466,-666,-811", "-429,-592,574", "-355,545,-477",
		"703,-491,-529", "-328,-685,520", "413,935,-424", "-391,539,-444", "586,-435,557",
		"-364,-763,-893", "807,-499,-711", "755,-354,-619", "553,889,-390",
		"",
		"--- scanner 2 ---",
		"649,640,665", "682,-795,504", "-784,533,-524", "-644,584,-595", "-588,-843,648",
		"-30,6,44", "-674,560,763", "500,723,-460", "609,671,-379", "-555,-800,653",
		"-675,-892,-343", "697,-426,-610", "578,704,681", "493,664,-388", "-671,-858,530",
		"-667,343,800", "571,-461,-707", "-138,-166,112", "-889,563,-600", "646,-828,498",
		"640,759,510", "-630,509,768", "-681,-892,-333", "673,-379,-804", "-742,-814,-386",
		"577,-820,562",
		"",
		"--- scanner 3 ---",
		"-589,542,597", "605,-692,669", "-500,565,-823", "-660,373,557", "-458,-679,-417",
		"-488,449,543", "-626,468,-788", "338,-750,-386", "528,-832,-391", "562,-778,733",
		"-938,-730,414", "543,643,-506", "-524,371,-870", "407,773,750", "-104,29,83",
		"378,-903,-323", "-778,-728,485", "426,699,580", "-438,-605,-362", "-469,-447,-387",
		"509,732,623", "647,635,-688", "-868,-804,481", "614,-800,639", "595,780,-596",
		"",
		"--- scanner 4 ---",
		"727,592,562", "-293,-554,779", "441,611,-461", "-714,465,-776", "-743,427,-804",
		"-660,-479,-426", "832,-632,460", "927,-485,-438", "408,393,-506", "466,436,-512",
		"110,16,151", "-258,-428,682", "-393,719,612", "-211,-452,876", "808,-476,-593",
		"-575,615,604", "-485,667,467", "-680,325,-822", "-627,-443,-432", "872,-547,-609",
		"833,512,582", "807,604,487", "839,-516,451", "891,-625,532", "-652,-548,-490",
		"30,-46,-14",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (79, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (3621, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
