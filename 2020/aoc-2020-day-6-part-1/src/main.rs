use std::collections::HashSet;
use std::fs;

fn main () {
	let output_string = fs::read_to_string ("input").unwrap ();
	let output_lines: Vec <& str> = output_string.trim ().split ('\n').collect ();
	let group_answers: Vec <HashSet <char>> = output_lines.split (|s| * s == "").map (
		|group_lines| group_lines.iter ().map (|group_line| group_line.chars ()).flatten ().fold (
			HashSet::new (),
			|mut items, ch| { items.insert (ch); items },
		),
	).collect ();
	let num_answers: usize = group_answers.iter ().map (|answers| answers.len ()).sum ();
	println! ("Total number of answers: {}", num_answers);
}
