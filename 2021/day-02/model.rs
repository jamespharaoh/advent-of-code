use super::*;

pub type Val = u16;

#[ derive (Clone, Copy, Debug) ]
pub enum Command {
	Down (Val),
	Up (Val),
	Forward (Val),
}

enum_parser_display! {
	Command,
	Forward (val) = [ "forward ", val ],
	Down (val) = [ "down ", val ],
	Up (val) = [ "up ", val ],
}
