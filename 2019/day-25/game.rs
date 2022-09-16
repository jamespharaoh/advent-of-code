use super::*;

use model::Cpu;
use model::Door;
use model::RcStr;
use model::RunResult;
use model::Val;

pub struct Game {
	cpu: Cpu,
}

impl Game {

	#[ must_use ]
	pub fn new (prog: Vec <Val>) -> Self {
		let cpu = Cpu::new (prog);
		Self { cpu }
	}

	pub fn read_output (& mut self) -> GenResult <GameOutputVec> {
		self.cpu.set_max_ops (20_000);
		let mut output_buffer = String::new ();
		let mut output_line = String::new ();
		loop {
			#[ allow (clippy::wildcard_enum_match_arm) ]
			match self.cpu.run () {
				RunResult::Input | RunResult::Halt => break,
				RunResult::Output (val) => {
					let ch = val.to_char ().map_err (|_err| format! ("Invalid output: {val}")) ?;
					output_buffer.push (ch);
					if ch == '\n' {
						output_line.clear ();
					} else {
						output_line.push (ch);
					}
				},
				other => return Err (other.into ()),
			}
		}
		let output_lines: Vec <& str> = output_buffer.split ('\n').collect ();
		match GameOutputVec::parse_from_lines (& output_lines) {
			Ok (output) => Ok (output),
			Err (err) => Err (err),
		}
	}

	pub fn command (& mut self, command: & str) {
		self.cpu.input_str (& format! ("{command}\n"));
	}

}

#[ derive (Clone, Debug) ]
pub struct GameOutputVec {
	outputs: Vec <GameOutput>,
}

impl Deref for GameOutputVec {
	type Target = [GameOutput];
	fn deref (& self) -> & [GameOutput] {
		& self.outputs
	}
}

struct_parser! {
	GameOutputVec { outputs } = [ @collect outputs ]
}

#[ derive (Clone, Debug) ]
pub enum GameOutput {
	Room (GameOutputRoom),
	Taken (RcStr),
	Dropped (RcStr),
	EjectedLighter,
	EjectedHeavier,
	Prompt,
	Solution (RcStr),
}

enum_parser! {
	GameOutput,
	Room (room) = [ room ],
	Taken (item) = [ "\nYou take the ", item = parse_item, ".\n\n" ],
	Dropped (item) = [ "\nYou drop the ", item = parse_item, ".\n\n" ],
	EjectedLighter = [ "A loud, robotic voice says \"Alert! Droids on this ship are lighter than the detected value!\" and you are ejected back to the checkpoint.\n" ],
	EjectedHeavier = [ "A loud, robotic voice says \"Alert! Droids on this ship are heavier than the detected value!\" and you are ejected back to the checkpoint.\n" ],
	Prompt = [ "Command?\n" ],
	Solution (code) = [
		"A loud, robotic voice says \"Analysis complete! You may proceed.\" and you enter the ",
		"cockpit.\n",
		"Santa notices your small droid, looks puzzled for a moment, realizes what has happened, ",
		"and radios your ship directly.\n",
		"\"Oh, hello! You should be able to get in by typing ",
		@str code = (|ch| { ch.is_ascii_digit () }, 1 .. ), " on the keypad at the main ",
		"airlock.\"\n",
	]
}

fn parse_item (parser: & mut Parser) -> ParseResult <RcStr> {
	let item = parser.take_rest_while (|ch| ch.is_ascii_alphanumeric () || ch == ' ', .. ) ?;
	Ok (Rc::from (& * item))
}

#[ derive (Clone, Debug) ]
pub struct GameOutputRoom {
	pub name: RcStr,
	pub descrip: RcStr,
	pub doors: Vec <Door>,
	pub items: Vec <RcStr>,
}

struct_parser! {
	GameOutputRoom { name, descrip, doors, items } = [
		"\n\n\n",
		name = parse_room_name,
		descrip, "\n",
		"\n",
		"Doors here lead:\n",
		@collect doors = |parser| {
			parse! (parser, "- ", door, "\n");
			Ok (door)
		},
		"\n",
		items = parse_room_items,
	]
}

fn parse_room_name (parser: & mut Parser) -> ParseResult <RcStr> {
	parse! (parser, "== ", rest: InpStr, "\n");
	let name = rest.borrowed ().strip_suffix (" ==").ok_or_else (|| parser.err ()) ?;
	Ok (RcStr::from (name))
}

fn parse_room_items (parser: & mut Parser) -> ParseResult <Vec <RcStr>> {
	if parser.expect ("Items here:\n").is_err () { return Ok (Vec::new ()) }
	let items = parser
		.repeat (|parser| {
			parse! (parser, "- ", item, "\n");
			Ok (item)
		})
		.collect ();
	parser.expect ("\n") ?;
	Ok (items)
}
