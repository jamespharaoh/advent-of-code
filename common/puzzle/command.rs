use super::*;

pub struct PuzzleCommand {
	name: & 'static str,
	invoke_fn: Box <dyn Fn (Vec <OsString>) -> GenResult <()>>,
}

impl PuzzleCommand {

	#[ inline ]
	pub fn new <
		Args: ArgsParse + 'static,
		InvokeFn: Fn (Args) -> GenResult <()> + 'static,
	> (
		name: & 'static str,
		invoke_fn: InvokeFn,
	) -> Self {

		let invoke_fn = Box::new (
			move |args| invoke_fn (Args::parse (args).unwrap ()),
		);

		Self { name, invoke_fn }

	}

	#[ inline ]
	#[ must_use ]
	pub const fn name (& self) -> & str {
		self.name
	}

	#[ inline ]
	pub fn invoke (& self, args: impl Iterator <Item = OsString>) -> GenResult <()> {
		(self.invoke_fn) (args.into_iter ().collect::<Vec <OsString>> ())
	}

}
