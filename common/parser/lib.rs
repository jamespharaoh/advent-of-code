use aoc_inpstr::*;
use aoc_misc::prelude::*;
use aoc_nums as nums;

use nums::IntConv;

mod delim;
mod display;
mod enums;
mod from_parser;
mod parse;
mod parser;
mod structs;

pub use delim::*;
pub use display::*;
pub use from_parser::*;
pub use parser::*;

pub mod prelude {
	pub use crate::parser::Parser;
}

pub type ParseResult <Item> = Result <Item, ParseError>;
