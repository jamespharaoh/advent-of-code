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

	pub const ALL: & 'static [Self; 24] = & [
		Self::None, Self::Clockwise,
		Self::CounterClockwise, Self::UpsideDown,
		Self::Up, Self::ClockwiseUp,
		Self::CounterClockwiseUp, Self::UpsideDownUp,
		Self::Down, Self::ClockwiseDown,
		Self::CounterClockwiseDown, Self::UpsideDownDown,
		Self::Left, Self::ClockwiseLeft,
		Self::CounterClockwiseLeft, Self::UpsideDownLeft,
		Self::Right, Self::ClockwiseRight,
		Self::CounterClockwiseRight, Self::UpsideDownRight,
		Self::Around, Self::ClockwiseAround,
		Self::CounterClockwiseAround, Self::UpsideDownAround,
	];

	#[ inline ]
	#[ must_use ]
	pub const fn idx (self) -> usize {
		match self {
			Self::None => 0,
			Self::Clockwise => 1,
			Self::CounterClockwise => 2,
			Self::UpsideDown => 3,
			Self::Up => 4,
			Self::ClockwiseUp => 5,
			Self::CounterClockwiseUp => 6,
			Self::UpsideDownUp => 7,
			Self::Down => 8,
			Self::ClockwiseDown => 9,
			Self::CounterClockwiseDown => 10,
			Self::UpsideDownDown => 11,
			Self::Left => 12,
			Self::ClockwiseLeft => 13,
			Self::CounterClockwiseLeft => 14,
			Self::UpsideDownLeft => 15,
			Self::Right => 16,
			Self::ClockwiseRight => 17,
			Self::CounterClockwiseRight => 18,
			Self::UpsideDownRight => 19,
			Self::Around => 20,
			Self::ClockwiseAround => 21,
			Self::CounterClockwiseAround => 22,
			Self::UpsideDownAround => 23,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn apply (self, pos: Pos) -> Pos {
		match self {
			Self::None => Pos { x: pos.x, y: pos.y, z: pos.z },
			Self::Clockwise => Pos { x: - pos.y, y: pos.x, z: pos.z },
			Self::CounterClockwise => Pos { x: pos.y, y: - pos.x, z: pos.z },
			Self::UpsideDown => Pos { x: - pos.x, y: - pos.y, z: pos.z },
			Self::Up => Pos { x: pos.x, y: - pos.z, z: pos.y },
			Self::ClockwiseUp => Pos { x: pos.z, y: pos.x, z: pos.y },
			Self::CounterClockwiseUp => Pos { x: - pos.z, y: - pos.x, z: pos.y },
			Self::UpsideDownUp => Pos { x: - pos.x, y: pos.z, z: pos.y },
			Self::Down => Pos { x: pos.x, y: pos.z, z: - pos.y },
			Self::ClockwiseDown => Pos { x: - pos.z, y: pos.x, z: - pos.y },
			Self::CounterClockwiseDown => Pos { x: pos.z, y: - pos.x, z: - pos.y },
			Self::UpsideDownDown => Pos { x: - pos.x, y: - pos.z, z: - pos.y },
			Self::Left => Pos { x: pos.z, y: pos.y, z: - pos.x },
			Self::ClockwiseLeft => Pos { x: - pos.y, y: pos.z, z: - pos.x },
			Self::CounterClockwiseLeft => Pos { x: pos.y, y: - pos.z, z: - pos.x },
			Self::UpsideDownLeft => Pos { x: - pos.z, y: - pos.y, z: - pos.x },
			Self::Right => Pos { x: - pos.z, y: pos.y, z: pos.x },
			Self::ClockwiseRight => Pos { x: - pos.y, y: - pos.z, z: pos.x },
			Self::CounterClockwiseRight => Pos { x: pos.y, y: pos.z, z: pos.x },
			Self::UpsideDownRight => Pos { x: pos.z, y: - pos.y, z: pos.x },
			Self::Around => Pos { x: - pos.x, y: pos.y, z: - pos.z },
			Self::ClockwiseAround => Pos { x: - pos.y, y: - pos.x, z: - pos.z },
			Self::CounterClockwiseAround => Pos { x: pos.y, y: pos.x, z: - pos.z },
			Self::UpsideDownAround => Pos { x: pos.x, y: - pos.y, z: - pos.z },
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn left (self) -> Self {
		match self {
			Self::None => Self::Left,
			Self::Clockwise => Self::ClockwiseUp,
			Self::CounterClockwise => Self::CounterClockwiseDown,
			Self::UpsideDown => Self::UpsideDownRight,
			Self::Up => Self::CounterClockwiseLeft,
			Self::ClockwiseUp => Self::CounterClockwiseAround,
			Self::CounterClockwiseUp => Self::CounterClockwise,
			Self::UpsideDownUp => Self::CounterClockwiseRight,
			Self::Down => Self::ClockwiseLeft,
			Self::ClockwiseDown => Self::Clockwise,
			Self::CounterClockwiseDown => Self::ClockwiseAround,
			Self::UpsideDownDown => Self::ClockwiseRight,
			Self::Left => Self::Around,
			Self::ClockwiseLeft => Self::UpsideDownUp,
			Self::CounterClockwiseLeft => Self::UpsideDownDown,
			Self::UpsideDownLeft => Self::UpsideDown,
			Self::Right => Self::None,
			Self::ClockwiseRight => Self::Up,
			Self::CounterClockwiseRight => Self::Down,
			Self::UpsideDownRight => Self::UpsideDownAround,
			Self::Around => Self::Right,
			Self::ClockwiseAround => Self::CounterClockwiseUp,
			Self::CounterClockwiseAround => Self::ClockwiseDown,
			Self::UpsideDownAround => Self::UpsideDownLeft,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn around (self) -> Self {
		self.left ().left ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn right (self) -> Self {
		self.left ().left ().left ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn clockwise (self) -> Self {
		match self {
			Self::None => Self::Clockwise,
			Self::Clockwise => Self::UpsideDown,
			Self::CounterClockwise => Self::None,
			Self::UpsideDown => Self::CounterClockwise,
			Self::Up => Self::ClockwiseUp,
			Self::ClockwiseUp => Self::UpsideDownUp,
			Self::CounterClockwiseUp => Self::Up,
			Self::UpsideDownUp => Self::CounterClockwiseUp,
			Self::Down => Self::ClockwiseDown,
			Self::ClockwiseDown => Self::UpsideDownDown,
			Self::CounterClockwiseDown => Self::Down,
			Self::UpsideDownDown => Self::CounterClockwiseDown,
			Self::Left => Self::ClockwiseLeft,
			Self::ClockwiseLeft => Self::UpsideDownLeft,
			Self::CounterClockwiseLeft => Self::Left,
			Self::UpsideDownLeft => Self::CounterClockwiseLeft,
			Self::Right => Self::ClockwiseRight,
			Self::ClockwiseRight => Self::UpsideDownRight,
			Self::CounterClockwiseRight => Self::Right,
			Self::UpsideDownRight => Self::CounterClockwiseRight,
			Self::Around => Self::ClockwiseAround,
			Self::ClockwiseAround => Self::UpsideDownAround,
			Self::CounterClockwiseAround => Self::Around,
			Self::UpsideDownAround => Self::CounterClockwiseAround,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn upside_down (self) -> Self {
		self.clockwise ().clockwise ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn counter_clockwise (self) -> Self {
		self.clockwise ().clockwise ().clockwise ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn up (self) -> Self {
		match self {
			Self::None => Self::Up,
			Self::Clockwise => Self::ClockwiseRight,
			Self::CounterClockwise => Self::CounterClockwiseLeft,
			Self::UpsideDown => Self::UpsideDownDown,
			Self::Up => Self::UpsideDownAround,
			Self::ClockwiseUp => Self::UpsideDownRight,
			Self::CounterClockwiseUp => Self::UpsideDownLeft,
			Self::UpsideDownUp => Self::UpsideDown,
			Self::Down => Self::None,
			Self::ClockwiseDown => Self::Right,
			Self::CounterClockwiseDown => Self::Left,
			Self::UpsideDownDown => Self::Around,
			Self::Left => Self::ClockwiseUp,
			Self::ClockwiseLeft => Self::Clockwise,
			Self::CounterClockwiseLeft => Self::CounterClockwiseAround,
			Self::UpsideDownLeft => Self::ClockwiseDown,
			Self::Right => Self::CounterClockwiseUp,
			Self::ClockwiseRight => Self::ClockwiseAround,
			Self::CounterClockwiseRight => Self::CounterClockwise,
			Self::UpsideDownRight => Self::CounterClockwiseDown,
			Self::Around => Self::UpsideDownUp,
			Self::ClockwiseAround => Self::ClockwiseLeft,
			Self::CounterClockwiseAround => Self::CounterClockwiseRight,
			Self::UpsideDownAround => Self::Down,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn flip (self) -> Self {
		self.up ().up ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn down (self) -> Self {
		self.up ().up ().up ()
	}

	#[ inline ]
	#[ must_use ]
	pub const fn rev (self) -> Self {
		match self {
			Self::None => Self::None,
			Self::Clockwise => Self::CounterClockwise,
			Self::CounterClockwise => Self::Clockwise,
			Self::UpsideDown => Self::UpsideDown,
			Self::Up => Self::Down,
			Self::ClockwiseUp => Self::CounterClockwiseRight,
			Self::CounterClockwiseUp => Self::ClockwiseLeft,
			Self::UpsideDownUp => Self::UpsideDownUp,
			Self::Down => Self::Up,
			Self::ClockwiseDown => Self::CounterClockwiseLeft,
			Self::CounterClockwiseDown => Self::ClockwiseRight,
			Self::UpsideDownDown => Self::UpsideDownDown,
			Self::Left => Self::Right,
			Self::ClockwiseLeft => Self::CounterClockwiseUp,
			Self::CounterClockwiseLeft => Self::ClockwiseDown,
			Self::UpsideDownLeft => Self::UpsideDownLeft,
			Self::Right => Self::Left,
			Self::ClockwiseRight => Self::CounterClockwiseDown,
			Self::CounterClockwiseRight => Self::ClockwiseUp,
			Self::UpsideDownRight => Self::UpsideDownRight,
			Self::Around => Self::Around,
			Self::ClockwiseAround => Self::ClockwiseAround,
			Self::CounterClockwiseAround => Self::CounterClockwiseAround,
			Self::UpsideDownAround => Self::UpsideDownAround,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub const fn combine (self, other: Self) -> Self {
		match self {
			Self::None => other,
			Self::Clockwise => other.clockwise (),
			Self::CounterClockwise => other.counter_clockwise (),
			Self::UpsideDown => other.upside_down (),
			Self::Up => other.up (),
			Self::ClockwiseUp => other.up ().clockwise (),
			Self::CounterClockwiseUp => other.up ().counter_clockwise (),
			Self::UpsideDownUp => other.up ().upside_down (),
			Self::Down => other.down (),
			Self::ClockwiseDown => other.down ().clockwise (),
			Self::CounterClockwiseDown => other.down ().counter_clockwise (),
			Self::UpsideDownDown => other.down ().upside_down (),
			Self::Left => other.left (),
			Self::ClockwiseLeft => other.left ().clockwise (),
			Self::CounterClockwiseLeft => other.left ().counter_clockwise (),
			Self::UpsideDownLeft => other.left ().upside_down (),
			Self::Right => other.right (),
			Self::ClockwiseRight => other.right ().clockwise (),
			Self::CounterClockwiseRight => other.right ().counter_clockwise (),
			Self::UpsideDownRight => other.right ().upside_down (),
			Self::Around => other.around (),
			Self::ClockwiseAround => other.around ().clockwise (),
			Self::CounterClockwiseAround => other.around ().counter_clockwise (),
			Self::UpsideDownAround => other.around ().upside_down (),
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
				(1, 2, 3) | (2, 3, 1) | (3, 1, 2) => 1,
				(3, 2, 1) | (2, 1, 3) | (1, 3, 2) => -1,
				_ => panic! (),
			} * pos.x.signum () * pos.y.signum () * pos.z.signum ();
			0 < sign
		}
		for rotate in Rotation::ALL.iter ().copied () {
			let result = rotate.apply (base);
			assert! (right_handed (result));
			assert! (! seen.contains (& result), "Duplicated rotation {:?}", result);
			seen.insert (result);
			let rotate_left = rotate.left ();
			assert! (! seen_left.contains (& rotate_left),
				"Duplicated rotation left {rotate_left:?}");
			seen_left.insert (rotate_left);
			let rotate_up = rotate.up ();
			assert! (! seen_up.contains (& rotate_up), "Duplicated rotation up {rotate_up:?}");
			seen_up.insert (rotate_up);
			let rotate_clockwise = rotate.clockwise ();
			assert! (! seen_clockwise.contains (& rotate_clockwise),
				"Duplicated rotation clockwise {rotate_clockwise:?}");
			seen_clockwise.insert (rotate_clockwise);
			let rotate_four_lefts = rotate.left ().left ().left ().left ();
			assert_eq! (rotate, rotate_four_lefts,
				"Four lefts from {rotate:?} arrives at {rotate_four_lefts:?}");
			let rotate_four_ups = rotate.up ().up ().up ().up ();
			assert_eq! (rotate, rotate_four_ups,
				"Four ups from {rotate:?} arrives at {rotate_four_ups:?}");
			let rotate_four_clockwises = rotate.clockwise ().clockwise ().clockwise ().clockwise ();
			assert_eq! (rotate, rotate_four_clockwises,
				"Four clockwises from {rotate:?} arrives at {rotate_four_clockwises:?}");
			assert_eq! (rotate, rotate.up ().right ().down ().counter_clockwise ());
			assert_eq! (rotate, rotate.flip ().around ().upside_down ());
			let rotate_two_revs = rotate.rev ().rev ();
			assert_eq! (rotate, rotate_two_revs,
				"Two reverses of {rotate:?} arrives at {rotate_two_revs:?}");
			let pos_forward_rev = rotate.rev ().apply (rotate.apply (base));
			assert_eq! (base, pos_forward_rev,
				"Applying forward and reverse to {rotate:?} arrives at {pos_forward_rev:?}");
			for other in Rotation::ALL.iter ().copied () {
				let pos_apply_twice = other.apply (rotate.apply (base));
				let pos_combine = other.combine (rotate).apply (base);
				assert_eq! (pos_apply_twice, pos_combine,
					"Applying {rotate:?} then {other:?} gives {pos_apply_twice:?} but combining \
					then applying gives {pos_combine:?}");
				for other_base in Rotation::ALL.iter ().copied ().map (|rot| rot.apply (base)) {
					if other.apply (other_base) != result { continue }
					assert_eq! (other.rev ().combine (rotate).apply (base), other_base);
				}
			}
		}
	}

}
