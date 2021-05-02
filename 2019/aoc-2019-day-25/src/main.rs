use intcode::Machine;
use intcode::Mem;
use intcode::RunResult;
use rustyline::Editor;
use std::char;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;

mod intcode;

fn main () {
	let args: Vec <String> = env::args ().collect ();
	match args.get (1).map (String::as_ref) {
		Some ("inter") => main_interactive (),
		Some ("auto") => main_automated (),
		_ => {
			println! ("Syntax:");
			println! ("  {} inter   Run as interactive game", args [0]);
			println! ("  {} auto    Run using automation", args [0]);
		},
	}
}

fn main_automated () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_programme = intcode::from_str (& input_string);
	let mut engine = GameEngine::new (input_programme.clone ());
	engine.run ();
}

type RcStr = Rc <str>;

struct GameEngine {
	machine: Machine,
	state: GameState,
	rooms: HashMap <RcStr, GameRoom>,
	items: HashMap <RcStr, GameItem>,
	unexplored: Vec <(RcStr, RcStr)>,
	uncollected: Vec <RcStr>,
	unprobed: Vec <(RcStr, RcStr)>,
	current_room: RcStr,
	current_routes: Option <HashMap <RcStr, (RcStr, RcStr)>>,
	previous_room: RcStr,
	previous_door: RcStr,
	next_weight: u32,
}

#[ derive (Debug) ]
struct GameRoom {
	name: RcStr,
	description: RcStr,
	items: Vec <RcStr>,
	doors: HashMap <RcStr, GameDoor>,
}

#[ derive (Debug) ]
struct GameDoor {
    name: RcStr,
    from_room: RcStr,
    to_room: RcStr,
    is_secure: bool,
}

struct GameItem {
	name: RcStr,
	found_room: RcStr,
	in_room: RcStr,
	banned: bool,
	weight: u32,
}

#[ derive (Eq, PartialEq) ]
enum GameState {
	Init,
	Halted,
	Explore,
	Exploring (Vec <(RcStr, RcStr)>),
	Collect,
	Collecting (RcStr, Vec <(RcStr, RcStr)>),
	Probe,
	Probing (Vec <(RcStr, RcStr)>, RcStr, u32),
	Poison,
}

impl GameEngine {

	fn new (programme: Mem) -> GameEngine {
		let machine = Machine::new (programme);
		GameEngine {
			machine,
			state: GameState::Init,
			rooms: HashMap::new (),
			items: HashMap::new (),
			unexplored: Vec::new (),
			uncollected: Vec::new (),
			unprobed: Vec::new (),
			current_room: "".into (),
			current_routes: None,
			previous_room: "".into (),
			previous_door: "".into (),
			next_weight: 1,
		}
	}

