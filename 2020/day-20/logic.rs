//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Dir;
use model::Grid;
use model::Pixel;
use model::Pos;
use model::Tag;
use model::Tile;
use model::Tiles;

pub fn part_one (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let tiles: Tiles =
		input.tiles.iter ()
			.map (|tile| (tile.id, Tile::new (tile.id, tile.grid.clone ())))
			.collect ();
	Ok (
		tiles.values ()
			.filter (|tile| get_shared_sides (& tiles, tile).count_ones () == 2)
			.map (|tile| tile.id.as_u64 ())
			.try_fold (1, |prod, val| chk! (prod * val)) ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {

	check_input (input) ?;

	let grid = assemble_image (input) ?;

	let (row_dir, col_dir, posns) =
		find_monsters (& grid).ok_or ("No monsters found") ?;

	let mut grid = grid.transform ([ row_dir, col_dir ]);
	remove_monsters (& mut grid, & posns);

	Ok (
		grid.values ()
			.filter (|& pixel| pixel == Pixel::White)
			.count ()
			.as_u32 ()
	)

}

const MONSTER: [u128; 3] = [
	0b_0000_0000_0000_0000_0010,
	0b_1000_0110_0001_1000_0111,
	0b_0100_1001_0010_0100_1000,
];

fn find_monsters (grid: & Grid) -> Option <(Dir, Dir, Vec <Pos>)> {
	let Pos { y: height, x: width } = grid.size ();
	[
		(Pos::ZERO, Dir::Down, Dir::Right, width),
		(Pos::ZERO, Dir::Right, Dir::Down, height),
		(Pos::new (0, grid.last_key ().x), Dir::Down, Dir::Left, width),
		(Pos::new (0, grid.last_key ().x), Dir::Left, Dir::Down, height),
		(Pos::new (grid.last_key ().y, 0), Dir::Up, Dir::Right, width),
		(Pos::new (grid.last_key ().y, 0), Dir::Right, Dir::Up, height),
		(grid.last_key (), Dir::Up, Dir::Left, width),
		(grid.last_key (), Dir::Left, Dir::Up, height),
	].into_iter ()
		.map (|(start, row_dir, col_dir, size)|
			(row_dir, col_dir, find_monsters_transform (grid, start, row_dir, col_dir, size)))
		.find (|& (_, _, ref posns)| ! posns.is_empty ())
}

fn find_monsters_transform (
	grid: & Grid,
	start: Pos,
	row_dir: Dir,
	col_dir: Dir,
	size: i8,
) -> Vec <Pos> {
	let row_off = grid.offset (row_dir);
	let col_off = grid.offset (col_dir);
	grid.cursor (start).unwrap ().walk (row_off)
		.map (|cur| cur.walk (col_off)
			.fold (0_u128, |sum, cur|
				sum << 1_u32 | u128::from (cur.item () == Pixel::White)))
		.tuple_windows ()
		.enumerate ()
		.flat_map (|(y, (mut a, mut b, mut c))| {
			let y = y.as_i8 ();
			let mut results = Vec::new ();
			for x in (0 .. size - 19).rev () {
				if detect_monster ([a, b, c]) {
					results.push (Pos::new (y, x));
				}
				a >>= 1_u32;
				b >>= 1_u32;
				c >>= 1_u32;
			}
			results
		})
		.collect ()
}

const fn detect_monster (rows: [u128; 3]) -> bool {
	if rows [0] & MONSTER [0] != MONSTER [0] { return false }
	if rows [1] & MONSTER [1] != MONSTER [1] { return false }
	if rows [2] & MONSTER [2] != MONSTER [2] { return false }
	true
}

fn remove_monsters (grid: & mut Grid, posns: & [Pos]) {
	for & pos in posns {
		let mut bit = 1 << 19_u32;
		for x in 0 .. 20 {
			for y in 0 .. 3 {
				if MONSTER [y.as_usize ()] & bit != 0 {
					grid.set (pos + Pos::new (y, x), Pixel::Black);
				}
			}
			bit >>= 1_u32;
		}
	}
}

fn assemble_image (input: & Input) -> GenResult <Grid> {
	let mut tiles: Tiles =
		input.tiles.iter ()
			.map (|tile| (tile.id, Tile::new (tile.id, tile.grid.clone ())))
			.collect ();
	let first = pick_first (& mut tiles) ?;
	let mut translate = Pos::ZERO;
	let mut result = first.grid.clone ();
	let mut prev_row = first.tags [2];
	let mut prev_col = first.tags [1];
	loop {
		loop {
			translate.x += first.grid.size ().x;
			let mut next = some_or! (pick_next (& mut tiles, 4, prev_col), break);
			paint_next (& mut result, & mut next, translate) ?;
			prev_col = next.tags [1];
		}
		translate.y += first.grid.size ().y;
		translate.x = 0;
		let mut next = some_or! (pick_next (& mut tiles, 7, prev_row), break);
		paint_next (& mut result, & mut next, translate) ?;
		prev_row = next.tags [2];
		prev_col = next.tags [1];
	}
	if ! tiles.is_empty () { return Err ("Failed to assemble all tiles".into ()) }
	Ok (result)
}

fn pick_next (tiles: & mut Tiles, tag_idx: u8, tag: Tag) -> Option <Tile> {
	let tile_id = tiles.values ().find (|tile| tile.tags.contains (& tag)) ?.id;
	let mut tile = tiles.remove (& tile_id).unwrap ();
	match (tag_idx, tile.tags.into_iter ().position (|other_tag| other_tag == tag).unwrap ()) {
		(4, 0) | (7, 3) => { tile.flip (); },
		(4, 1) | (7, 0) => { tile.flip (); tile.rotate_right (); },
		(4, 2) | (7, 1) => { tile.flip (); tile.rotate_around (); },
		(4, 3) | (7, 2) => { tile.flip (); tile.rotate_left (); },
		(4, 4) | (7, 7) => { },
		(4, 5) | (7, 4) => { tile.rotate_right (); },
		(4, 6) | (7, 5) => { tile.rotate_around (); },
		(4, 7) | (7, 6) => { tile.rotate_left (); },
		_ => unreachable! (),
	}
	if tile.tags [0 .. 4].contains (& tag) { tile.flip (); }
	while tile.tags [tag_idx.as_usize ()] != tag { tile.rotate_left (); }
	Some (tile)
}

fn paint_next (result: & mut Grid, tile: & mut Tile, translate: Pos) -> GenResult <()> {
	let required_size = chk! (translate + tile.grid.size ()) ?;
	if result.size ().y < required_size.y || result.size ().x < required_size.x {
		* result = result.resize ([0, 0], [
			cmp::max (result.native_size () [0], required_size.y.as_usize ()),
			cmp::max (result.native_size () [1], required_size.x.as_usize ()),
		]) ?;
	}
	for (pos, pixel) in tile.grid.translate (translate) ?.iter () {
		result.set (pos, pixel);
	}
	Ok (())
}

fn pick_first (tiles: & mut Tiles) -> GenResult <Tile> {
	let id = tiles.values ()
		.find (|tile| get_shared_sides (tiles, tile).count_ones () == 2)
		.map (|tile| tile.id)
		.ok_or ("No solution found") ?;
	let mut tile = tiles.remove (& id).unwrap ();
	match get_shared_sides (tiles, & tile) {
		0b_0110 => (),
		0b_0011 => tile.rotate_left (),
		0b_1001 => tile.rotate_around (),
		0b_1100 => tile.rotate_right (),
		_ => return Err ("No solution found".into ()),
	}
	Ok (tile)
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.tiles.len () < 2 { return Err ("Must provide at least two tiles".into ()) }
	if input.tiles.iter ().duplicates_by (|tile| tile.id).next ().is_some () {
		return Err ("Every tile must have a unique id".into ());
	}
	if ! input.tiles.iter ().map (|tile| tile.grid.size ()).all_equal () {
		return Err ("Tile sizes must all be the same size".into ());
	}
	let tile_size = input.tiles [0].grid.size ();
	if tile_size.y != tile_size.x {
		return Err ("Tile size must be a square".into ());
	}
	if tile_size.y < 3 || tile_size.x < 3 {
		return Err ("Tile size must be at least 3×3".into ());
	}
	if 32 < tile_size.y || 32 < tile_size.x {
		return Err ("Tile size must be at most 32×32".into ());
	}
	Ok (())
}

fn get_shared_sides (tiles: & Tiles, tile: & Tile) -> u8 {
	tile.tags.into_iter ().take (4)
		.map (|tag| tiles.values ()
			.filter (|other| tile.id != other.id)
			.any (|other| other.tags.contains (& tag)))
		.fold (0_u8, |sum, val| (sum << 1_u32) | u8::from (val))
}
