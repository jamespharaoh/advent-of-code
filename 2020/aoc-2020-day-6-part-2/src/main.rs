use std::collections::HashSet;
use std::fs;

fn main () {
	let output_string = fs::read_to_string ("input").unwrap ();
	let output_lines: Vec <& str> = output_string.trim ().split ('\n').collect ();
	let group_answers: Vec <HashSet <char>> = output_lines.split (|s| * s == "").map (
		|group_lines| group_lines.iter ().map (|group_line|
			group_line.chars ().fold (
				HashSet::new (),
				|mut passenger_answers, ch| {
					passenger_answers.insert (ch);
					passenger_answers
				},
			),
		).fold (
			None,
			|group_answers: Option <HashSet <char>>, passenger_answers| {
				if let Some (group_answers) = group_answers {
					Some (group_answers.intersection (& passenger_answers).cloned ().collect ())
				} else {
					Some (passenger_answers.clone ())
				}
			},
		),
	).map (Option::unwrap).collect ();
	let num_answers: usize =
		group_answers.iter ().map (|group_answers| group_answers.len ()).sum ();
	println! ("Number of answers: {}", num_answers);
}
