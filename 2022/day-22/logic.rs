use super::*;

use input::Input;
use model::Coord;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Pos3;
use model::Step;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let grid = & input.grid;
	let (mut pos, _) = grid.iter ().find (|& (_, tile)| tile == Tile::Open).unwrap ();
	let mut dir = Dir::Right;
	for step in & input.path {
		match * step {
			Step::Forwards (num) => {
				'OUTER: for _ in 0 .. num {
					let mut next_pos = pos;
					loop {
						next_pos = chk! (next_pos + Pos::from (dir)) ?;
						if next_pos.x < 0 { next_pos.x = grid.end ().x - 1; }
						if grid.end ().x <= next_pos.x { next_pos.x = 0; }
						if next_pos.y < 0 { next_pos.y = grid.end ().y - 1; }
						if grid.end ().y <= next_pos.y { next_pos.y = 0; }
						match grid.get (next_pos).unwrap () {
							Tile::None => continue,
							Tile::Open => break,
							Tile::Wall => break 'OUTER,
						}
					}
					pos = next_pos;
				}
			},
			Step::Left => { dir = dir.left (); },
			Step::Right => { dir = dir.right (); },
		}
	}
	Ok (calc_result (pos, dir))
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let grid = & input.grid;
	let stepper = Stepper3d::build (grid) ?;
	let mut pos = stepper.start;
	for step in & input.path {
		match * step {
			Step::Forwards (num) => {
				for _ in 0 .. num {
					let next_pos = stepper.step (pos);
					if grid.get (next_pos.pos) == Some (Tile::Wall) { break }
					pos = next_pos;
				}
			},
			Step::Left => { pos.dir = pos.dir.left (); },
			Step::Right => { pos.dir = pos.dir.right (); },
		}
	}
	Ok (calc_result (pos.pos, pos.dir))
}

fn check_input (input: & Input) -> GenResult <()> {
	let grid = & input.grid;
	let side_len = calc_side_length (grid) ?;
	if ! grid.iter ().all (|(pos, tile)| {
		let square_pos = pos - pos % side_len;
		(tile == Tile::None) == (grid.get (square_pos).unwrap () == Tile::None)
	}) {
		return Err ("Grid is not arranged in squares".into ());
	}
	for y in grid.start ().y .. grid.end ().y {
		if ! (grid.start ().x .. grid.end ().x)
				.map (|x| Pos::new (y, x))
				.map (|pos| grid.get (pos).unwrap ())
				.any (|tile| tile != Tile::None) {
			return Err ("Must have at least one open or wall tile in each row".into ());
		}
	}
	for x in grid.start ().x .. grid.end ().x {
		if ! (grid.start ().y .. grid.end ().y)
				.map (|y| Pos::new (y, x))
				.map (|pos| grid.get (pos).unwrap ())
				.any (|tile| tile != Tile::None) {
			return Err ("Must have at least one open or wall tile in each column".into ());
		}
	}
	Ok (())
}

fn calc_result (pos: Pos, dir: Dir) -> u32 {
	let row = pos.y.pan_u32 () + 1;
	let col = pos.x.pan_u32 () + 1;
	let dir = match dir {
		Dir::Right => 0,
		Dir::Down => 1,
		Dir::Left => 2,
		Dir::Up => 3,
	};
	row * 1000 + col * 4 + dir
}

struct Stepper3d {
	side_len: Coord,
	axes_to_square: HashMap <(Pos3, Pos3), Pos>,
	start: StepperPos3d,
}

impl Stepper3d {

