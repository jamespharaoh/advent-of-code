use std::collections::HashMap;

type CoordVal = i64;

fn main () {

	let mut orbs: Vec <Orb> = vec! [
		(-9, -1, -1, 0, 0, 0).into (),
		(2, 9, 5, 0, 0, 0).into (),
		(10, 18, -12, 0, 0, 0).into (),
		(-6, 15, -7, 0, 0, 0).into (),
	];

	let ticks_to_repeat = repeat_count (& mut orbs);

	println! ("Total iterations until repetition: {}", ticks_to_repeat);

}

fn repeat_count (orbs: & mut Vec <Orb>) -> u64 {
	let x_info = repeat_info (& mut orbs.iter ().map (
		|orb| OrbCoord { position: orb.position.x, velocity: orb.velocity.x },
	).collect ());
	let y_info = repeat_info (& mut orbs.iter ().map (
		|orb| OrbCoord { position: orb.position.y, velocity: orb.velocity.y },
	).collect ());
	let z_info = repeat_info (& mut orbs.iter ().map (
		|orb| OrbCoord { position: orb.position.z, velocity: orb.velocity.z },
	).collect ());
	if x_info.0 != 0 || y_info.0 != 0 || z_info.0 != 0 { panic! () }
	let mut result: u64 = 1;
	result = num::integer::lcm (result, x_info.1);
	result = num::integer::lcm (result, y_info.1);
	result = num::integer::lcm (result, z_info.1);
	result
}

fn repeat_info (orbs: & mut Vec <OrbCoord>) -> (u64, u64) {
	let mut tick: u64 = 0;
	let mut history: HashMap <Vec <OrbCoord>, u64> = HashMap::new ();
	loop {
		if let Some (prev) = history.get (orbs) {
			return (* prev, tick);
		}
		history.insert (orbs.clone (), tick);
		for index_0 in 0 .. orbs.len () {
			for index_1 in 0 .. orbs.len () {
				if index_0 == index_1 { continue }
				let orb_0 = & orbs [index_0];
				let orb_1 = & orbs [index_1];
				let delta = (orb_1.position - orb_0.position).signum ();
				orbs [index_0].velocity += delta;
			};
		}
		for orb in orbs.iter_mut () {
			orb.position += orb.velocity;
		}
		tick += 1;
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Vec3 { x: CoordVal, y: CoordVal, z: CoordVal }

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Orb {
	position: Vec3,
	velocity: Vec3,
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct OrbCoord {
	position: CoordVal,
	velocity: CoordVal,
}

impl From <(CoordVal, CoordVal, CoordVal, CoordVal, CoordVal, CoordVal)> for Orb {
	fn from ((pos_x, pos_y, pos_z, vel_x, vel_y, vel_z): (CoordVal, CoordVal, CoordVal, CoordVal, CoordVal, CoordVal)) -> Orb {
		Orb {
			position: Vec3 { x: pos_x, y: pos_y, z: pos_z },
			velocity: Vec3 { x: vel_x, y: vel_y, z: vel_z },
		}
	}
}

#[ test ]
fn test_0 () {
	assert_eq! (2772, repeat_count (& mut vec! [
		(-1, 0, 2, 0, 0, 0).into (),
		(2, -10, -7, 0, 0, 0).into (),
		(4, -8, 8, 0, 0, 0).into (),
		(3, 5, -1, 0, 0, 0).into (),
	]));
}

#[ test ]
fn test_1 () {
	assert_eq! (4686774924, repeat_count (& mut vec! [
		(-8, -10, 0, 0, 0, 0).into (),
		(5, 5, 10, 0, 0, 0).into (),
		(2, -7, 3, 0, 0, 0).into (),
		(9, -8, -3, 0, 0, 0).into (),
	]));
}
