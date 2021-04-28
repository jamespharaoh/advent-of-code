use std::ops::AddAssign;

fn main () {

	let starting_positions: Vec <Vec3> = vec! [
		Vec3 { x: -9, y: -1, z: -1 },
		Vec3 { x: 2, y: 9, z: 5 },
		Vec3 { x: 10, y: 18, z: -12 },
		Vec3 { x: -6, y: 15, z: -7 },
	];

	let mut orbs: Vec <Orb> = starting_positions.into_iter ().map (Orb::new).collect ();

	for _ in 0 .. 1000 {
		tick (& mut orbs);
	}

	println! ("Total energy after 1000 iterations: {}", total_energy (& orbs));

}

fn tick (orbs: & mut Vec <Orb>) {
	for index_0 in 0 .. orbs.len () {
		for index_1 in 0 .. orbs.len () {
			if index_0 == index_1 { continue }
			let orb_0 = & orbs [index_0];
			let orb_1 = & orbs [index_1];
			let velocity_delta = Vec3 {
				x: (orb_1.position.x - orb_0.position.x).signum (),
				y: (orb_1.position.y - orb_0.position.y).signum (),
				z: (orb_1.position.z - orb_0.position.z).signum (),
			};
			orbs [index_0].velocity += velocity_delta;
		}
	}
	for orb in orbs.iter_mut () {
		orb.position += orb.velocity;
	}
}

fn total_energy (orbs: & Vec <Orb>) -> i64 {
	orbs.iter ().map (Orb::total_energy).sum ()
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
struct Vec3 { x: i64, y: i64, z: i64 }

impl AddAssign for Vec3 {
	fn add_assign (& mut self, other: Vec3) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
struct Orb {
	position: Vec3,
	velocity: Vec3,
}

impl Orb {
	fn new (position: Vec3) -> Orb {
		Orb {
			position,
			velocity: Vec3 { x: 0, y: 0, z: 0 },
		}
	}
	fn potential_energy (& self) -> i64 {
		self.position.x.abs () + self.position.y.abs () + self.position.z.abs ()
	}
	fn kinetic_energy (& self) -> i64 {
		self.velocity.x.abs () + self.velocity.y.abs () + self.velocity.z.abs ()
	}
	fn total_energy (& self) -> i64 {
		self.potential_energy () * self.kinetic_energy ()
	}
}

impl From <(i64, i64, i64, i64, i64, i64)> for Orb {
	fn from ((pos_x, pos_y, pos_z, vel_x, vel_y, vel_z): (i64, i64, i64, i64, i64, i64)) -> Orb {
		Orb {
			position: Vec3 { x: pos_x, y: pos_y, z: pos_z },
			velocity: Vec3 { x: vel_x, y: vel_y, z: vel_z },
		}
	}
}

#[ test ]
fn test_0 () {
	let mut orbs: Vec <Orb> = vec! [
		Vec3 { x: -1, y: 0, z: 2 },
		Vec3 { x: 2, y: -10, z: -7 },
		Vec3 { x: 4, y: -8, z: 8 },
		Vec3 { x: 3, y: 5, z: -1 },
	].into_iter ().map (Orb::new).collect ();
	let mut ticks = 0;
	loop {
		let expected: Option <Vec <Orb>> = match ticks {
			1 => Some (vec! [
				(2, -1, 1, 3, -1, -1).into (),
				(3, -7, -4, 1, 3, 3).into (),
				(1, -7, 5, -3, 1, -3).into (),
				(2, 2, 0, -1, -3, 1).into (),
			]),
			10 => Some (vec! [
				(2, 1, -3, -3, -2, 1).into (),
				(1, -8, 0, -1, 1, 3).into (),
				(3, -6, 1, 3, 2, -3).into (),
				(2, 0, 4, 1, -1, -1).into (),
			]),
			_ => None,
		};
		if let Some (expected) = expected {
			assert_eq! (& expected, & orbs, "after {} ticks", ticks);
		}
		if ticks == 10 { break }
		tick (& mut orbs);
		ticks += 1;
	}
	assert_eq! (179, total_energy (& orbs));
}

#[ test ]
fn test_1 () {
	let mut orbs: Vec <Orb> = vec! [
		Vec3 { x: -8, y: -10, z: 0 },
		Vec3 { x: 5, y: 5, z: 10 },
		Vec3 { x: 2, y: -7, z: 3 },
		Vec3 { x: 9, y: -8, z: -3 },
	].into_iter ().map (Orb::new).collect ();
	let mut ticks = 0;
	loop {
		let expected: Option <Vec <Orb>> = match ticks {
			10 => Some (vec! [
				(-9, -10, 1, -2, -2, -1).into (),
				(4, 10, 9, -3, 7, -2).into (),
				(8, -10, -3, 5, -1, -2).into (),
				(5, -10, 3, 0, -4, 5).into (),
			]),
			100 => Some (vec! [
				(8, -12, -9, -7, 3, 0).into (),
				(13, 16, -3, 3, -11, -5).into (),
				(-29, -11, -1, -3, 7, 4).into (),
				(16, -13, 23, 7, 1, 1).into (),
			]),
			_ => None,
		};
		if let Some (expected) = expected {
			assert_eq! (& expected, & orbs, "after {} ticks", ticks);
		}
		if ticks == 100 { break }
		tick (& mut orbs);
		ticks += 1;
	}
	assert_eq! (1940, total_energy (& orbs));
}
