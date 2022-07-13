use super::*;
use model::Pos;

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
pub enum Rotation {
	None, Clockwise, CounterClockwise, UpsideDown,
	Up, ClockwiseUp, CounterClockwiseUp, UpsideDownUp,
	Down, ClockwiseDown, CounterClockwiseDown, UpsideDownDown,
	Left, ClockwiseLeft, CounterClockwiseLeft, UpsideDownLeft,
	Right, ClockwiseRight, CounterClockwiseRight, UpsideDownRight,
	Around, ClockwiseAround, CounterClockwiseAround, UpsideDownAround,
}

#[ allow (dead_code) ]
impl Rotation {

	pub const ALL: & 'static [Rotation; 24] = & [
		Rotation::None, Rotation::Clockwise,
		Rotation::CounterClockwise, Rotation::UpsideDown,
		Rotation::Up, Rotation::ClockwiseUp,
		Rotation::CounterClockwiseUp, Rotation::UpsideDownUp,
		Rotation::Down, Rotation::ClockwiseDown,
		Rotation::CounterClockwiseDown, Rotation::UpsideDownDown,
		Rotation::Left, Rotation::ClockwiseLeft,
		Rotation::CounterClockwiseLeft, Rotation::UpsideDownLeft,
		Rotation::Right, Rotation::ClockwiseRight,
		Rotation::CounterClockwiseRight, Rotation::UpsideDownRight,
		Rotation::Around, Rotation::ClockwiseAround,
		Rotation::CounterClockwiseAround, Rotation::UpsideDownAround,
	];

	pub fn idx (self) -> usize {
		match self {
			Rotation::None => 0,
			Rotation::Clockwise => 1,
			Rotation::CounterClockwise => 2,
			Rotation::UpsideDown => 3,
			Rotation::Up => 4,
			Rotation::ClockwiseUp => 5,
			Rotation::CounterClockwiseUp => 6,
			Rotation::UpsideDownUp => 7,
			Rotation::Down => 8,
			Rotation::ClockwiseDown => 9,
			Rotation::CounterClockwiseDown => 10,
			Rotation::UpsideDownDown => 11,
			Rotation::Left => 12,
			Rotation::ClockwiseLeft => 13,
			Rotation::CounterClockwiseLeft => 14,
			Rotation::UpsideDownLeft => 15,
			Rotation::Right => 16,
			Rotation::ClockwiseRight => 17,
			Rotation::CounterClockwiseRight => 18,
			Rotation::UpsideDownRight => 19,
			Rotation::Around => 20,
			Rotation::ClockwiseAround => 21,
			Rotation::CounterClockwiseAround => 22,
			Rotation::UpsideDownAround => 23,
		}
	}

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
					"Applying {:?} then {:?} gives {:?} but combining then applying gives {:?}",
					rotate, other, pos_apply_twice, pos_combine);
				for other_base in Rotation::ALL.iter ().copied ().map (|rot| rot.apply (base)) {
					if other.apply (other_base) != result { continue }
					assert_eq! (other.rev ().combine (rotate).apply (base), other_base);
				}
			}
		}
	}

}
