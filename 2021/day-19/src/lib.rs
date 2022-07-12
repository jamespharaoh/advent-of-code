use aoc_common::*;

puzzle_info! {
	name = "Beacon Scanner";
	year = 2021;
	day = 19;
	part_one = |lines| logic::calc_result_part_one (lines);
	part_two = |lines| logic::calc_result_part_two (lines);
}

mod logic {

	use aoc_common::*;
	use super::model::Input;
	use super::model::Pos;
	use super::rotation::Rotation;

	pub fn calc_result_part_one (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		let (_, beacons) = calc_result (& input) ?;
		Ok (beacons.len () as i64)
	}

	pub fn calc_result_part_two (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		let (scanners, _) = calc_result (& input) ?;
		let scanners: Vec <Pos> = scanners.into_iter ().collect ();
		let mut max_distance: u16 = 0;
		for idx_0 in 0 .. scanners.len () - 1 {
			for idx_1 in idx_0 + 1 .. scanners.len () {
				let pos_0 = scanners [idx_0];
				let pos_1 = scanners [idx_1];
				let distance = i16::abs_diff (pos_0.x, pos_1.x) + i16::abs_diff (pos_0.y, pos_1.y)
					+ i16::abs_diff (pos_0.z, pos_1.z);
				if distance > max_distance { max_distance = distance }
			}
		}
		Ok (max_distance as i64)
	}

	fn calc_result (input: & Input) -> GenResult <(HashSet <Pos>, HashSet <Pos>)> {
		let mut up_matches_by_scanner: HashMap <Rc <String>, Vec <Rc <UpMatch>>> = HashMap::new ();
		let mut done: HashMap <Rc <String>, (Rotation, Pos)> = HashMap::new ();
		let mut final_scanners: HashSet <Pos> = HashSet::new ();
		let mut final_beacons: HashSet <Pos> = HashSet::new ();
		let mut all_down_matches;
		{
			let base_scanner_name = input.scanner_names [0].clone ();
			done.insert (base_scanner_name.clone (), (Rotation::None, Pos::ZERO));
			let base_scanner = & input.scanners [& input.scanner_names [0]];
			let base_up_matches = calc_up_matches (& input, base_scanner_name.clone ());
			all_down_matches = calc_down_matches (& base_up_matches);
			final_beacons.extend (base_scanner.beacons.iter ());
		}
		while done.len () < input.scanner_names.len () {
			for up_scanner_name in input.scanner_names.iter () {
				if done.contains_key (up_scanner_name) { continue }
				let up_matches = up_matches_by_scanner.entry (up_scanner_name.clone ())
					.or_insert_with (|| calc_up_matches (& input, up_scanner_name.clone ()));
				for up_match in up_matches.iter () {
					let down_match = match all_down_matches.get (& up_match.beacons) {
						Some (down_match) => down_match,
						None => continue,
					};
					let temp_rotate = down_match.rotate.rev ().combine (up_match.rotate);
					let temp_offset = down_match.rotate.rev ().apply (down_match.origin - up_match.origin);
					let (down_rotate, down_offset) = done [& down_match.scanner_name];
					let up_rotate = down_rotate.combine (temp_rotate);
					let up_offset = down_rotate.apply (temp_offset) + down_offset;
					let up_scanner = & input.scanners [& up_match.scanner_name];
					let up_beacons: Vec <Pos> = {
						let mut up_beacons: Vec <Pos> = up_scanner.beacons.iter ().cloned ().map (
							|beacon| up_rotate.apply (beacon) + up_offset,
						).collect ();
						up_beacons.sort ();
						up_beacons
					};
					done.insert (up_scanner_name.clone (), (up_rotate, up_offset));
					final_scanners.insert (up_offset);
					final_beacons.extend (up_beacons.iter ());
					for (down_match_key, down_match) in calc_down_matches (& up_matches).into_iter () {
						all_down_matches.entry (down_match_key.clone ()).or_insert (down_match);
					}
					up_matches_by_scanner.remove (up_scanner_name);
					break;
				}
			}
		}
		Ok ((final_scanners, final_beacons))
	}

