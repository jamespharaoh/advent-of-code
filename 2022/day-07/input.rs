use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub lines: Vec <InputLine <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { lines, params } = [ params, @lines lines ]
}

enum_decl_parser_display! {
	input_lifetime = 'inp;
	#[ derive (Clone, Debug) ]
	pub enum InputLine <'inp> {
		CdCommand { dest: InpStr <'inp> } = [
			"$ cd ",
			@str dest = (|ch| { ch.is_ascii_lowercase () || ch == '.' || ch == '/' }, 1 .. ),
		],
		LsCommand = [ "$ ls" ],
		DirEntry { name: InpStr <'inp> } = [
			"dir ",
			@str name = (|ch| { ch.is_ascii_lowercase () || ch == '.' }, 1 .. ),
		],
		FileEntry { size: u32, name: InpStr <'inp> } = [
			size, " ",
			@str name = (|ch| { ch.is_ascii_lowercase () || ch == '.' }, 1 .. ),
		],
	}
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
