use super::*;

const MAX_BUILDER_DEPTH: u32 = 100;

#[ derive (Clone) ]
pub struct Matcher {
	inner: Rc <MatcherInner>,
}

impl Debug for Matcher {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.debug_struct ("Matcher")
			.field ("id", & self.inner.id)
			.field ("matches", & self.inner.matches)
			.field ("next", & self.inner.next)
			.finish ()
	}
}

#[ derive (Clone) ]
pub struct MatcherInner {
	id: u32,
	matches: bool,
	next: Vec <(char, Matcher)>,
	builder: RcWeak <RefCell <MatcherBuilderInner>>,
}

impl Matcher {

	#[ inline ]
	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.inner.next.is_empty () && ! self.inner.matches
	}

	#[ inline ]
	#[ must_use ]
	pub fn matches (& self, msg: & str) -> bool {
		self.match_prefix (msg).any (|(_, suffix)| suffix.is_empty ())
	}

	#[ allow (clippy::string_slice) ]
	#[ inline ]
	pub fn match_prefix <'msg: 'iter, 'iter> (
		& self,
		msg: & 'msg str,
	) -> impl Iterator <Item = (& 'iter str, & 'iter str)> + 'msg {
		let mut matcher = Some (self.clone ());
		let mut msg_chars = msg.chars ();
		let mut match_len = 0;
		iter::from_fn (move || {
			let matcher_val = some_or! (matcher.as_ref (), return None);
			let result = matcher_val.inner.matches.then (||
				(& msg [ .. match_len], & msg [match_len .. ]));
			let msg_ch = some_or! (msg_chars.next (), {
				matcher = None;
				return Some (result);
			});
			let next_matcher = matcher_val.inner.next.iter ().find (|&& (ch, _)| ch == msg_ch);
			let & (_, ref next_matcher) = some_or! (next_matcher, {
				matcher = None;
				return Some (result);
			});
			matcher = Some (next_matcher.clone ());
			match_len += msg_ch.len_utf8 ();
			Some (result)
		}).flatten ()
	}

	#[ inline ]
	pub fn push_empty (self) -> GenResult <Self> {
		let builder = self.inner.builder.upgrade ().unwrap ();
		let mut builder = builder.deref ().borrow_mut ();
		builder.push_empty (self, 0)
	}

	#[ inline ]
	pub fn push_char (self, new_ch: char) -> GenResult <Self> {
		let builder = self.inner.builder.upgrade ().unwrap ();
		let mut builder = builder.deref ().borrow_mut ();
		builder.push_char (self, new_ch, 0)
	}

	#[ inline ]
	pub fn concat (self, other: Self) -> GenResult <Self> {
		let builder = self.inner.builder.upgrade ().unwrap ();
		let mut builder = builder.deref ().borrow_mut ();
		builder.concat (self, other, 0)
	}

	#[ inline ]
	pub fn union (self, other: Self) -> GenResult <Self> {
		let builder = self.inner.builder.upgrade ().unwrap ();
		let mut builder = builder.deref ().borrow_mut ();
		builder.union (self, other, 0)
	}

}

pub struct MatcherBuilder {
	inner: Rc <RefCell <MatcherBuilderInner>>,
}

#[ derive (Debug) ]
struct MatcherBuilderInner {
	next_id: u32,
	none: Matcher,
	push_empty: HashMap <u32, Matcher>,
	push_char: HashMap <(u32, char), Matcher>,
	union: HashMap <(u32, u32), Matcher>,
	concat: HashMap <(u32, u32), Matcher>,
}

impl MatcherBuilder {

	#[ must_use ]
	pub fn new () -> Self {
		Self {
			inner: Rc::new_cyclic (|builder| RefCell::new (
				MatcherBuilderInner {
					next_id: 1_u32,
					none: Matcher {
						inner: Rc::new (MatcherInner {
							id: 0,
							matches: false,
							next: Vec::new (),
							builder: RcWeak::clone (builder),
						}),
					},
					push_empty: HashMap::new (),
					push_char: HashMap::new (),
					union: HashMap::new (),
					concat: HashMap::new (),
				}
			)),
		}
	}

	pub fn char (& self, ch: char) -> GenResult <Matcher> {
		self.none ().push_char (ch)
	}

	pub fn empty (& self) -> GenResult <Matcher> {
		self.none ().push_empty ()
	}

	#[ must_use ]
	pub fn none (& self) -> Matcher {
		self.inner.deref ().borrow ().none.clone ()
	}

	#[ must_use ]
	pub fn len (& self) -> usize {
		let inner = self.inner.deref ().borrow ();
		[
			inner.push_empty.len (),
			inner.push_char.len (),
			inner.concat.len (),
			inner.union.len (),
		].into_iter ().sum ()
	}

	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.len () == 0
	}

}

