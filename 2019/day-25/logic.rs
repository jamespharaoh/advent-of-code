//! Logic for solving the puzzles

use super::*;

use game::Game;
use game::GameOutput;
use game::GameOutputRoom;
use input::Input;
use model::Door;
use model::RcStr;

pub fn part_one (input: & Input) -> GenResult <String> {
	let mut game = Game::new (input.data.clone ());
	let (items, door) = explore (& mut game) ?;
	try_combinations (& mut game, & items, door)
}

fn try_combinations (game: & mut Game, items: & [RcStr], door: Door) -> GenResult <String> {
	for item in items {
		game.command (& format! ("drop {item}"));
		game.read_output () ?;
	}
	let mut combo_idx = 1_usize;
	while combo_idx < (1 << items.len ()) {
		let combo_items: Vec <RcStr> =
			items.iter ().enumerate ()
				.filter (|& (item_idx, _)| combo_idx & (1 << item_idx) != 0)
				.map (|(_, item)| Rc::clone (item))
				.collect ();
		for item in & combo_items {
			game.command (& format! ("take {item}"));
			game.read_output () ?;
		}
		game.command (& door.to_string ());
		let outputs = game.read_output () ?;
		let lighter = match outputs.get (1) {
			Some (& GameOutput::Solution (ref code)) => return Ok (code.to_string ()),
			Some (& GameOutput::EjectedLighter) => true,
			Some (& GameOutput::EjectedHeavier) => false,
			Some (other) => return Err (format! ("Unexpected output: {other:?}").into ()),
			None => return Err ("No output".into ()),
		};
		for item in & combo_items {
			game.command (& format! ("drop {item}"));
			game.read_output () ?;
		}
		if lighter {
			combo_idx += ((combo_idx - 1) & ! combo_idx) + 1;
		} else {
			combo_idx += 1;
		}
	}
	Err ("No solution found".into ())
}

fn explore (game: & mut Game) -> GenResult <(Vec <RcStr>, Door)> {
	let explorer = Explorer::new (game) ?;
	let & (door, _) =
		explorer.rooms [& explorer.player].doors.iter ()
			.find (|&& (_, ref dest)| dest.is_none ())
			.unwrap ();
	Ok ((explorer.items, door))
}

pub struct Explorer {
	pub rooms: HashMap <RcStr, Room>,
	pub items: Vec <RcStr>,
	pub player: RcStr,
}

impl Explorer {
	pub fn new (game: & mut Game) -> GenResult <Self> {
		let outputs = game.read_output () ?;
		let output = match outputs.get (0) {
			Some (& GameOutput::Room (ref room)) => room,
			Some (other) => return Err (format! ("Unexpected output: {other:?}").into ()),
			None => return Err ("No output".into ()),
		};
		let mut explorer = Self {
			rooms: HashMap::new (),
			items: Vec::new (),
			player: Rc::clone (& output.name),
		};
		explorer.track_room (output);
		explorer.explore (game) ?;
		Ok (explorer)
	}
	fn explore (& mut self, game: & mut Game) -> GenResult <()> {
		let mut route = VecDeque::new ();
		loop {
			if route.is_empty () {
				for (path_route, path_dest) in self.iter_paths () {
					if path_dest.as_ref () == "Security Checkpoint" { continue }
					let & (unexplored_door, _) = some_or! (
						self.rooms [& path_dest].doors.iter ()
							.find (|&& (_, ref dest)| dest.is_none ()),
						continue);
					for path_door in path_route { route.push_back (path_door); }
					route.push_back (unexplored_door);
					break;
				}
			}
			if route.is_empty () {
				if self.player.as_ref () == "Security Checkpoint" { break }
				for (path_route, path_dest) in self.iter_paths () {
					if path_dest.as_ref () != "Security Checkpoint" { continue }
					for path_door in path_route { route.push_back (path_door); }
					break;
				}
				if route.is_empty () {
					return Err ("Didn't find Security Checkpoint".into ());
				}
			}
			let chosen_door = route.pop_front ().unwrap ();
			game.command (& chosen_door.to_string ());
			let outputs = game.read_output () ?;
			let output = match outputs.get (0) {
				Some (& GameOutput::Room (ref output)) => output,
				Some (other) => return Err (format! ("Unexpected output: {other:?}").into ()),
				None => return Err ("No output".into ()),
			};
			self.track_step (chosen_door, output) ?;
			for item in & output.items {
				if [
					"escape pod",
					"giant electromagnet",
					"infinite loop",
					"molten lava",
					"photons",
				].contains (& item.as_ref ()) {
					continue;
				}
				game.command (& format! ("take {item}"));
				game.read_output () ?;
				self.items.push (Rc::clone (item));
			}
		}
		Ok (())
	}
	fn track_room (& mut self, output: & GameOutputRoom) {
		if self.rooms.contains_key (& output.name) { return }
		let new_room = Room {
			name: Rc::clone (& output.name),
			descrip: Rc::clone (& output.descrip),
			doors: output.doors.iter ().map (|& door| (door, None)).collect (),
		};
		self.rooms.insert (Rc::clone (& new_room.name), new_room);
	}
	fn track_step (& mut self, through_door: Door, output: & GameOutputRoom) -> GenResult <()> {
		let from_room_name = Rc::clone (& self.player);
		self.track_room (output);
		self.player = Rc::clone (& output.name);
		let to_room_name = Rc::clone (& self.player);
		self.set_door_dest (& from_room_name, through_door, & to_room_name) ?;
		self.set_door_dest (& to_room_name, through_door.rev (), & from_room_name) ?;
		Ok (())
	}
	fn set_door_dest (& mut self, from: & RcStr, door: Door, to: & RcStr) -> GenResult <()> {
		let room = self.rooms.get_mut (from).unwrap ();
		let & mut (_, ref mut door_dest) =
			room.doors.iter_mut ().find (|&& mut (room_door, _)| room_door == door)
				.ok_or ("Inconsistent rooms/doors") ?;
		if let Some (ref val) = * door_dest {
			if val != to { return Err ("Inconsistent rooms/doors".into ()); }
		}
		* door_dest = Some (Rc::clone (to));
		Ok (())
	}
	fn iter_paths (& self) -> impl Iterator <Item = (Vec <Door>, RcStr)> + '_ {
		let mut seen = HashSet::new ();
		seen.insert (Rc::clone (& self.player));
		let mut todo = VecDeque::new ();
		todo.push_back ((Vec::new (), Rc::clone (& self.player)));
		iter::from_fn (move || {
			let (route, room_name) = some_or! (todo.pop_front (), return None);
			for & (door, ref dest_room) in & self.rooms [& room_name].doors {
				let dest_room = some_or! (dest_room.as_ref (), continue);
				if ! seen.insert (Rc::clone (dest_room)) { continue }
				let mut route = route.clone ();
				route.push (door);
				todo.push_back ((route, Rc::clone (dest_room)));
			}
			Some ((route, room_name))
		})
	}
}

#[ derive (Clone, Debug) ]
pub struct Room {
	pub name: RcStr,
	pub descrip: RcStr,
	pub doors: Vec <(Door, Option <RcStr>)>,
}
