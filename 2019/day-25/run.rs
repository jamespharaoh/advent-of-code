#![ allow (clippy::print_stdout) ]

use super::*;

use std::path::PathBuf;

use game::Game;
use input::Input;
use logic::Explorer;
use logic::Room;
use model::Cpu;
use model::Door;
use model::RcStr;
use model::RunResult;
use model::Val;

#[ derive (clap::Parser) ]
pub struct RunArgs {

	#[ clap (from_global, value_parser = PathBuf) ]
	input: PathBuf,

}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::wildcard_enum_match_arm) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_string = fs::read_to_string (& args.input) ?;
	let input_lines: Vec <& str> = input_string.trim_end ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	let mut cpu = Cpu::new (input.data.clone ());
	let mut input_buf = String::new ();
	let mut output_buf = String::new ();
	loop {
		match cpu.run () {
			RunResult::Input => {
				input_buf.clear ();
				io::stdin ().read_line (& mut input_buf) ?;
				match input_buf.as_str ().trim () {
					"explore" => explore (input.data.clone ()) ?,
					_ => cpu.input_str (& input_buf),
				}
			},
			RunResult::Output (val) => {
				let ch = val.as_char ();
				if ch == '\n' {
					println! ("{output_buf}");
					output_buf.clear ();
				} else {
					output_buf.push (ch);
				}
			},
			RunResult::Halt => break,
			other => return Err (other.into ()),
		}
	}
	Ok (())
}

fn explore (prog: Vec <Val>) -> GenResult <()> {
	let mut game = Game::new (prog);
	let explorer = Explorer::new (& mut game) ?;
	let mut posns: HashMap <RcStr, (i8, i8)> = HashMap::new ();
	let mut todo = Vec::new ();
	todo.push (((0, 0), Rc::from ("Hull Breach")));
	let mut seen = HashSet::new ();
	seen.insert (Rc::from ("Hull Breach"));
	while let Some (((n, e), room_name)) = todo.pop () {
		posns.insert (Rc::clone (& room_name), (n, e));
		for & (door, ref dest) in & explorer.rooms [& room_name].doors {
			let dest = some_or! (dest, continue);
			if ! seen.insert (Rc::clone (dest)) { continue }
			let (n, e) = match door {
				Door::North => (n + 1, e),
				Door::South => (n - 1, e),
				Door::East => (n, e + 1),
				Door::West => (n, e - 1),
			};
			todo.push (((n, e), Rc::clone (dest)));
		}
	}
	let rooms: Vec <& Room> =
		explorer.rooms.values ()
			.sorted_by_key (|& room| & room.name)
			.collect ();
	for room in rooms {
		println! ("Room: {}", room.name);
		println! ("  {}", room.descrip);
	}
	Ok (())
}
