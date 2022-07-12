use aoc_common::*;

pub mod rotation;

puzzle_info! {
	name = "Beacon Scanner";
	year = 2021;
	day = 19;
	part_one = |lines| logic::calc_result_part_one (lines);
	part_two = |lines| logic::calc_result_part_two (lines);
}

mod logic {

	use super::*;
	use model::Coord;
	use model::Input;
	use model::Pos;
	use rotation::Rotation;
	use bithash::BitHash;

	type ScannerHash = BitHash <48>;

	pub fn calc_result_part_one (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		let (_, beacons) = calc_result (& input) ?;
		Ok (beacons.len () as i64)
	}

	pub fn calc_result_part_two (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		let (scanners, _) = calc_result (& input) ?;
		let scanners: Vec <Pos> = scanners.into_iter ().collect ();
		Ok (
			scanners.iter ().combinations (2).map (|scanners| {
				let (pos_0, pos_1) = (scanners [0], scanners [1]);
				Coord::abs_diff (pos_0.x, pos_1.x)
					+ Coord::abs_diff (pos_0.y, pos_1.y)
					+ Coord::abs_diff (pos_0.z, pos_1.z)
			}).max ().unwrap () as i64
		)
	}

	fn calc_result (input: & Input) -> GenResult <(HashSet <Pos>, HashSet <Pos>)> {

		struct OriginalScanner {
			beacons: Vec <Pos>,
			hash: ScannerHash,
			matched: Cell <bool>,
		}

		struct RotatedScanner {
			original: Rc <OriginalScanner>,
			beacons: Vec <Pos>,
			hash: ScannerHash,
		}

		struct PlacedScanner {
			original: Rc <OriginalScanner>,
			beacons: Vec <Pos>,
			hash: ScannerHash,
		}

		fn calc_scanner_hash (beacons: & [Pos]) -> ScannerHash {
			beacons.iter ().copied ().enumerate ()
				.flat_map (|(beacon_0_idx, beacon_0)| beacons.iter ().copied ()
					.skip (beacon_0_idx + 1)
					.map (move |beacon_1| beacon_1 - beacon_0))
				.fold (ScannerHash::new (), |hash, val| hash.update (& val))
		}

		let mut original_scanners = input.scanners.iter ().map (|(_, scanner)|
			Rc::new (OriginalScanner {
				beacons: scanner.beacons.iter ().copied ().sorted ().collect (),
				hash: calc_scanner_hash (& scanner.beacons),
				matched: Cell::new (false),
			})
		).collect::<Vec <_>> ();

		let base_scanner = original_scanners.remove (0);
		let base_scanner = Rc::new (PlacedScanner {
			original: base_scanner.clone (),
			beacons: base_scanner.beacons.clone (),
			hash: base_scanner.hash,
		});

		let mut rotated_scanners = original_scanners.iter ().flat_map (|scanner|
			Rotation::ALL.iter ().copied ().map (|rotate| {
				let beacons = scanner.beacons.iter ().copied ()
					.map (|beacon| rotate.apply (beacon))
					.sorted ()
					.collect::<Vec <_>> ();
				let hash = calc_scanner_hash (& beacons);
				Rc::new (RotatedScanner {
					original: scanner.clone (),
					beacons, hash,
				})
			})
		).collect::<Vec <_>> ();

		#[ derive (Clone) ]
		struct ScannerMatch {
			placed: Rc <PlacedScanner>,
			rotated: Rc <RotatedScanner>,
			priority: usize,
		}

		impl PartialEq for ScannerMatch {
			fn eq (& self, other: & Self) -> bool {
				self.priority == other.priority
			}
		}

		impl Eq for ScannerMatch {}

		impl PartialOrd for ScannerMatch {
			fn partial_cmp (& self, other: & Self) -> Option <cmp::Ordering> {
				usize::partial_cmp (& self.priority, & other.priority)
			}
		}

		impl Ord for ScannerMatch {
			fn cmp (& self, other: & Self) -> cmp::Ordering {
				usize::cmp (& self.priority, & other.priority)
			}
		}

		let mut scanner_matches = BinaryHeap::new ();

		scanner_matches.extend (rotated_scanners.iter ().map (|rotated_scanner|
			ScannerMatch {
				placed: base_scanner.clone (),
				rotated: rotated_scanner.clone (),
				priority: (base_scanner.hash & rotated_scanner.hash).bits (),
			}
		));

		let mut scanner_positions: HashSet <Pos> = HashSet::new ();
		scanner_positions.insert (Pos::zero ());

		let mut beacon_positions: HashSet <Pos> = HashSet::new ();
		beacon_positions.extend (base_scanner.beacons.iter ().copied ());

		let mut scanner_matches_buffer = Vec::new ();
		'OUTER: loop {
			if scanner_positions.len () == input.scanners.len () { break }
			scanner_matches.extend (scanner_matches_buffer.drain ( .. ));

			while let Some (scanner_match) = scanner_matches.pop () {
				if rotated_scanners.is_empty () { break 'OUTER }
				if scanner_match.rotated.original.matched.get () { continue }
				let placed = scanner_match.placed.clone ();
				let rotated = scanner_match.rotated.clone ();
				for placed_beacon in placed.beacons.iter ().copied () {
					for rotated_beacon in rotated.beacons.iter ().copied () {
						let offset = placed_beacon - rotated_beacon;
						let count = itertools::merge_join_by(
								placed.beacons.iter ().copied (),
								rotated.beacons.iter ().copied ()
									.map (|beacon| beacon + offset),
								|left, right| left.cmp (right))
							.filter (|merged| merged.is_both ())
							.count ();
						if count < 12 { continue }
						let newly_placed = Rc::new (PlacedScanner {
							original: rotated.original.clone (),
							beacons: rotated.beacons.iter ().cloned ()
								.map (|beacon| beacon + offset).collect (),
							hash: rotated.hash,
						});
						newly_placed.original.matched.set (true);
						scanner_positions.insert (offset);
						beacon_positions.extend (newly_placed.beacons.iter ().copied ());
						scanner_matches.extend (rotated_scanners.iter ()
							.map (|other_scanner| ScannerMatch {
								placed: newly_placed.clone (),
								rotated: other_scanner.clone (),
								priority: (newly_placed.hash & other_scanner.hash).bits (),
							}));
						continue 'OUTER;
					}
				}
				scanner_matches_buffer.push (scanner_match);
			}

			rotated_scanners.retain (|scanner| ! scanner.original.matched.get ());

		}

		Ok ((scanner_positions, beacon_positions))

	}

}

