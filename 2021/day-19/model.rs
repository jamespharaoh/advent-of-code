use super::*;

use rotation::Rotation;

pub const SCANNER_HASH_TOTAL_U64S: usize = SCANNER_HASH_TOTAL_BITS / 64;

pub type Coord = i16;
pub type Pos = pos::PosXYZ <Coord>;
pub type ScannerHash = BitHash <SCANNER_HASH_TOTAL_U64S>;
pub type ScannerHasher = BitHasher <RandomHasher, SCANNER_HASH_TOTAL_U64S, SCANNER_HASH_ENTRY_BITS>;

pub (crate) struct OriginalScanner {
	pub beacons: Vec <Pos>,
	pub matched: Cell <bool>,
}

pub (crate) struct UnplacedScanner {
	pub original: Rc <OriginalScanner>,
	pub hashes: [ScannerHash; 6],
}

pub (crate) struct PlacedScanner {
	pub original: Rc <OriginalScanner>,
	pub hashes: [ScannerHash; 4],
	pub beacons: Vec <Pos>,
}

#[ derive (Clone) ]
pub (crate) struct ScannerMatch {
	pub placed: Rc <PlacedScanner>,
	pub unplaced: Rc <UnplacedScanner>,
	pub priority: u32,
	pub placed_rotate: Rotation,
	pub unplaced_rotate: Rotation,
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
