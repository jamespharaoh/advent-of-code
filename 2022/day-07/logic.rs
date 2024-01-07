use super::*;

use input::Input;
use input::InputLine;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let root = build_dir_tree (input) ?;
	let mut todo = vec! [ root ];
	let mut total = 0;
	while let Some (dir) = todo.pop () {
		if dir.size <= 100_000 { chk! (total += dir.size) ?; }
		todo.extend (
			dir.entries.iter ()
				.filter_map (|entry| match * entry {
					DirEntry::Dir (ref dir) => Some (Rc::clone (dir)),
					DirEntry::File (_) => None,
				}));
	}
	Ok (total)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let root = build_dir_tree (input) ?;
	let space_needed = chk! (root.size - 40_000_000) ?;
	let mut todo = vec! [ root ];
	let mut smallest = None;
	while let Some (dir) = todo.pop () {
		if space_needed <= dir.size
				&& smallest.map_or (true, |smallest| dir.size < smallest) {
			smallest = Some (dir.size);
		}
		todo.extend (
			dir.entries.iter ()
				.filter_map (|entry| match * entry {
					DirEntry::Dir (ref dir) => Some (Rc::clone (dir)),
					DirEntry::File (_) => None,
				}));
	}
	Ok (smallest.ok_or ("No solution found") ?)
}

fn build_dir_tree (input: & Input) -> GenResult <Rc <Dir>> {
	struct Raw <'inp> {
		children: HashMap <Vec <InpStr <'inp>>, Vec <InpStr <'inp>>>,
		file_sizes: HashMap <Vec <InpStr <'inp>>, u32>,
	}
	let mut raw = Raw {
		children: HashMap::new (),
		file_sizes: HashMap::new (),
	};
	let mut current_dir: Vec <InpStr> = Vec::new ();
	for line in & input.lines {
		match * line {
			InputLine::CdCommand { ref dest } if dest == "/" => {
				current_dir.clear ();
			},
			InputLine::CdCommand { ref dest } if dest == ".." => {
				current_dir.pop ().ok_or ("Tried to change directory to parent of root") ?;
			},
			InputLine::CdCommand { ref dest } => {
				if ! dest.chars ().all (|ch| ch.is_ascii_lowercase () || ch == '.') {
					return Err ("Invalid dest for cd command".into ());
				}
				current_dir.push (dest.clone ());
			},
			InputLine::LsCommand => {
				raw.children.entry (current_dir.clone ()).or_default ();
			},
			InputLine::DirEntry { ref name } => {
				let parent =
					raw.children.get_mut (& current_dir)
						.ok_or ("Unexpected dir entry") ?;
				if parent.contains (name) {
					return Err ("Duplicated entry".into ());
				}
				parent.push (name.clone ());
			},
			InputLine::FileEntry { size, ref name } => {
				let parent =
					raw.children.get_mut (& current_dir)
						.ok_or ("Unexpected dir entry") ?;
				if parent.contains (name) {
					return Err ("Duplicated entry".into ());
				}
				parent.push (name.clone ());
				let mut file_path = current_dir.clone ();
				file_path.push (name.clone ());
				raw.file_sizes.insert (file_path, size);
			},
		}
	}
	fn build_dir (raw: & Raw, path: & [InpStr]) -> GenResult <Rc <Dir>> {
		let mut entries: Vec <DirEntry> = Vec::new ();
		let mut size = 0;
		for child_name in raw.children.get (path).ok_or ("Incomplete tree") ? {
			let mut child_path = path.to_vec ();
			child_path.push (child_name.clone ());
			if let Some (& file_size) = raw.file_sizes.get (& child_path) {
				entries.push (DirEntry::File (file_size));
				size += file_size;
			} else {
				let dir = build_dir (raw, & child_path) ?;
				size += dir.size;
				entries.push (DirEntry::Dir (dir));
			}
		}
		Ok (Rc::new (Dir { entries, size }))
	}
	build_dir (& raw, & [])
}

#[ derive (Debug) ]
struct Dir {
	entries: Vec <DirEntry>,
	size: u32,
}

#[ derive (Debug) ]
enum DirEntry {
	Dir (Rc <Dir>),
	#[ allow (dead_code) ]
	File (u32),
}
