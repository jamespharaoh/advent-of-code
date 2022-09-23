use super::*;

pub type RuleId = u8;

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Rule {
	pub options: Vec <RuleOption>,
}

struct_parser_display! {
	Rule { options } = [ @delim " | " options ]
}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct RuleOption {
	pub items: Vec <RuleItem>,
}

struct_parser_display! {
	RuleOption { items } = [ @delim " " items ]
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum RuleItem {
	Rule (RuleId),
	Char (char),
}

enum_parser_display! {
	RuleItem,
	Rule (rule_id) = [ rule_id ],
	Char (ch) = [ "\"", @char ch = |ch| { ch.is_ascii_lowercase () }, "\"" ],
}

impl Rule {

	#[ inline ]
	#[ must_use ]
	pub fn two (rule_id_0: RuleId, rule_id_1: RuleId) -> Self {
		Self { options: vec! [
			RuleOption { items: vec! [
				RuleItem::Rule (rule_id_0),
				RuleItem::Rule (rule_id_1),
			] },
		] }
	}

	#[ inline ]
	pub fn deps (& self) -> impl Iterator <Item = RuleId> + '_ {
		self.options.iter ()
			.flat_map (|option| option.items.iter ()
				.filter_map (|& item| match item {
					RuleItem::Rule (rule_id) => Some (rule_id),
					RuleItem::Char (_) => None,
				}))
	}

}