mod model {

	use aoc_common::*;

	pub type Coord = i16;
	pub type Pos = pos::PosXYZ <Coord>;

	pub struct Input {
		pub scanners: HashMap <Rc <String>, InputScanner>,
		pub scanner_names: Vec <Rc <String>>,
	}

	impl Input {
		pub fn parse (lines: & [& str]) -> GenResult <Input> {
			let mut scanners = HashMap::new ();
			let mut scanner_names = Vec::new ();
			let mut lines_iter = lines.iter ();
			let mut line_idx: usize = 0;
			let err = |line_idx, line| format! ("Invalid input: {}: {}", line_idx + 1, line);
			loop {
				let line = match lines_iter.next () {
					Some (line) => line,
					None => break,
				};
				if ! line.starts_with ("--- ") { Err (err (line_idx, line)) ? }
				if ! line.ends_with (" ---") { Err (err (line_idx, line)) ? }
				let name = Rc::new (line [4 .. line.len () - 4].to_string ());
				line_idx += 1;
				let mut beacons = Vec::new ();
				loop {
					let line = match lines_iter.next () {
						Some (line) => line,
						None => break,
					};
					if line.is_empty () { line_idx += 1; break }
					let line_parts: Vec <& str> = line.split (",").collect ();
					if line_parts.len () != 3 { Err (err (line_idx, line)) ? }
					beacons.push (Pos {
						x: line_parts [0].parse ().map_err (|_| err (line_idx, line)) ?,
						y: line_parts [1].parse ().map_err (|_| err (line_idx, line)) ?,
						z: line_parts [2].parse ().map_err (|_| err (line_idx, line)) ?,
					});
					line_idx += 1;
				}
				scanners.insert (name.clone (), InputScanner { beacons });
				scanner_names.push (name);
			}
			Ok (Input { scanners, scanner_names })
		}
	}

	#[ derive (Debug) ]
	pub struct InputScanner {
		pub beacons: Vec <Pos>,
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
		assert_eq! (79, logic::calc_result_part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (3621, logic::calc_result_part_two (EXAMPLE) ?);
		Ok (())
	}

}
