use super::*;

pub use wire::Wire;
pub use wire_id::WireId;
pub use wire_input::WireInput;

pub type WireVal = u16;

mod wire {

	use super::*;

	#[ derive (Clone, Debug) ]
	pub struct Wire <'inp> {
		pub id: WireId <'inp>,
		pub input: WireInput <'inp>,
	}

	struct_parser_display! {
		input_lifetime = 'inp;
		Wire <'inp> { id, input } = [ input, " -> ", id ]
	}

}

mod wire_id {

	use super::*;

	wrapper_deref! {
		#[ derive (Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq) ]
		pub struct WireId <'inp> {
			id: InpStr <'inp>,
		}
	}

	struct_parser_display! {
		input_lifetime = 'inp;
		WireId <'inp> { id } = [ @str id = ('a' ..= 'z', 1 ..= 10) ]
	}

	impl TryFrom <& str> for WireId <'static> {
		type Error = GenError;
		fn try_from (src: & str) -> GenResult <Self> {
			if src.is_empty () {
				return Err ("Wire ID must have at least one character".into ());
			}
			if ! src.chars ().all (|ch| ch.is_ascii_lowercase ()) {
				return Err ("Wire ID must be all lowercase ASCII".into ());
			}
			Ok (WireId {
				id: InpStr::alloc (src),
			})
		}
	}

}

mod wire_input {

	use super::*;

	enum_decl_parser_display! {
		input_lifetime = 'inp;
		#[ derive (Clone, Debug) ]
		pub enum WireInput <'inp> {
			Not (arg: WireId <'inp>) = [ "NOT ", arg ],
			AndOne (arg: WireId <'inp>) = [ "1 AND ", arg ],
			And (arg_0: WireId <'inp>, arg_1: WireId <'inp>) = [ arg_0, " AND ", arg_1 ],
			Or (arg_0: WireId <'inp>, arg_1: WireId <'inp>) = [ arg_0, " OR ", arg_1 ],
			LeftShift (arg_0: WireId <'inp>, arg_1: WireVal) = [ arg_0, " LSHIFT ", arg_1 = 1 ..= 15 ],
			RightShift (arg_0: WireId <'inp>, arg_1: WireVal) = [ arg_0, " RSHIFT ", arg_1 = 1 ..= 15 ],
			Static (val: WireVal) = [ val ],
			Wire (arg: WireId <'inp>) = [ arg ],
		}
	}

	impl <'inp> WireInput <'inp> {

		#[ inline ]
		#[ must_use ]
		pub fn inputs (& self) -> ArrayVec <& WireId <'inp>, 2> {
			match * self {
				Self::Static (_) => array_vec! [],
				Self::Wire (ref arg) => array_vec! [ arg ],
				Self::Not (ref arg) => array_vec! [ arg ],
				Self::And (ref arg_0, ref arg_1) => array_vec! [ arg_0, arg_1 ],
				Self::AndOne (ref arg) => array_vec! [ arg ],
				Self::Or (ref arg_0, ref arg_1) => array_vec! [ arg_0, arg_1 ],
				Self::LeftShift (ref arg_0, _) => array_vec! [ arg_0 ],
				Self::RightShift (ref arg_0, _) => array_vec! [ arg_0 ],
			}
		}

	}

}
