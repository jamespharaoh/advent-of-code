use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::mem;
use std::ops::Add;
use std::str;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.split ("\n").collect ();
	let num_steps = shortest_path (& input_lines);
	println! ("Number of steps: {}", num_steps);
}

fn shortest_path <LineRef: AsRef <str>> (lines: & [LineRef]) -> u64 {

	// collect array of chars and work out size

	let mut chars: Vec <char> = Vec::new ();
	let mut width: u32 = 0;
	let mut height: u32 = 0;
	for line in lines {
		let line = line.as_ref ();
		if line.trim ().len () == 0 { continue }
		height += 1;
		if width == 0 { width = line.len () as u32 }
		for ch in line.chars () { chars.push (ch) }
	}
	let size: Pos = (width, height).into ();

	// collect paths through normal walking and portals

	let mut portals_temp: HashMap <String, Pos> = HashMap::new ();
	let mut start: Option <Pos> = None;
	let mut end: Option <Pos> = None;
	let mut paths: HashMap <Pos, Vec <Pos>> = HashMap::new ();
	for x in 0 .. size.x {
		for y in 0 .. size.y {
			let pos: Pos = (x, y).into ();
			if chars [pos.to_index (size)] != '.' { continue }
			for dir in [ Dir::Up, Dir::Down, Dir::Left, Dir::Right ].iter ().cloned () {
				let adj_0 = chars [(pos + dir).to_index (size)];
				let adj_1 = chars [(pos + dir + dir).to_index (size)];
				if adj_0 == '.' {
					paths.entry (pos).or_insert (Vec::new ()).push (pos + dir);
				} else if adj_0.is_ascii_uppercase () {
					let portal_name = if dir.read_backwards () {
						format! ("{}{}", adj_1, adj_0)
					} else {
						format! ("{}{}", adj_0, adj_1)
					};
					if portal_name == "AA" {
						start = Some (pos);
					} else if portal_name == "ZZ" {
						end = Some (pos);
					} else {
						if let Some (other_pos) = portals_temp.remove (& portal_name) {
							paths.entry (pos).or_insert (Vec::new ()).push (other_pos);
							paths.entry (other_pos).or_insert (Vec::new ()).push (pos);
						} else {
							portals_temp.insert (portal_name, pos);
						}
					}
				} else if adj_0 == '#' || adj_0 == ' ' {
					// do nothing
				} else {
					panic! ("Invalid char: {}", adj_0);
				}
			}
		}
	}
	if ! portals_temp.is_empty () { panic! () }
	let start = start.unwrap ();
	let end = end.unwrap ();

	// walk the maze

	let mut visited: HashSet <Pos> = HashSet::new ();
	visited.insert (start);
	let mut current: Vec <Pos> = Vec::new ();
	current.push (start);
	let mut steps: u64 = 0;
	loop {
		steps += 1;
		let mut current_temp: Vec <Pos> = Vec::new ();
		mem::swap (& mut current, & mut current_temp);
		for pos in current_temp.into_iter () {
			for next_pos in paths [& pos].iter ().cloned () {
				if visited.contains (& next_pos) { continue }
				visited.insert (next_pos);
				if next_pos == end { return steps }
				current.push (next_pos);
			}
		}
	}

}

#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
enum Dir { Up, Down, Left, Right }

impl Dir {
	fn read_backwards (& self) -> bool {
		match self {
			Dir::Up => true,
			Dir::Down => false,
			Dir::Left => true,
			Dir::Right => false,
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Pos { x: u32, y: u32 }

impl Pos {
	fn to_index (& self, size: Pos) -> usize {
		self.y as usize * size.x as usize + self.x as usize
	}
}

impl From <(u32, u32)> for Pos {
	fn from ((x, y): (u32, u32)) -> Pos {
		Pos { x, y }
	}
}

impl Add <Dir> for Pos {
	type Output = Pos;
	fn add (self, dir: Dir) -> Pos {
		match dir {
			Dir::Up => Pos { x: self.x, y: self.y - 1 },
			Dir::Down => Pos { x: self.x, y: self.y + 1 },
			Dir::Left => Pos { x: self.x - 1, y: self.y },
			Dir::Right => Pos { x: self.x + 1, y: self.y },
		}
	}
}

#[ test ]
fn test_0 () {
	assert_eq! (23, shortest_path (& vec! [
		"         A           ",
		"         A           ",
		"  #######.#########  ",
		"  #######.........#  ",
		"  #######.#######.#  ",
		"  #######.#######.#  ",
		"  #######.#######.#  ",
		"  #####  B    ###.#  ",
		"BC...##  C    ###.#  ",
		"  ##.##       ###.#  ",
		"  ##...DE  F  ###.#  ",
		"  #####    G  ###.#  ",
		"  #########.#####.#  ",
		"DE..#######...###.#  ",
		"  #.#########.###.#  ",
		"FG..#########.....#  ",
		"  ###########.#####  ",
		"             Z       ",
		"             Z       ",
	]));
}

#[ test ]
fn test_1 () {
	assert_eq! (58, shortest_path (& vec! [
		"                   A               ",
		"                   A               ",
		"  #################.#############  ",
		"  #.#...#...................#.#.#  ",
		"  #.#.#.###.###.###.#########.#.#  ",
		"  #.#.#.......#...#.....#.#.#...#  ",
		"  #.#########.###.#####.#.#.###.#  ",
		"  #.............#.#.....#.......#  ",
		"  ###.###########.###.#####.#.#.#  ",
		"  #.....#        A   C    #.#.#.#  ",
		"  #######        S   P    #####.#  ",
		"  #.#...#                 #......VT",
		"  #.#.#.#                 #.#####  ",
		"  #...#.#               YN....#.#  ",
		"  #.###.#                 #####.#  ",
		"DI....#.#                 #.....#  ",
		"  #####.#                 #.###.#  ",
		"ZZ......#               QG....#..AS",
		"  ###.###                 #######  ",
		"JO..#.#.#                 #.....#  ",
		"  #.#.#.#                 ###.#.#  ",
		"  #...#..DI             BU....#..LF",
		"  #####.#                 #.#####  ",
		"YN......#               VT..#....QG",
		"  #.###.#                 #.###.#  ",
		"  #.#...#                 #.....#  ",
		"  ###.###    J L     J    #.#.###  ",
		"  #.....#    O F     P    #.#...#  ",
		"  #.###.#####.#.#####.#####.###.#  ",
		"  #...#.#.#...#.....#.....#.#...#  ",
		"  #.#####.###.###.#.#.#########.#  ",
		"  #...#.#.....#...#.#.#.#.....#.#  ",
		"  #.###.#####.###.###.#.#.#######  ",
		"  #.#.........#...#.............#  ",
		"  #########.###.###.#############  ",
		"           B   J   C               ",
		"           U   P   P               ",
	]));
}