	fn calc_up_matches (input: & Input, scanner_name: Rc <String>) -> Vec <Rc <UpMatch>> {
		let scanner = & input.scanners [& scanner_name];
		let mut up_matches: Vec <Rc <UpMatch>> = Vec::new ();
		let beacons = & scanner.beacons;
		for rotate in [
			Rotation::None, Rotation::Clockwise, Rotation::CounterClockwise, Rotation::UpsideDown,
			Rotation::Around, Rotation::ClockwiseAround, Rotation::CounterClockwiseAround,
			Rotation::UpsideDownAround,
		] {
			let beacons: Vec <Pos> = beacons.iter ().cloned ().map (|beacon| rotate.apply (beacon)).collect ();
			let mut todo: VecDeque <Pos> = VecDeque::new ();
			todo.push_back (Pos::MAX);
			let mut seen: HashSet <Pos> = HashSet::new ();
			let mut origins: HashSet <Pos> = HashSet::new ();
			while let Some (base_bound) = todo.pop_front () {
				for beacon in beacons.iter () {
					if beacon.x >= base_bound.x && beacon.y < base_bound.y
						&& beacon.z >= base_bound.z { continue }
					let new_bound = Pos {
						x: cmp::min (beacon.x, base_bound.x),
						y: cmp::min (beacon.y, base_bound.y),
						z: cmp::min (beacon.z, base_bound.z),
					};
					if seen.contains (& new_bound) { continue }
					seen.insert (new_bound);
					todo.push_back (new_bound);
					let found = beacons.iter ().filter (|& beacon|
						beacon.x >= new_bound.x && beacon.y >= new_bound.y && beacon.z >= new_bound.z,
					).count ();
					if found == 12 {
						origins.insert (new_bound);
					}
				}
			}
			for origin in origins.iter ().cloned () {
				let mut beacons: Vec <Pos> = beacons.iter ().cloned ().filter (
					|beacon| beacon.x >= origin.x && beacon.y >= origin.y && beacon.z >= origin.z,
				).map (
					|beacon| Pos { x: beacon.x - origin.x, y: beacon.y - origin.y, z: beacon.z - origin.z },
				).collect ();
				beacons.sort ();
				let size = beacons.iter ().cloned ().fold (Pos::ZERO,
					|size, beacon| Pos {
						x: cmp::max (size.x, beacon.x),
						y: cmp::max (size.y, beacon.y),
						z: cmp::max (size.z, beacon.z),
					},
				);
				up_matches.push (Rc::new (UpMatch {
					scanner_name: scanner_name.clone (),
					rotate,
					origin,
					size,
					beacons: Rc::new (beacons),
				}));
			}
		}
		up_matches
	}

	fn calc_down_matches (
		up_matches: & [Rc <UpMatch>],
	) -> HashMap <Rc <Vec <Pos>>, Rc <DownMatch>> {
		let mut down_matches: HashMap <Rc <Vec <Pos>>, Rc <DownMatch>> = HashMap::new ();
		for up_match in up_matches.iter () {
			for new_rotate in [
				Rotation::ClockwiseAround,
				Rotation::UpsideDownDown,
				Rotation::UpsideDownLeft,
			] {
				let mut beacons: Vec <Pos> = up_match.beacons.iter ().cloned ().map (
					|beacon| new_rotate.apply (beacon - up_match.size),
				).collect ();
				beacons.sort ();
				let beacons = Rc::new (beacons);
				down_matches.entry (beacons.clone ()).or_insert_with (||
					Rc::new (DownMatch {
						scanner_name: up_match.scanner_name.clone (),
						rotate: new_rotate.combine (up_match.rotate),
						origin: new_rotate.apply (up_match.origin + up_match.size),
					})
				);
			}
		}
		down_matches
	}

	#[ derive (Clone) ]
	pub struct UpMatch {
		pub scanner_name: Rc <String>,
		pub rotate: Rotation,
		pub origin: Pos,
		pub size: Pos,
		pub beacons: Rc <Vec <Pos>>,
	}

	#[ derive (Clone) ]
	pub struct DownMatch {
		scanner_name: Rc <String>,
		rotate: Rotation,
		origin: Pos,
	}

}

mod model {

	use aoc_common::*;

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

	#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Pos { pub x: i16, pub y: i16, pub z: i16 }

	impl Pos {
		pub const ZERO: Pos = Pos { x: 0, y: 0, z: 0 };
		pub const MAX: Pos = Pos { x: i16::MAX, y: i16::MAX, z: i16::MAX };
	}

