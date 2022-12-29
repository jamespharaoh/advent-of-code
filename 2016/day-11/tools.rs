use super::*;

use std::fs::File;
use std::io::Read as _;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

args_decl! {
	pub struct CorpusGenArgs {
		output_dir: PathBuf,
		num_files: Option <usize>,
		num_comps: Option <usize>,
		name_len: Option <usize>,
	}
}

#[ allow (clippy::needless_pass_by_value) ]
pub fn corpus_gen (args: CorpusGenArgs) -> GenResult <()> {
	let mut rand = File::open ("/dev/urandom") ?;
	for _ in 0 .. args.num_files.unwrap_or (10) {
		let names =
			iter::from_fn (
					|| -> Option <String> {
						let mut name = String::new ();
						while name.len () < args.name_len.unwrap_or (4) {
							let mut buf = [0];
							assert_eq! (rand.read (& mut buf).unwrap (), 1);
							let ch = ok_or! (buf [0].to_char (), continue);
							if ! ch.is_ascii_lowercase () { continue }
							name.push (ch);
						}
						Some (name)
					})
				.take (args.num_comps.unwrap_or (5))
				.collect::<Vec <String>> ();
		let comps =
			iter::from_fn (
					|| -> Option <[u8; 2]> {
						let mut buf = [0];
						assert_eq! (rand.read (& mut buf).unwrap (), 1);
						Some ([(buf [0] & 0x3) + 1, ((buf [0] >> 2_u32) & 0x3) + 1])
					})
				.take (7)
				.collect::<Vec <[u8; 2]>> ();
		let mut output = String::new ();
		for floor in 1 ..= 4 {
			let mut items = Vec::new ();
			for (name, [gen, chip]) in
				names.iter ()
					.zip (comps.iter ().copied ())
					.take (args.num_comps.unwrap_or (5)) {
				if gen == floor {
					items.push (format! ("{name} generator"));
				}
				if chip == floor {
					items.push (format! ("{name}-compatible microchip"));
				}
			}
			let floor_name = match floor {
				1 => "first", 2 => "second", 3 => "third", 4 => "fourth",
				_ => unreachable! (),
			};
			if items.is_empty () {
				writeln! (& mut output, "The {floor_name} floor contains nothing relevant.")
					.unwrap ();
			} else if items.len () == 1 {
				writeln! (& mut output, "The {floor_name} floor contains a {}.", items [0])
					.unwrap ();
			} else if items.len () == 2 {
				writeln! (& mut output, "The {floor_name} floor contains a {} and a {}.",
					items [0], items [1]).unwrap ();
			} else {
				write! (& mut output, "The {} floor contains a {}", floor_name, items [0]).unwrap ();
				for item in & items [1 .. items.len () - 1] {
					write! (& mut output, ", a {item}").unwrap ();
				}
				writeln! (& mut output, ", and a {}.", items [items.len () - 1]).unwrap ();
			}
		}
		let mut sum_command =
			Command::new ("sha1sum")
				.stdin (Stdio::piped ())
				.stdout (Stdio::piped ())
				.spawn ()
				.unwrap ();
		let mut sum_stdin = sum_command.stdin.take ().unwrap ();
		sum_stdin.write_all (output.as_bytes ()).unwrap ();
		drop (sum_stdin);
		let sum_output = sum_command.wait_with_output ().unwrap ();
		let sum_output_vec = sum_output.stdout [0 .. 40].to_vec ();
		let sum_output_str = String::from_utf8 (sum_output_vec).unwrap ();
		let mut output_path = args.output_dir.clone ();
		output_path.push (sum_output_str);
		let mut output_file = File::create (output_path).unwrap ();
		output_file.write_all (output.as_bytes ()).unwrap ();
	}
	Ok (())
}
