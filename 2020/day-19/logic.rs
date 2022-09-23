//! Logic for solving the puzzles

use super::*;

use input::Input;
use input::InputRule;
use matcher::Matcher;
use matcher::MatcherBuilder;
use model::Rule;
use model::RuleId;
use model::RuleItem;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let rules = get_rules (input) ?;
	let mut builder = Builder::new (& rules);
	let matcher = builder.matcher (0) ?;
	Ok (
		input.messages.iter ()
			.filter (|& msg| matcher.matches (msg))
			.count ()
			.as_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let rules = get_rules (input) ?;
	let mut builder = Builder::new (& rules);
	let prefixes = builder.matcher (42) ?;
	let suffixes = builder.matcher (31) ?;
	input.messages.iter ()
		.map (|msg| match_message (& prefixes, & suffixes, msg))
		.try_fold (0, |sum, item| item.map (|val| sum + u32::from (val)))
}

pub fn match_message (prefixes: & Matcher, suffixes: & Matcher, msg: & str) -> GenResult <bool> {
	let mut todo = Vec::new ();
	todo.push ((msg, 0_u32, 0_u32));
	let mut num_iters = 0_u32;
	while let Some ((msg, num_prefix, num_suffix)) = todo.pop () {
		if num_iters == 100 { return Err ("Giving up after max iters".into ()) }
		num_iters += 1;
		if num_suffix == 0 {
			for (_, msg) in prefixes.match_prefix (msg) {
				todo.push ((msg, num_prefix + 1, num_suffix));
			}
		}
		if 2 <= num_prefix && num_suffix < num_prefix - 1 {
			for (_, msg) in suffixes.match_prefix (msg) {
				if msg.is_empty () { return Ok (true) }
				todo.push ((msg, num_prefix, num_suffix + 1));
			}
		}
	}
	Ok (false)
}

fn get_rules (input: & Input) -> GenResult <HashMap <RuleId, Rule>> {
	let mut rules: HashMap <RuleId, Rule> = HashMap::new ();
	for & InputRule { id: rule_id, ref rule } in & input.rules {
		if rules.contains_key (& rule_id) {
			return Err (format! ("Duplicated rule id: {rule_id}").into ());
		}
		rules.insert (rule_id, rule.clone ());
	}
	for (& rule_id, rule) in & rules {
		if let Some (dep) = rule.deps ().find (|dep| ! rules.contains_key (dep)) {
			return Err (format! ("Rule {rule_id} references rule {dep} which doesn't exist")
				.into ());
		}
	}
	Ok (rules)
}

struct Builder <'rul> {
	cache: HashMap <RuleId, Matcher>,
	rules: & 'rul HashMap <RuleId, Rule>,
	builder: MatcherBuilder,
}

impl <'rul> Builder <'rul> {
	fn new (rules: & 'rul HashMap <RuleId, Rule>) -> Self {
		Builder {
			cache: HashMap::new (),
			rules,
			builder: MatcherBuilder::new (),
		}
	}
	fn matcher (& mut self, rule_id: RuleId) -> GenResult <Matcher> {
		if ! self.rules.contains_key (& rule_id) {
			return Err (format! ("No such rule: {rule_id}").into ());
		}
		let mut todo = Vec::new ();
		todo.push (rule_id);
		'OUTER: while let Some (rule_id) = todo.pop () {
			for dep_rule_id in self.rules [& rule_id].deps () {
				if self.cache.contains_key (& dep_rule_id) { continue }
				if dep_rule_id == rule_id {
					return Err (format! ("Rule {rule_id} directly references itself").into ());
				}
				if todo.contains (& dep_rule_id) {
					return Err (format! ("Rule {rule_id} indirectly references itself").into ());
				}
				todo.push (rule_id);
				todo.push (dep_rule_id);
				continue 'OUTER;
			}
			let matcher =
				self.rules [& rule_id].options.iter ()
					.map (|option| option.items.iter ()
						.map (|& item| match item {
							RuleItem::Rule (rule_id) => Ok (self.cache [& rule_id].clone ()),
							RuleItem::Char (ch) => self.builder.char (ch),
						})
						.try_fold (self.builder.empty () ?, |matcher, item| matcher.concat (item ?)))
					.try_fold (self.builder.none (), |matcher, option| matcher.union (option ?)) ?;
			self.cache.insert (rule_id, matcher);
		}
		if 5000 < self.builder.len () { return Err ("Giving up due to complexity".into ()) }
		Ok (self.cache.remove (& rule_id).unwrap ())
	}
}
