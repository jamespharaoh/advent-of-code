use super::*;

pub type Val = i32;

enum_decl_parser_display! {
	input_lifetime = 'inp;
	#[ derive (Clone, Debug) ]
	pub enum Json <'inp> {
		Array (items: Vec <Json <'inp>>) = [
			"[", @confirm, @delim "," items, "]",
		],
		Object (items: Vec <(InpStr <'inp>, Json <'inp>)>) = [
			"{", @confirm, @delim "," items {
				(key, value) = [
					"\"", @str key = (|ch| { ch != '"' }, .. ), "\":", value,
				],
			}, "}",
		],
		Number (value: Val) = [ value ],
		String (value: InpStr <'inp>) = [
			"\"", @confirm, @str value = (|ch| { ch != '"' }, .. ), "\"",
		],
	}
}
