use super::*;

use model::Pos;
use model::Size;

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Input {
	pub nodes: Vec <Node>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { nodes, params } = [
		params,
		"root@ebhq-gridcenter# df -h\n",
		"Filesystem", @skip " ",
		"Size", @skip " ",
		"Used", @skip " ",
		"Avail", @skip " ",
		"Use%\n",
		@lines nodes,
	]
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub struct Node {
	pub pos: Pos,
	pub size: Size,
	pub used: Size,
	pub avail: Size,
	pub use_pc: u8,
}

struct_parser_display! {
	Node { pos: Pos { x, y }, size, used, avail, use_pc } = [
		"/dev/grid/node-x", x, "-y", y, @skip " ",
		size, "T", @skip " ",
		used, "T", @skip " ",
		avail, "T", @skip " ",
		use_pc, "%",
	]
}

input_params! {
	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct InputParams {
	}
}
