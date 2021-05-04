use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	lazy_static! {
		static ref LINE_RE: Regex = Regex::new (r"^(.+?) bags contain ([^\.]*)\.$").unwrap ();
		static ref BAG_QTY_RE: Regex = Regex::new (r"^(\d+) (.+?) bags?$").unwrap ();
	}
	let direct_contents: HashMap <String, Vec <(String, u64)>> =
			input_lines.iter ().map (|input_line| {
		let line_captures = LINE_RE.captures (input_line).unwrap ();
		let container = line_captures.get (1).unwrap ().as_str ().to_string ();
		let contents_str = line_captures.get (2).unwrap ().as_str ();
		let contents: Vec <(String, u64)> = if contents_str == "no other bags" {
			Vec::new ()
		} else {
			contents_str.split (", ").map (|contents_str| {
				let contents_captures = BAG_QTY_RE.captures (contents_str).unwrap ();
				(
					contents_captures.get (2).unwrap ().as_str ().to_string (),
					contents_captures.get (1).unwrap ().as_str ().parse ().unwrap (),
				)
			}).collect ()
		};
		(container, contents)
	}).collect ();
	let mut all_contents: HashMap <String, HashMap <String, u64>> = HashMap::new ();
	loop {
		let mut progress = false;
		let mut incomplete = false;
		for (this_container, this_contents) in direct_contents.iter () {
			if all_contents.contains_key (this_container) { continue }
			if this_contents.iter ().all (
				|(this_contents_name, _)| all_contents.contains_key (this_contents_name),
			) {
				let mut this_all_contents: HashMap <String, u64> = HashMap::new ();
				for (this_contents_name, this_contents_num) in this_contents.iter () {
					* this_all_contents.entry (
						this_contents_name.clone (),
					).or_insert (0) += this_contents_num;
					for (that_contents_name, that_contents_num)
							in all_contents [this_contents_name].iter () {
						* this_all_contents.entry (
							that_contents_name.clone (),
						).or_insert (0) += this_contents_num * that_contents_num;
					}
				}
				all_contents.insert (this_container.clone (), this_all_contents);
				progress = true;
			} else {
				incomplete = true;
			}
		}
		if ! progress { panic! () }
		if ! incomplete { break }
	}
	let container_name = "shiny gold".to_string ();
	println! ("One {} bag must contain:", container_name);
	let mut total_contents = 0;
	for (contents_name, contents_num) in all_contents [& container_name].iter () {
		println! (" - {} × {}", contents_num, contents_name);
		total_contents += contents_num;
	}
	println! ("Total contents: {}", total_contents);
}