	fn run (& mut self) {
		'OUTER: loop {
			let mut state_temp = GameState::Poison;
			mem::swap (& mut state_temp, & mut self.state);
			match state_temp {
				GameState::Init => {
					self.handle_output_room ();
					self.state = GameState::Explore;
					println! ("Start exploring");
				},
				GameState::Explore => {
					for unexp_idx in 0 .. self.unexplored.len () {
						let (unexp_room_name, unexp_door) = & self.unexplored [unexp_idx];
						let unexp_room_name = unexp_room_name.clone ();
						let unexp_door = unexp_door.clone ();
						if ! self.can_reach (& unexp_room_name) { continue }
						let mut route = self.find_route (& unexp_room_name);
						route.push ((unexp_room_name.clone (), unexp_door.clone ()));
						self.state = GameState::Exploring (route);
						self.unexplored.remove (unexp_idx);
						continue 'OUTER;
					}
					if ! self.unexplored.is_empty () {
						println! ("Still need to explore: {:?}", self.unexplored);
						println! ("Routes: {:?}", self.current_routes.as_ref ().unwrap ());
						panic! ();
					}
					self.state = GameState::Collect;
				},
				GameState::Exploring (mut route) => {
					if route.is_empty () {
						self.state = GameState::Explore;
						continue 'OUTER;
					}
					let (step_room, step_door) = route.remove (0);
					if step_room != self.current_room { panic! () }
					self.follow_door (& step_door);
					self.state = GameState::Exploring (route);
				},
				GameState::Collect => {
					if self.uncollected.is_empty () {
						self.state = GameState::Probe;
						continue 'OUTER;
					}
					let item_name = self.uncollected.pop ().unwrap ();
					let item_room = self.items [& item_name].found_room.clone ();
					let route = self.find_route (& item_room);
					self.state = GameState::Collecting (item_name.clone (), route);
				},
				GameState::Collecting (item_name, mut item_route) => {
					if ! item_route.is_empty () {
						let (from_room, via_door) = item_route.remove (0);
						if self.current_room != from_room { panic! () }
						self.follow_door (& via_door);
						self.state = GameState::Collecting (item_name, item_route);
					} else {
						self.take_item (& item_name);
						self.state = GameState::Collect;
					}
				},
				GameState::Probe => {
					if self.unprobed.is_empty () {
						todo! ();
					}
					let (probe_room, probe_door) = self.unprobed.remove (0);
					let probe_route = self.find_route (& probe_room);
					self.state = GameState::Probing (probe_route, probe_door, 0);
				},
				GameState::Probing (mut probe_route, probe_door, probe_weight) => {
					if ! probe_route.is_empty () {
						let (route_room, route_door) = probe_route.remove (0);
						if self.current_room != route_room { panic! (); }
						self.follow_door (& route_door);
						self.state = GameState::Probing (probe_route, probe_door, probe_weight);
						continue 'OUTER;
					}
					let item_names: Vec <RcStr> =
						self.items.iter ().map (|(k, _)| k.clone ()).collect ();
					for item_name in item_names.into_iter () {
						let item = & self.items [& item_name];
						let have_item = item.in_room.len () == 0;
						let want_item = (item.weight & probe_weight) != 0;
						if have_item && ! want_item {
							self.drop_item (& item_name);
						}
						if ! have_item && want_item {
							self.take_item (& item_name);
						}
					}
					self.follow_door (& probe_door);
					if self.state == GameState::Halted { return }
					if probe_weight + 1 == self.next_weight { panic! () }
					self.state = GameState::Probing (Vec::new (), probe_door, probe_weight + 1);
					continue 'OUTER;
				},
				GameState::Halted => {
					todo! ();
				},
				GameState::Poison => {
					panic! ();
				},
			}
		}
	}

	fn resolve_routes (& mut self) {
		if self.current_routes.is_some () { return }
		let mut routes: HashMap <RcStr, (RcStr, RcStr)> = HashMap::new ();
		let mut current: Vec <RcStr> = Vec::new ();
		current.push (self.current_room.clone ());
		while ! current.is_empty () {
			let mut current_temp: Vec <RcStr> = Vec::new ();
			mem::swap (& mut current_temp, & mut current);
			for this_room in current_temp.into_iter () {
				for door in self.rooms [& this_room].doors.values () {
					if door.to_room.len () == 0 { continue }
					if routes.contains_key (& door.to_room) { continue }
					if door.to_room == self.current_room { continue }
					routes.insert (door.to_room.clone (), (this_room.clone (), door.name.clone ()));
					current.push (door.to_room.clone ());
				}
			}
		}
		self.current_routes = Some (routes);
	}

	fn can_reach (& mut self, destination: & RcStr) -> bool {
		if destination == & self.current_room { return true }
		self.resolve_routes ();
		let routes = self.current_routes.as_ref ().unwrap ();
		routes.contains_key (destination)
	}

	fn find_route (
		& mut self,
		destination: & RcStr,
	) -> Vec <(RcStr, RcStr)> {
		self.resolve_routes ();
		if destination == & self.current_room { return Vec::new () }
		let routes = self.current_routes.as_ref ().unwrap ();
		if ! routes.contains_key (destination) { panic! () }
		let mut result = Vec::new ();
		let mut destination = destination.clone ();
		while let Some ((from_room, from_door)) = routes.get (& destination) {
			result.push ((from_room.clone (), from_door.clone ()));
			destination = from_room.clone ();
		}
		result.reverse ();
		result
	}

	fn follow_door (& mut self, door_name: & RcStr) {
		println! ("Followed door to {}", door_name);
		self.previous_room = self.current_room.clone ();
		self.previous_door = door_name.clone ();
		self.machine.input_line (& door_name);
		self.handle_output_room ();
	}

	fn handle_output_room (& mut self) {
		let mut output_lines = self.get_output ();
		if self.state == GameState::Halted {
			println! ("Game halted");
			for line in output_lines {
				println! ("{}", line);
			}
			return;
		}
		'ROOM: loop {
			enum State { Start, Description, None, Paths, Items }
			let mut state = State::Start;
			let mut room_name: RcStr = "".into ();
			let mut room_description: RcStr = "".into ();
			let mut room_items: Vec <RcStr> = Vec::new ();
			let mut room_doors: HashMap <RcStr, GameDoor> = HashMap::new ();
  			for output_line_idx in 0 .. output_lines.len () {
				let output_line = & output_lines [output_line_idx];
				match state {
					State::Start => {
						if output_line == "" {
							continue;
						} else if output_line.starts_with ("== ")
								&& output_line.ends_with (" ==") {
							room_name = output_line [3 .. output_line.len () - 3].into ();
							state = State::Description;
						}
					},
					State::Description => {
						room_description = output_line.as_str ().into ();
						state = State::None;
					},
					State::None => {
						if output_line == "" {
							continue;
						} else if output_line == "Doors here lead:" {
							state = State::Paths;
						} else if output_line == "Items here:" {
							state = State::Items;
						} else if output_line == "A loud, robotic voice says \"Alert! Droids on \
									this ship are heavier than the detected value!\" and you are \
									ejected back to the checkpoint."
								|| output_line == "A loud, robotic voice says \"Alert! Droids on \
									this ship are lighter than the detected value!\" and you are \
									ejected back to the checkpoint." {
							let previous_room = self.rooms.get_mut (& self.previous_room).unwrap ();
							let previous_door = previous_room.doors.get_mut (& self.previous_door).unwrap ();
							previous_door.is_secure = true;
							self.unprobed.push ((self.previous_room.clone (), self.previous_door.clone ()));
							output_lines = output_lines [output_line_idx .. ].to_vec ();
							self.previous_room = "".into ();
							self.previous_door = "".into ();
							continue 'ROOM;
						} else {
							panic! ("Unexpected output: {}", output_line);
						}
					},
					State::Paths => {
						if output_line == "" {
							state = State::None;
						} else if output_line.starts_with ("- ") {
						    let door_name: RcStr = output_line [2 .. ].into ();
							room_doors.insert (door_name.clone (), GameDoor {
							    name: door_name.clone (),
							    from_room: room_name.clone (),
							    to_room: "".into (),
							    is_secure: false,
							});
						} else {
							panic! ();
						}
					},
					State::Items => {
						if output_line == "" {
							state = State::None;
						} else if output_line.starts_with ("- ") {
							room_items.push (output_line [2 .. ].into ());
						} else {
							panic! ();
						}
					},
				}
			}
			if self.previous_room.len () > 0 {
				let previous_room = self.rooms.get_mut (& self.previous_room).unwrap ();
				let previous_door = previous_room.doors.get_mut (& self.previous_door).unwrap ();
				previous_door.to_room = room_name.clone ();
				self.previous_room = "".into ();
				self.previous_door = "".into ();
			}
			if room_name.len () == 0 {
				panic! ("{:?}", output_lines);
			}
			self.current_room = room_name.clone ();
			self.current_routes = None;
			if self.rooms.contains_key (& room_name) { return }
			println! ("Found new room: {}", room_name);
			println! ("  {}", room_description);
			for room_item in room_items.iter () {
				println! ("  Item: {}", room_item);
				let banned = [
					"giant electromagnet",
					"infinite loop",
				].contains (& room_item.deref ());
				self.items.insert (room_item.clone (), GameItem {
					name: room_item.clone (),
					found_room: room_name.clone (),
					in_room: room_name.clone (),
					banned,
					weight: 0,
				});
				if ! banned {
					self.uncollected.push (room_item.clone ());
				}
			}
			for room_door in room_doors.values () {
				self.unexplored.push ((
					room_door.from_room.clone (),
					room_door.name.clone (),
				));
			}
			self.rooms.insert (room_name.clone (), GameRoom {
				name: room_name,
				description: room_description,
				items: room_items,
				doors: room_doors,
			});
			break;
		}
	}

	fn take_item (& mut self, item_name: & RcStr) {
		let checkpoint = self.machine.clone ();
		self.machine.input_line (& format! ("take {}", item_name));
		let output_lines = self.get_output ();
		if self.state == GameState::Halted {
			self.machine = checkpoint;
			println! ("Picking up {} failed, marking item as banned", item_name);
			let item = self.items.get_mut (item_name).unwrap ();
			item.banned = true;
			return;
		}
		let expected_line: RcStr = format! ("You take the {}.", item_name).into ();
		let mut taken = false;
		for output_line in output_lines {
			if output_line.is_empty () {
				// do nothing
			} else if output_line == expected_line.deref () {
				taken = true;
			} else {
				panic! (":: {}", output_line);
			}
		}
		if ! taken { panic! (); }
		let item = self.items.get_mut (item_name).unwrap ();
		if item.weight == 0 {
			item.weight = self.next_weight;
			self.next_weight <<= 1;
			println! ("Picked up {} (assigned pseudo-weight {})", item_name, item.weight);
		} else {
			println! ("Picked up {}", item_name);
		}
		item.in_room = "".into ();
	}

	fn drop_item (& mut self, item_name: & RcStr) {
		self.machine.input_line (& format! ("drop {}", item_name));
		let output_lines = self.get_output ();
		if self.state == GameState::Halted { panic! () }
		let expected_line: RcStr = format! ("You drop the {}.", item_name).into ();
		let mut dropped = false;
		for output_line in output_lines {
			if output_line.is_empty () {
				// do nothing
			} else if output_line == expected_line.deref () {
				dropped = true;
			} else {
				panic! (":: {}", output_line);
			}
		}
		if ! dropped { panic! (); }
		let item = self.items.get_mut (item_name).unwrap ();
		println! ("Dropped up {}", item_name);
		item.in_room = self.current_room.clone ();
	}

	fn get_output (& mut self) -> Vec <String> {
		let mut output_lines: Vec <String> = Vec::new ();
		let mut output_buffer = String::new ();
		loop {
			match self.machine.run () {
				RunResult::Output (ch) => {
					let ch = char::from_u32 (ch as u32).unwrap ();
					if ch == '\n' {
						if output_buffer == "Command?" {
							return output_lines;
						} else {
							output_lines.push (output_buffer);
						}
						output_buffer = String::new ();
					} else {
						output_buffer.push (ch);
					}
				},
				RunResult::Halt => {
					self.state = GameState::Halted;
					return output_lines;
				},
				_ => panic! (),
			}
		}
	}

}

fn main_interactive () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_programme = intcode::from_str (& input_string);
	let mut editor = Editor::<()>::new ();
	let mut machine = Machine::new (input_programme.clone ());
	let mut output_buffer = String::new ();
	loop {
		match machine.run () {
			RunResult::Output (ch) => {
				let ch = char::from_u32 (ch as u32).unwrap ();
				if ch == '\n' {
					if output_buffer == "Command?" {
						let line = editor.readline ("Command? ").unwrap ();
						machine.input_line (& line);
					} else {
						println! ("{}", output_buffer);
					}
					output_buffer = String::new ();
				} else {
					output_buffer.push (ch);
				}
			},
			RunResult::Halt => return,
			_ => panic! (),
		}
	}
}