impl Default for MatcherBuilder {
	fn default () -> Self { Self::new () }
}

impl MatcherBuilderInner {

	fn push_char (
		& mut self,
		mut matcher: Matcher,
		new_ch: char,
		depth: u32,
	) -> GenResult <Matcher> {

		if MAX_BUILDER_DEPTH < depth { return Err ("Too much recursion".into ()) }

		let cache_key = (matcher.inner.id, new_ch);
		if let Some (matcher) = self.push_char.get (& cache_key) {
			return Ok (matcher.clone ());
		}

		let inner = Rc::make_mut (& mut matcher.inner);
		inner.id = self.next_id;
		self.next_id += 1;

		if let Some (idx) = inner.next.iter ().position (|& (ch, _)| ch == new_ch) {
			inner.next [idx].1 =
				self.push_empty (
					inner.next [idx].1.clone (),
					depth + 1) ?;
		} else {
			inner.next.push ((
				new_ch,
				self.push_empty (
					self.none.clone (),
					depth + 1) ?,
			));
			inner.next.sort_by_key (|& (ch, _)| ch);
		}
		self.push_char.insert (cache_key, matcher.clone ());

		Ok (matcher)

	}

	fn push_empty (
		& mut self,
		mut matcher: Matcher,
		depth: u32,
	) -> GenResult <Matcher> {

		if MAX_BUILDER_DEPTH < depth { return Err ("Too much recursion".into ()) }
		if matcher.inner.matches { return Ok (matcher) }

		let cache_key = matcher.inner.id;
		if let Some (matcher) = self.push_empty.get (& cache_key) {
			return Ok (matcher.clone ());
		}

		let inner = Rc::make_mut (& mut matcher.inner);
		inner.id = self.next_id;
		self.next_id += 1;

		inner.matches = true;

		self.push_empty.insert (cache_key, matcher.clone ());
		Ok (matcher)

	}

	fn concat (
		& mut self,
		mut matcher: Matcher,
		other: Matcher,
		depth: u32,
	) -> GenResult <Matcher> {

		if MAX_BUILDER_DEPTH < depth { return Err ("Too much recursion".into ()) }

		let cache_key = (matcher.inner.id, other.inner.id);
		if let Some (matcher) = self.concat.get (& cache_key) {
			return Ok (matcher.clone ());
		}

		let inner = Rc::make_mut (& mut matcher.inner);
		inner.id = self.next_id;
		self.next_id += 1;

		self.concat_real (inner, & other, depth) ?;

		self.concat.insert (cache_key, matcher.clone ());
		Ok (matcher)

	}

	fn union (
		& mut self,
		mut matcher: Matcher,
		other: Matcher,
		depth: u32
	) -> GenResult <Matcher> {

		if MAX_BUILDER_DEPTH < depth { return Err ("Too much recursion".into ()) }

		let cache_key = (matcher.inner.id, other.inner.id);
		if let Some (matcher) = self.union.get (& cache_key) {
			return Ok (matcher.clone ());
		}

		let inner = Rc::make_mut (& mut matcher.inner);
		inner.id = self.next_id;
		self.next_id += 1;

		self.union_real (inner, & other, depth + 1) ?;

		self.union.insert (cache_key, matcher.clone ());
		Ok (matcher)

	}

	fn concat_real (
		& mut self,
		inner: & mut MatcherInner,
		other: & Matcher,
		depth: u32,
	) -> GenResult <()> {
		if MAX_BUILDER_DEPTH < depth { return Err ("Too much recursion".into ()) }
		for & mut (_, ref mut next_matcher) in inner.next.iter_mut () {
			* next_matcher =
				self.concat (
					next_matcher.clone (),
					other.clone (),
					depth + 1) ?;
		}
		if inner.matches {
			inner.matches = false;
			self.union_real (inner, other, depth + 1) ?;
		}
		Ok (())
	}

	fn union_real (
		& mut self,
		inner: & mut MatcherInner,
		other: & Matcher,
		depth: u32,
	) -> GenResult <()> {
		if MAX_BUILDER_DEPTH < depth { return Err ("Too much recursion".into ()) }
		if other.inner.matches { inner.matches = true; }
		for & (other_ch, ref other_matcher) in & other.inner.next {
			if let Some (idx) = inner.next.iter ().position (|& (ch, _)| ch == other_ch) {
				inner.next [idx].1 =
					self.union (
						inner.next [idx].1.clone (),
						other_matcher.clone (),
						depth + 1) ?;
			} else {
				inner.next.push ((other_ch, other_matcher.clone ()));
				inner.next.sort_by_key (|& (ch, _)| ch);
			}
		}
		Ok (())
	}

}
