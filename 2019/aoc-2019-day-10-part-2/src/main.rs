use gcd::Gcd;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::ops;

fn main () {
	let input_str = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <String> =
		input_str.trim ().split ('\n').map (str::to_owned).collect ();
	let order = find_order (input_lines, Vec2 { x: 14, y: 17 });
	for (index, pos) in order.into_iter ().enumerate () {
		println! ("{}: ({}, {})", index + 1, pos.x, pos.y);
	}
}

fn find_order (
	input_lines: Vec <String>,
	base_pos: Vec2,
) -> Vec <Vec2> {

	let size = Vec2 { x: input_lines [0].len () as i64, y: input_lines.len () as i64 };
	let grid: Vec <bool> = input_lines.iter ().map (
		|input_line| input_line.chars ().map (
			|input_ch| match input_ch {
				'#' => true,
				'.' => false,
				_ => panic! (),
			},
		),
	).flatten ().collect ();

	let mut targets_by_dir: HashMap <Dir, Vec <(i64, Vec2)>> = HashMap::new ();

	for target_idx in 0 .. grid.len () {
		if ! grid [target_idx] { continue }
		let target_pos = pos_vec (size, target_idx);
		if target_pos == base_pos { continue }
		let diff = target_pos - base_pos;
		let (dir, mag) = diff.dir_mag ();
		targets_by_dir.entry (dir).or_insert (Vec::new ()).push ((mag, target_pos));
	}

	let mut targets_ordered: Vec <(usize, Dir, Vec2)> = Vec::new ();

	for (dir, mut mag_targets) in targets_by_dir.into_iter () {
		mag_targets.sort_by_key (|(ref mag, _pos)| * mag);
		for (idx, (_mag, target)) in mag_targets.into_iter ().enumerate () {
			targets_ordered.push ((idx, dir, target));
		}
	}

	targets_ordered.sort_by_key (|(ref idx, ref dir, _pos)| (* idx, * dir));

	targets_ordered.into_iter ().map (|(_idx, _dir, ref pos)| * pos).collect ()

}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Vec2 { x: i64, y: i64 }

impl Vec2 {

	fn dir_mag (& self) -> (Dir, i64) {
		let gcd = Gcd::gcd (self.x.abs () as u64, self.y.abs () as u64) as i64;
		(Dir (Vec2 { x: self.x / gcd, y: self.y / gcd }), gcd)
	}

	fn group (& self) -> DirGroup {
		match (self.x.signum (), self.y.signum ()) {
			(-1, -1) => DirGroup::UpLeft,
			( 0, -1) => DirGroup::Up,
			( 1, -1) => DirGroup::UpRight,
			(-1,  0) => DirGroup::Left,
			( 0,  0) => DirGroup::Zero,
			( 1,  0) => DirGroup::Right,
			(-1,  1) => DirGroup::DownLeft,
			( 0,  1) => DirGroup::Down,
			( 1,  1) => DirGroup::DownRight,
			_ => unreachable! (),
		}
	}

}

impl ops::Sub for Vec2 {
	type Output = Vec2;
	fn sub (self, other: Vec2) -> Vec2 {
		Vec2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

fn pos_vec (size: Vec2, index: usize) -> Vec2 {
	Vec2 { x: (index as i64) % size.x, y: (index as i64) / size.x }
}

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
enum DirGroup { Zero, Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft }

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Dir (Vec2);

impl Dir {
	fn group (& self) -> DirGroup {
		self.0.group ()
	}
}

impl cmp::Ord for Dir {
	fn cmp (& self, other: & Dir) -> cmp::Ordering {
		if self.0 == other.0 { return cmp::Ordering::Equal };
		match cmp::Ord::cmp (& self.group (), & other.group ()) {
			cmp::Ordering::Less => cmp::Ordering::Less,
			cmp::Ordering::Greater => cmp::Ordering::Greater,
			cmp::Ordering::Equal => cmp::Ord::cmp (& (self.0.y * other.0.x), & (self.0.x * other.0.y)),
		}
	}
}

impl cmp::PartialOrd for Dir {
	fn partial_cmp (& self, other: & Dir) -> Option <cmp::Ordering> {
		Some (self.cmp (other))
	}
}

#[ test ]
pub fn test_0 () {
	let order = find_order (
		vec! [
			".#..##.###...#######", "##.############..##.",
			".#.######.########.#", ".###.#######.####.#.",
			"#####.##.#.##.###.##", "..#####..#.#########",
			"####################", "#.####....###.#.#.##",
			"##.#################", "#####.##.###..####..",
			"..######..##.#######", "####.##.####...##..#",
			".#####..#.######.###", "##...#.##########...",
			"#.##########.#######", ".####.#.###.###.#.##",
			"....##.##.###..#####", ".#.#.###########.###",
			"#.#.#.#####.####.###", "###.##.####.##.#..##",
		].into_iter ().map (str::to_string).collect (),
		Vec2 { x: 11, y: 13 },
	);
	assert_eq! (Vec2 { x: 11, y: 12 }, order [0]);
	assert_eq! (Vec2 { x: 12, y: 1 }, order [1]);
	assert_eq! (Vec2 { x: 12, y: 2 }, order [2]);
	assert_eq! (Vec2 { x: 12, y: 8 }, order [9]);
	assert_eq! (Vec2 { x: 16, y: 0 }, order [19]);
	assert_eq! (Vec2 { x: 16, y: 9 }, order [49]);
	assert_eq! (Vec2 { x: 10, y: 16 }, order [99]);
	assert_eq! (Vec2 { x: 9, y: 6 }, order [198]);
	assert_eq! (Vec2 { x: 8, y: 2 }, order [199]);
	assert_eq! (Vec2 { x: 10, y: 9 }, order [200]);
	assert_eq! (Vec2 { x: 11, y: 1 }, order [298]);
}
