use std::collections::HashSet;
use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

type PointSet = HashSet <(i64, i64)>;

fn main () {
	let paths = load_paths ();
	let points_0 = line_points (& paths [0]);
	let points_1 = line_points (& paths [1]);
	let nearest = nearest_crossover (& points_0, & points_1);
	println! ("Nearest crossover: {}", nearest);
}

fn nearest_crossover (points_0: & PointSet, points_1: & PointSet) -> i64 {
	HashSet::intersection (points_0, points_1).map (
		|point| point.0.abs () + point.1.abs (),
	).min ().unwrap ()
}

fn line_points (path: & str) -> PointSet {
	let mut result = HashSet::new ();
	let mut pos = (0, 0);
	for part in path.split (',') {
		let mut part_chars = part.chars ();
		let direction = part_chars.next ().unwrap ();
		let count: u64 = part_chars.as_str ().parse ().unwrap ();
		for _ in 0 .. count {
			match direction {
				'L' => pos.0 -= 1,
				'R' => pos.0 += 1,
				'D' => pos.1 -= 1,
				'U' => pos.1 += 1,
				_ => panic! ("Invalid direction: {}", direction),
			}
			result.insert (pos);
		}
	}
	result
}

fn load_paths () -> Vec <String> {
	let file = File::open ("input").unwrap ();
	let reader = BufReader::new (file);
	reader.lines ().collect::<Result <Vec <String>, _>> ().unwrap ()
}
