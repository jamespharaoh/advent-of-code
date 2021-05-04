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
	let direct_contents: HashMap <String, Vec <String>> = input_lines.iter ().map (|input_line| {
		let line_captures = LINE_RE.captures (input_line).unwrap ();
		let container = line_captures.get (1).unwrap ().as_str ().to_string ();
		let contents_str = line_captures.get (2).unwrap ().as_str ();
		let contents: Vec <String> = if contents_str == "no other bags" {
			Vec::new ()
		} else {
			contents_str.split (", ").map (|contents_str| {
				let contents_captures = BAG_QTY_RE.captures (contents_str).unwrap ();
				contents_captures.get (2).unwrap ().as_str ().to_string ()
			}).collect ()
		};
		(container, contents)
	}).collect ();
	let mut all_contents: HashMap <String, Vec <String>> = HashMap::new ();
	loop {
		let mut progress = false;
		let mut incomplete = false;
		for (container, contents) in direct_contents.iter () {
			if all_contents.contains_key (container) { continue }
			if contents.iter ().all (|contents| all_contents.contains_key (contents)) {
				let contents: HashSet <String> = contents.iter ().map (
					|contents| all_contents [contents].iter (),
				).flatten ().chain (contents).cloned ().collect ();
				all_contents.insert (container.clone (), contents.into_iter ().collect ());
				progress = true;
			} else {
				incomplete = true;
			}
		}
		if ! progress { panic! () }
		if ! incomplete { break }
	}
	let search_for: String = "shiny gold".to_string ();
	let num_containers: usize = all_contents.iter ().filter (
		|(_, contents)| contents.contains (& search_for),
	).count ();
	println! ("Number of containers: {}", num_containers);
}
