use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

type Point = (i64, i64);
type PointSet = HashSet <Point>;
type PointMap = HashMap <Point, u64>;

fn main () {
	let paths = load_paths ();
	let points_0 = line_points (& paths [0]);
	let points_1 = line_points (& paths [1]);
	let nearest = nearest_crossover (& points_0, & points_1);
	println! ("Nearest crossover: {}", nearest);
}

fn nearest_crossover (point_map_0: & PointMap, point_map_1: & PointMap) -> u64 {
	let point_set_0: PointSet = point_map_0.iter ().map (|(k, _)| k.clone ()).collect ();
	let point_set_1: PointSet = point_map_1.iter ().map (|(k, _)| k.clone ()).collect ();
	HashSet::intersection (& point_set_0, & point_set_1).map (
		|point| point_map_0 [point] + point_map_1 [point]
	).min ().unwrap ()
}

fn line_points (path: & str) -> PointMap {
	let mut result = HashMap::new ();
	let mut pos = (0, 0);
	let mut distance = 0;
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
			distance += 1;
			result.entry (pos).or_insert (distance);
		}
	}
	result
}

fn load_paths () -> Vec <String> {
	let file = File::open ("input").unwrap ();
	let reader = BufReader::new (file);
	reader.lines ().collect::<Result <Vec <String>, _>> ().unwrap ()
}