	fn build (grid: & Grid) -> GenResult <Self> {
		let num_filled = grid.values ().filter (|& tile| tile != Tile::None).count ();
		if num_filled % 6 != 0 { return Err ("Can't work out cube size".into ()) }
		let num_per_face = num_filled / 6;
		let side_len =
			(Coord::ZERO .. 255_i16)
				.find (|& val| chk! (val.pan_usize () * val.pan_usize ()).unwrap () == num_per_face)
				.ok_or ("Can't work out side length") ?;
		let mut todo: VecDeque <Pos> =
			grid.iter ()
				.filter (|& (pos, _)| pos.y % side_len == 0 && pos.x % side_len == 0)
				.filter (|& (_, tile)| tile == Tile::Open)
				.map (|(pos, _)| pos)
				.collect ();
		let mut square_to_axes = HashMap::new ();
		square_to_axes.insert (todo.pop_front ().unwrap (), (Pos3::new (0, -1, 0), Pos3::new (1, 0, 0)));
		'OUTER: while let Some (square) = todo.pop_front () {
			for (dir, other_square) in [
				(Dir::Left, square + Pos::new (side_len, 0)),
				(Dir::Right, square + Pos::new (- side_len, 0)),
				(Dir::Up, square + Pos::new (0, side_len)),
				(Dir::Down, square + Pos::new (0, - side_len)),
			] {
				let Some (& (other_axis_y, other_axis_x)) =
					square_to_axes.get (& other_square)
					else { continue };
				let (axis_x, axis_y) = axes_trans (dir, (other_axis_x, other_axis_y));
				square_to_axes.insert (square, (axis_y, axis_x));
				continue 'OUTER;
			}
			todo.push_back (square);
		}
		let axes_to_square: HashMap <_, _> =
			square_to_axes.iter ().map (|(& square, & axes)| (axes, square)).collect ();
		let start = {
			let (pos, _) = grid.iter ().find (|& (_, tile)| tile == Tile::Open).unwrap ();
			let axes = square_to_axes [& pos];
			let dir = Dir::Right;
			StepperPos3d { pos, axes, dir }
		};
		Ok (Self { side_len, axes_to_square, start })
	}

	fn square_for (& self, pos: Pos) -> Pos {
		Pos {
			x: pos.x - pos.x.rem_euclid (self.side_len),
			y: pos.y - pos.y.rem_euclid (self.side_len),
		}
	}

	fn step (& self, mut pos: StepperPos3d) -> StepperPos3d {
		let old_square = self.square_for (pos.pos);
		pos.pos = chk! (pos.pos + Pos::from (pos.dir)).unwrap ();
		let new_square = self.square_for (pos.pos);
		if new_square != old_square {
			pos.axes = axes_trans (pos.dir, pos.axes);
			let mut pos_temp = pos.pos - new_square;
			pos.pos = loop {
				if let Some (& new_square) = self.axes_to_square.get (& pos.axes) {
					break new_square + pos_temp;
				}
				pos.dir = pos.dir.left ();
				pos_temp = Pos::new (self.side_len - 1 - pos_temp.x, pos_temp.y);
				pos.axes = rotate_axes_right (pos.axes);
			};
		}
		pos
	}

}

#[ derive (Clone, Copy, Debug) ]
struct StepperPos3d {
	pos: Pos,
	axes: (Pos3, Pos3),
	dir: Dir,
}

fn axes_trans (dir: Dir, (axis_x, axis_y): (Pos3, Pos3)) -> (Pos3, Pos3) {
	match dir {
		Dir::Left => (rotate_3d (axis_x, - axis_y), axis_y),
		Dir::Right => (rotate_3d (axis_x, axis_y), axis_y),
		Dir::Up => (axis_x, rotate_3d (axis_y, - axis_x)),
		Dir::Down => (axis_x, rotate_3d (axis_y, axis_x)),
	}
}

fn rotate_3d (pos: Pos3, axis: Pos3) -> Pos3 {
	match axis {
		Pos3 { x: 1, y: 0, z: 0 } => Pos3 { x: pos.x, y: - pos.z, z: pos.y },
		Pos3 { x: -1, y: 0, z: 0 } => Pos3 { x: pos.x, y: pos.z, z: - pos.y },
		Pos3 { x: 0, y: 1, z: 0 } => Pos3 { x: pos.z, y: pos.y, z: - pos.x },
		Pos3 { x: 0, y: -1, z: 0 } => Pos3 { x: - pos.z, y: pos.y, z: pos.x },
		Pos3 { x: 0, y: 0, z: 1 } => Pos3 { x: - pos.y, y: pos.x, z: pos.z },
		Pos3 { x: 0, y: 0, z: -1 } => Pos3 { x: pos.y, y: - pos.x, z: pos.z },
		_ => unreachable! (),
	}
}

fn rotate_axes_right ((axis_x, axis_y): (Pos3, Pos3)) -> (Pos3, Pos3) {
	(- axis_y, axis_x)
}

fn calc_side_length (grid: & Grid) -> GenResult <Coord> {
	let num_filled = grid.values ().filter (|& tile| tile != Tile::None).count ();
	if num_filled % 6 != 0 { return Err ("Can't work out cube size".into ()) }
	let num_per_face = num_filled / 6;
	Ok (
		(Coord::ZERO .. 255_i16)
			.find (|& val| chk! (val.pan_usize () * val.pan_usize ()).unwrap () == num_per_face)
			.ok_or ("Can't work out side length") ?
	)
}
