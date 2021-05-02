use gcd::Gcd;
use std::fs;

fn main () {
	let input_str = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <String> =
		input_str.trim ().split ('\n').map (str::to_owned).collect ();
	let (pos, num_visible) = find_best_base (input_lines);
	println! ("Best base: ({}, {})", pos.x, pos.y);
	println! ("Num visible: {}", num_visible);
}

fn find_best_base (
	input_lines: Vec <String>,
) -> (Vec2, usize) {

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

	let mut max_visible: usize = 0;
	let mut max_visible_idx: usize = 0;

	for base_idx in 0 .. grid.len () {
		if ! grid [base_idx] { continue }
		let base_pos = pos_vec (size, base_idx);

		let mut num_visible: usize = 0;

		'TARGET: for target_idx in 0 .. grid.len () {
			if ! grid [target_idx] { continue }
			if target_idx == base_idx { continue }
			let target_pos = pos_vec (size, target_idx);
			let target_diff = diff_vec (base_pos, target_pos);
			let (target_dir, target_mag) = dir_vec_mag (target_diff);

			for block_idx in 0 .. grid.len () {
				if ! grid [block_idx] { continue }
				if block_idx == base_idx || block_idx == target_idx { continue }
				let block_pos = pos_vec (size, block_idx);
				let block_diff = diff_vec (base_pos, block_pos);
				let (block_dir, block_mag) = dir_vec_mag (block_diff);

				if target_dir != block_dir { continue }
				if target_mag < block_mag { continue }
				continue 'TARGET;

			}

			num_visible += 1;

		}

		if num_visible > max_visible {
			max_visible = num_visible;
			max_visible_idx = base_idx;
		}

	}

	(pos_vec (size, max_visible_idx), max_visible)

}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
struct Vec2 { x: i64, y: i64 }

fn pos_vec (size: Vec2, index: usize) -> Vec2 {
	Vec2 { x: (index as i64) % size.x, y: (index as i64) / size.x }
}

fn diff_vec (source: Vec2, dest: Vec2) -> Vec2 {
	Vec2 { x: dest.x - source.x, y: dest.y - source.y }
}

fn dir_vec_mag (diff: Vec2) -> (Vec2, i64) {
	let gcd = Gcd::gcd (diff.x.abs () as u64, diff.y.abs () as u64) as i64;
	(Vec2 { x: diff.x / gcd, y: diff.y / gcd }, gcd)
}

#[ test ]
pub fn test_0 () {
	assert_eq! (
		(Vec2 { x: 5, y: 8 }, 33),
		find_best_base (vec! [
			"......#.#.", "#..#.#....", "..#######.", ".#.#.###..", ".#..#.....",
			"..#....#.#", "#..#....#.", ".##.#..###", "##...#..#.", ".#....####",
		].into_iter ().map (str::to_string).collect ()),
	);
}

#[ test ]
pub fn test_1 () {
	assert_eq! (
		(Vec2 { x: 1, y: 2 }, 35),
		find_best_base (vec! [
			"#.#...#.#.", ".###....#.", ".#....#...", "##.#.#.#.#", "....#.#.#.",
			".##..###.#", "..#...##..", "..##....##", "......#...", ".####.###.",
		].into_iter ().map (str::to_string).collect ()),
	);
}

#[ test ]
pub fn test_2 () {
	assert_eq! (
		(Vec2 { x: 6, y: 3 }, 41),
		find_best_base (vec! [
			".#..#..###", "####.###.#", "....###.#.", "..###.##.#", "##.##.#.#.",
			"....###..#", "..#.#..#.#", "#..#.#.###", ".##...##.#", ".....#.#..",
		].into_iter ().map (str::to_string).collect ()),
	);
}

#[ test ]
pub fn test_3 () {
	assert_eq! (
		(Vec2 { x: 11, y: 13 }, 210),
		find_best_base (vec! [
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
		].into_iter ().map (str::to_string).collect ()),
	);
}