	impl fmt::Debug for Pos {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "Pos ({}, {}, {})", self.x, self.y, self.z) ?;
			Ok (())
		}
	}

	impl ops::Add <Pos> for Pos {
		type Output = Pos;
		fn add (self, other: Pos) -> Pos {
			Pos { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
		}
	}

	impl ops::Sub <Pos> for Pos {
		type Output = Pos;
		fn sub (self, other: Pos) -> Pos {
			Pos { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
		}
	}

}

mod rotation {

	use crate::model::Pos;

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub enum Rotation {
		None,
		Clockwise,
		CounterClockwise,
		UpsideDown,
		Up,
		ClockwiseUp,
		CounterClockwiseUp,
		UpsideDownUp,
		Down,
		ClockwiseDown,
		CounterClockwiseDown,
		UpsideDownDown,
		Left,
		ClockwiseLeft,
		CounterClockwiseLeft,
		UpsideDownLeft,
		Right,
		ClockwiseRight,
		CounterClockwiseRight,
		UpsideDownRight,
		Around,
		ClockwiseAround,
		CounterClockwiseAround,
		UpsideDownAround,
	}

	#[ allow (dead_code) ]
	impl Rotation {

		const ALL: & 'static [Rotation] = & [
			Rotation::None,
			Rotation::Clockwise,
			Rotation::CounterClockwise,
			Rotation::UpsideDown,
			Rotation::Up,
			Rotation::ClockwiseUp,
			Rotation::CounterClockwiseUp,
			Rotation::UpsideDownUp,
			Rotation::Down,
			Rotation::ClockwiseDown,
			Rotation::CounterClockwiseDown,
			Rotation::UpsideDownDown,
			Rotation::Left,
			Rotation::ClockwiseLeft,
			Rotation::CounterClockwiseLeft,
			Rotation::UpsideDownLeft,
			Rotation::Right,
			Rotation::ClockwiseRight,
			Rotation::CounterClockwiseRight,
			Rotation::UpsideDownRight,
			Rotation::Around,
			Rotation::ClockwiseAround,
			Rotation::CounterClockwiseAround,
			Rotation::UpsideDownAround,
		];

		pub fn apply (self, pos: Pos) -> Pos {
			match self {
				Rotation::None => Pos { x: pos.x, y: pos.y, z: pos.z },
				Rotation::Clockwise => Pos { x: - pos.y, y: pos.x, z: pos.z },
				Rotation::CounterClockwise => Pos { x: pos.y, y: - pos.x, z: pos.z },
				Rotation::UpsideDown => Pos { x: - pos.x, y: - pos.y, z: pos.z },
				Rotation::Up => Pos { x: pos.x, y: - pos.z, z: pos.y },
				Rotation::ClockwiseUp => Pos { x: pos.z, y: pos.x, z: pos.y },
				Rotation::CounterClockwiseUp => Pos { x: - pos.z, y: - pos.x, z: pos.y },
				Rotation::UpsideDownUp => Pos { x: - pos.x, y: pos.z, z: pos.y },
				Rotation::Down => Pos { x: pos.x, y: pos.z, z: - pos.y },
				Rotation::ClockwiseDown => Pos { x: - pos.z, y: pos.x, z: - pos.y },
				Rotation::CounterClockwiseDown => Pos { x: pos.z, y: - pos.x, z: - pos.y },
				Rotation::UpsideDownDown => Pos { x: - pos.x, y: - pos.z, z: - pos.y },
				Rotation::Left => Pos { x: pos.z, y: pos.y, z: - pos.x },
				Rotation::ClockwiseLeft => Pos { x: - pos.y, y: pos.z, z: - pos.x },
				Rotation::CounterClockwiseLeft => Pos { x: pos.y, y: - pos.z, z: - pos.x },
				Rotation::UpsideDownLeft => Pos { x: - pos.z, y: - pos.y, z: - pos.x },
				Rotation::Right => Pos { x: - pos.z, y: pos.y, z: pos.x },
				Rotation::ClockwiseRight => Pos { x: - pos.y, y: - pos.z, z: pos.x },
				Rotation::CounterClockwiseRight => Pos { x: pos.y, y: pos.z, z: pos.x },
				Rotation::UpsideDownRight => Pos { x: pos.z, y: - pos.y, z: pos.x },
				Rotation::Around => Pos { x: - pos.x, y: pos.y, z: - pos.z },
				Rotation::ClockwiseAround => Pos { x: - pos.y, y: - pos.x, z: - pos.z },
				Rotation::CounterClockwiseAround => Pos { x: pos.y, y: pos.x, z: - pos.z },
				Rotation::UpsideDownAround => Pos { x: pos.x, y: - pos.y, z: - pos.z },
			}
		}

		pub fn left (self) -> Rotation {
			match self {
				Rotation::None => Rotation::Left,
				Rotation::Clockwise => Rotation::ClockwiseUp,
				Rotation::CounterClockwise => Rotation::CounterClockwiseDown,
				Rotation::UpsideDown => Rotation::UpsideDownRight,
				Rotation::Up => Rotation::CounterClockwiseLeft,
				Rotation::ClockwiseUp => Rotation::CounterClockwiseAround,
				Rotation::CounterClockwiseUp => Rotation::CounterClockwise,
				Rotation::UpsideDownUp => Rotation::CounterClockwiseRight,
				Rotation::Down => Rotation::ClockwiseLeft,
				Rotation::ClockwiseDown => Rotation::Clockwise,
				Rotation::CounterClockwiseDown => Rotation::ClockwiseAround,
				Rotation::UpsideDownDown => Rotation::ClockwiseRight,
				Rotation::Left => Rotation::Around,
				Rotation::ClockwiseLeft => Rotation::UpsideDownUp,
				Rotation::CounterClockwiseLeft => Rotation::UpsideDownDown,
				Rotation::UpsideDownLeft => Rotation::UpsideDown,
				Rotation::Right => Rotation::None,
				Rotation::ClockwiseRight => Rotation::Up,
				Rotation::CounterClockwiseRight => Rotation::Down,
				Rotation::UpsideDownRight => Rotation::UpsideDownAround,
				Rotation::Around => Rotation::Right,
				Rotation::ClockwiseAround => Rotation::CounterClockwiseUp,
				Rotation::CounterClockwiseAround => Rotation::ClockwiseDown,
				Rotation::UpsideDownAround => Rotation::UpsideDownLeft,
			}
		}

		pub fn around (self) -> Rotation { self.left ().left () }
		pub fn right (self) -> Rotation { self.left ().left ().left () }

		pub fn clockwise (self) -> Rotation {
			match self {
				Rotation::None => Rotation::Clockwise,
				Rotation::Clockwise => Rotation::UpsideDown,
				Rotation::CounterClockwise => Rotation::None,
				Rotation::UpsideDown => Rotation::CounterClockwise,
				Rotation::Up => Rotation::ClockwiseUp,
				Rotation::ClockwiseUp => Rotation::UpsideDownUp,
				Rotation::CounterClockwiseUp => Rotation::Up,
				Rotation::UpsideDownUp => Rotation::CounterClockwiseUp,
				Rotation::Down => Rotation::ClockwiseDown,
				Rotation::ClockwiseDown => Rotation::UpsideDownDown,
				Rotation::CounterClockwiseDown => Rotation::Down,
				Rotation::UpsideDownDown => Rotation::CounterClockwiseDown,
				Rotation::Left => Rotation::ClockwiseLeft,
				Rotation::ClockwiseLeft => Rotation::UpsideDownLeft,
				Rotation::CounterClockwiseLeft => Rotation::Left,
				Rotation::UpsideDownLeft => Rotation::CounterClockwiseLeft,
				Rotation::Right => Rotation::ClockwiseRight,
				Rotation::ClockwiseRight => Rotation::UpsideDownRight,
				Rotation::CounterClockwiseRight => Rotation::Right,
				Rotation::UpsideDownRight => Rotation::CounterClockwiseRight,
				Rotation::Around => Rotation::ClockwiseAround,
				Rotation::ClockwiseAround => Rotation::UpsideDownAround,
				Rotation::CounterClockwiseAround => Rotation::Around,
				Rotation::UpsideDownAround => Rotation::CounterClockwiseAround,
			}
		}

		pub fn upside_down (self) -> Rotation { self.clockwise ().clockwise () }
		pub fn counter_clockwise (self) -> Rotation { self.clockwise ().clockwise ().clockwise () }

		pub fn up (self) -> Rotation {
			match self {
				Rotation::None => Rotation::Up,
				Rotation::Clockwise => Rotation::ClockwiseRight,
				Rotation::CounterClockwise => Rotation::CounterClockwiseLeft,
				Rotation::UpsideDown => Rotation::UpsideDownDown,
				Rotation::Up => Rotation::UpsideDownAround,
				Rotation::ClockwiseUp => Rotation::UpsideDownRight,
				Rotation::CounterClockwiseUp => Rotation::UpsideDownLeft,
				Rotation::UpsideDownUp => Rotation::UpsideDown,
				Rotation::Down => Rotation::None,
				Rotation::ClockwiseDown => Rotation::Right,
				Rotation::CounterClockwiseDown => Rotation::Left,
				Rotation::UpsideDownDown => Rotation::Around,
				Rotation::Left => Rotation::ClockwiseUp,
				Rotation::ClockwiseLeft => Rotation::Clockwise,
				Rotation::CounterClockwiseLeft => Rotation::CounterClockwiseAround,
				Rotation::UpsideDownLeft => Rotation::ClockwiseDown,
				Rotation::Right => Rotation::CounterClockwiseUp,
				Rotation::ClockwiseRight => Rotation::ClockwiseAround,
				Rotation::CounterClockwiseRight => Rotation::CounterClockwise,
				Rotation::UpsideDownRight => Rotation::CounterClockwiseDown,
				Rotation::Around => Rotation::UpsideDownUp,
				Rotation::ClockwiseAround => Rotation::ClockwiseLeft,
				Rotation::CounterClockwiseAround => Rotation::CounterClockwiseRight,
				Rotation::UpsideDownAround => Rotation::Down,
			}
		}

		pub fn flip (self) -> Rotation { self.up ().up () }
		pub fn down (self) -> Rotation { self.up ().up ().up () }

		pub fn rev (self) -> Rotation {
			match self {
				Rotation::None => Rotation::None,
				Rotation::Clockwise => Rotation::CounterClockwise,
				Rotation::CounterClockwise => Rotation::Clockwise,
				Rotation::UpsideDown => Rotation::UpsideDown,
				Rotation::Up => Rotation::Down,
				Rotation::ClockwiseUp => Rotation::CounterClockwiseRight,
				Rotation::CounterClockwiseUp => Rotation::ClockwiseLeft,
				Rotation::UpsideDownUp => Rotation::UpsideDownUp,
				Rotation::Down => Rotation::Up,
				Rotation::ClockwiseDown => Rotation::CounterClockwiseLeft,
				Rotation::CounterClockwiseDown => Rotation::ClockwiseRight,
				Rotation::UpsideDownDown => Rotation::UpsideDownDown,
				Rotation::Left => Rotation::Right,
				Rotation::ClockwiseLeft => Rotation::CounterClockwiseUp,
				Rotation::CounterClockwiseLeft => Rotation::ClockwiseDown,
				Rotation::UpsideDownLeft => Rotation::UpsideDownLeft,
				Rotation::Right => Rotation::Left,
				Rotation::ClockwiseRight => Rotation::CounterClockwiseDown,
				Rotation::CounterClockwiseRight => Rotation::ClockwiseUp,
				Rotation::UpsideDownRight => Rotation::UpsideDownRight,
				Rotation::Around => Rotation::Around,
				Rotation::ClockwiseAround => Rotation::ClockwiseAround,
				Rotation::CounterClockwiseAround => Rotation::CounterClockwiseAround,
				Rotation::UpsideDownAround => Rotation::UpsideDownAround,
			}
		}

		pub fn combine (self, other: Rotation) -> Rotation {
			match self {
				Rotation::None => other,
				Rotation::Clockwise => other.clockwise (),
				Rotation::CounterClockwise => other.counter_clockwise (),
				Rotation::UpsideDown => other.upside_down (),
				Rotation::Up => other.up (),
				Rotation::ClockwiseUp => other.up ().clockwise (),
				Rotation::CounterClockwiseUp => other.up ().counter_clockwise (),
				Rotation::UpsideDownUp => other.up ().upside_down (),
				Rotation::Down => other.down (),
				Rotation::ClockwiseDown => other.down ().clockwise (),
				Rotation::CounterClockwiseDown => other.down ().counter_clockwise (),
				Rotation::UpsideDownDown => other.down ().upside_down (),
				Rotation::Left => other.left (),
				Rotation::ClockwiseLeft => other.left ().clockwise (),
				Rotation::CounterClockwiseLeft => other.left ().counter_clockwise (),
				Rotation::UpsideDownLeft => other.left ().upside_down (),
				Rotation::Right => other.right (),
				Rotation::ClockwiseRight => other.right ().clockwise (),
				Rotation::CounterClockwiseRight => other.right ().counter_clockwise (),
				Rotation::UpsideDownRight => other.right ().upside_down (),
				Rotation::Around => other.around (),
				Rotation::ClockwiseAround => other.around ().clockwise (),
				Rotation::CounterClockwiseAround => other.around ().counter_clockwise (),
				Rotation::UpsideDownAround => other.around ().upside_down (),
			}
		}

	}

	#[ cfg (test) ]
	mod tests {

		use aoc_common::*;
		use super::*;

		#[ test ]
		fn test_rotation () {
			let base = Pos { x: 1, y: 2, z: 3 };
			let mut seen: HashSet <Pos> = HashSet::new ();
			let mut seen_left: HashSet <Rotation> = HashSet::new ();
			let mut seen_up: HashSet <Rotation> = HashSet::new ();
			let mut seen_clockwise: HashSet <Rotation> = HashSet::new ();
			fn right_handed (pos: Pos) -> bool {
				let sign = match (pos.x.abs (), pos.y.abs (), pos.z.abs ()) {
					(1, 2, 3) => 1,
					(2, 3, 1) => 1,
					(3, 1, 2) => 1,
					(3, 2, 1) => -1,
					(2, 1, 3) => -1,
					(1, 3, 2) => -1,
					_ => panic! (),
				} * pos.x.signum () * pos.y.signum () * pos.z.signum ();
				sign > 0
			}
			for rotate in Rotation::ALL.iter ().copied () {
				let result = rotate.apply (base);
				assert! (right_handed (result));
				assert! (! seen.contains (& result), "Duplicated rotation {:?}", result);
				seen.insert (result);
				let rotate_left = rotate.left ();
				assert! (! seen_left.contains (& rotate_left), "Duplicated rotation left {:?}", rotate_left);
				seen_left.insert (rotate_left);
				let rotate_up = rotate.up ();
				assert! (! seen_up.contains (& rotate_up), "Duplicated rotation up {:?}", rotate_up);
				seen_up.insert (rotate_up);
				let rotate_clockwise = rotate.clockwise ();
				assert! (! seen_clockwise.contains (& rotate_clockwise), "Duplicated rotation clockwise {:?}", rotate_clockwise);
				seen_clockwise.insert (rotate_clockwise);
				let rotate_four_lefts = rotate.left ().left ().left ().left ();
				assert_eq! (rotate, rotate_four_lefts, "Four lefts from {:?} arrives at {:?}", rotate, rotate_four_lefts);
				let rotate_four_ups = rotate.up ().up ().up ().up ();
				assert_eq! (rotate, rotate_four_ups, "Four ups from {:?} arrives at {:?}", rotate, rotate_four_ups);
				let rotate_four_clockwises = rotate.clockwise ().clockwise ().clockwise ().clockwise ();
				assert_eq! (rotate, rotate_four_clockwises, "Four clockwises from {:?} arrives at {:?}", rotate, rotate_four_clockwises);
				assert_eq! (rotate, rotate.up ().right ().down ().counter_clockwise ());
				assert_eq! (rotate, rotate.flip ().around ().upside_down ());
				let rotate_two_revs = rotate.rev ().rev ();
				assert_eq! (rotate, rotate_two_revs, "Two reverses of {:?} arrives at {:?}", rotate, rotate_two_revs);
				let pos_forward_rev = rotate.rev ().apply (rotate.apply (base));
				assert_eq! (base, pos_forward_rev, "Applying forward and reverse to {:?} arrives at {:?}", base, pos_forward_rev);
				for other in Rotation::ALL.iter ().copied () {
					let pos_apply_twice = other.apply (rotate.apply (base));
					let pos_combine = other.combine (rotate).apply (base);
					assert_eq! (pos_apply_twice, pos_combine,
						"Appling {:?} then {:?} gives {:?} but combining then applying gives {:?}",
						rotate, other, pos_apply_twice, pos_combine);
				}
			}
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
		assert_eq! (79, logic::calc_result_part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (3621, logic::calc_result_part_two (EXAMPLE) ?);
		Ok (())
	}

}
