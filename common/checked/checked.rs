#![ allow (clippy::wildcard_enum_match_arm) ]
#![ allow (clippy::single_match_else) ]

extern crate proc_macro;

use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::token_stream::IntoIter as TokenStreamIter;
use std::iter::Peekable;
use std::vec::IntoIter as VecIter;

type TokenIter = Peekable <TokenStreamIter>;

#[proc_macro]
pub fn checked (tokens: TokenStream) -> TokenStream {
	let mut iter = tokens.into_iter ().peekable ();
	CheckedTop::parse (& mut iter).into ()
}

#[ derive (Debug) ]
enum CheckedTop {
	Expr (CheckedExpr),
	Assign (CheckedItem, Punct, Punct, CheckedExpr),
}

impl CheckedTop {
	fn parse (iter: & mut TokenIter) -> Self {
		match Self::parse_assign (& mut iter.clone ()) {
			Some (val) => val,
			_ => Self::Expr (CheckedExpr::parse (iter)),
		}
	}
	fn parse_assign (iter: & mut TokenIter) -> Option <Self> {
		let left = CheckedItem::parse (iter);
		let punct_0 = match iter.next () {
			Some (TokenTree::Punct (punct)) => punct,
			_ => panic! (),
		};
		if punct_0.spacing () != Spacing::Joint { return None }
		let punct_1 = match iter.next () {
			Some (TokenTree::Punct (punct)) => punct,
			_ => return None,
		};
		if punct_1.as_char () != '=' { return None }
		Some (Self::Assign (left, punct_0, punct_1, CheckedExpr::parse (iter)))
	}
}

impl From <CheckedTop> for TokenStream {
	fn from (top: CheckedTop) -> Self {
		match top {
			CheckedTop::Expr (expr) => expr.into (),
			CheckedTop::Assign (left, punct_0, _punct_1, right) => {
				let fn_name = match punct_0.as_char () {
					'+' => "try_add_assign",
					'/' => "try_div_assign",
					'*' => "try_mul_assign",
					'%' => "try_rem_assign",
					'-' => "try_sub_assign",
					_ => unreachable! (),
				};
				[
					// {
					//   match $right {
					//     Ok (__right) => ($left).$fn_name (__right),
					//     Err (__err) => Err (__err),
					//   }
					// }
					TokenTree::Group (Group::new (Delimiter::Brace, [
						TokenTree::Ident (Ident::new ("match", Span::mixed_site ())),
						right.into (),
						TokenTree::Group (Group::new (Delimiter::Brace, [
							TokenTree::Ident (Ident::new ("Ok", Span::mixed_site ())),
							TokenTree::Group (Group::new (Delimiter::Parenthesis, [
								TokenTree::Ident (Ident::new ("__right", Span::mixed_site ())),
							].into_iter ().collect ())),
							TokenTree::Punct (Punct::new ('=', Spacing::Joint)),
							TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
							TokenTree::Group (Group::new (Delimiter::Parenthesis, left.into ())),
							TokenTree::Punct (Punct::new ('.', Spacing::Alone)),
							TokenTree::Ident (Ident::new (fn_name, Span::mixed_site ())),
							TokenTree::Group (Group::new (Delimiter::Parenthesis, [
								TokenTree::Ident (Ident::new ("__right", Span::mixed_site ())),
							].into_iter ().collect ())),
							TokenTree::Punct (Punct::new (',', Spacing::Alone)),
							TokenTree::Ident (Ident::new ("Err", Span::mixed_site ())),
							TokenTree::Group (Group::new (Delimiter::Parenthesis, [
								TokenTree::Ident (Ident::new ("__err", Span::mixed_site ())),
							].into_iter ().collect ())),
							TokenTree::Punct (Punct::new ('=', Spacing::Joint)),
							TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
							TokenTree::Ident (Ident::new ("Err", Span::mixed_site ())),
							TokenTree::Group (Group::new (Delimiter::Parenthesis, [
								TokenTree::Ident (Ident::new ("__err", Span::mixed_site ())),
							].into_iter ().collect ())),
							TokenTree::Punct (Punct::new (',', Spacing::Alone)),
						].into_iter ().collect ())),
					].into_iter ().collect ()))
				].into_iter ().collect ()
			},
		}
	}
}

#[ derive (Clone, Debug) ]
enum CheckedExpr {
	Item (CheckedItem),
	Add (Box <CheckedExpr>, Punct, Box <CheckedExpr>),
	Div (Box <CheckedExpr>, Punct, Box <CheckedExpr>),
	Mul (Box <CheckedExpr>, Punct, Box <CheckedExpr>),
	Rem (Box <CheckedExpr>, Punct, Box <CheckedExpr>),
	Sub (Box <CheckedExpr>, Punct, Box <CheckedExpr>),
}

impl CheckedExpr {
	fn parse (iter: & mut TokenIter) -> Self {
		Self::parse_loose (iter)
	}
	fn parse_loose (iter: & mut TokenIter) -> Self {
		let mut result = Self::parse_tight (iter);
		while iter.peek ().is_some () {
			let punct = match iter.next ().unwrap () {
				TokenTree::Punct (punct) => punct,
				_ => panic! (),
			};
			result = match punct.as_char () {
				'+' => Self::Add (result.into (), punct, Self::parse_tight (iter).into ()),
				'-' => Self::Sub (result.into (), punct, Self::parse_tight (iter).into ()),
				_ => panic! (),
			}
		}
		result
	}
	fn parse_tight (iter: & mut TokenIter) -> Self {
		let mut result = Self::parse_single (iter);
		while iter.peek ().is_some () {
			let punct = match * iter.peek ().unwrap () {
				TokenTree::Punct (ref punct) => punct,
				_ => panic! (),
			};
			let ctor = match punct.as_char () {
				'/' => Self::Div,
				'*' => Self::Mul,
				'%' => Self::Rem,
				_ => return result,
			};
			let punct = match iter.next ().unwrap () {
				TokenTree::Punct (punct) => punct,
				_ => panic! (),
			};
			result = ctor (result.into (), punct, Self::parse_single (iter).into ());
		}
		result
	}
	fn parse_single (iter: & mut TokenIter) -> Self {
		match iter.peek () {
			Some (& TokenTree::Group (ref group))
					if group.delimiter () == Delimiter::Parenthesis => {
				let inner = Self::parse_loose (& mut group.stream ().into_iter ().peekable ());
				iter.next ().unwrap ();
				inner
			},
			Some (_) => Self::Item (CheckedItem::parse (iter)),
			_ => panic! (),
		}
	}
}

impl From <Box <CheckedExpr>> for TokenStream {
	fn from (expr: Box <CheckedExpr>) -> Self {
		Self::from (& * expr)
	}
}

impl From <& Box <CheckedExpr>> for TokenStream {
	fn from (expr: & Box <CheckedExpr>) -> Self {
		Self::from (& ** expr)
	}
}

impl From <CheckedExpr> for TokenStream {
	fn from (expr: CheckedExpr) -> Self {
		Self::from (& expr)
	}
}

impl From <& CheckedExpr> for TokenStream {
	fn from (expr: & CheckedExpr) -> Self {
		let (fn_name, left, _punct, right) = match * expr {
			CheckedExpr::Item (ref item) => {
				// Ok::<_, Overflow> ($item)
				return [
					TokenTree::Ident (Ident::new ("Ok", Span::mixed_site ())),
					TokenTree::Punct (Punct::new (':', Spacing::Joint)),
					TokenTree::Punct (Punct::new (':', Spacing::Alone)),
					TokenTree::Punct (Punct::new ('<', Spacing::Alone)),
					TokenTree::Ident (Ident::new ("_", Span::mixed_site ())),
					TokenTree::Punct (Punct::new (',', Spacing::Alone)),
					TokenTree::Ident (Ident::new ("Overflow", Span::mixed_site ())),
					TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
					TokenTree::Group (Group::new (Delimiter::Parenthesis, item.into ())),
				].into_iter ().collect ();
			},
			CheckedExpr::Add (ref left, ref punct, ref right) => ("try_add", left, punct, right),
			CheckedExpr::Div (ref left, ref punct, ref right) => ("try_div", left, punct, right),
			CheckedExpr::Mul (ref left, ref punct, ref right) => ("try_mul", left, punct, right),
			CheckedExpr::Rem (ref left, ref punct, ref right) => ("try_rem", left, punct, right),
			CheckedExpr::Sub (ref left, ref punct, ref right) => ("try_sub", left, punct, right),
		};
		[
			// match $left {
			//   Ok (__left) => match $right {
			//     Ok (__right) => __left.$fn_name (__right),
			//     Err (__err) => Err (__err),
			//   },
			//   Err (__err) => Err (__err),
			// }
			TokenTree::Ident (Ident::new ("match", Span::mixed_site ())),
			left.into (),
			TokenTree::Group (Group::new (Delimiter::Brace, [
				TokenTree::Ident (Ident::new ("Ok", Span::mixed_site ())),
				TokenTree::Group (Group::new (Delimiter::Parenthesis, [
					TokenTree::Ident (Ident::new ("__left", Span::mixed_site ())),
				].into_iter ().collect ())),
				TokenTree::Punct (Punct::new ('=', Spacing::Joint)),
				TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
				TokenTree::Ident (Ident::new ("match", Span::mixed_site ())),
				right.into (),
				TokenTree::Group (Group::new (Delimiter::Brace, [
					TokenTree::Ident (Ident::new ("Ok", Span::mixed_site ())),
					TokenTree::Group (Group::new (Delimiter::Parenthesis, [
						TokenTree::Ident (Ident::new ("__right", Span::mixed_site ())),
					].into_iter ().collect ())),
					TokenTree::Punct (Punct::new ('=', Spacing::Joint)),
					TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
					TokenTree::Ident (Ident::new ("__left", Span::mixed_site ())),
					TokenTree::Punct (Punct::new ('.', Spacing::Alone)),
					TokenTree::Ident (Ident::new (fn_name, Span::mixed_site ())),
					TokenTree::Group (Group::new (Delimiter::Parenthesis, [
						TokenTree::Ident (Ident::new ("__right", Span::mixed_site ())),
					].into_iter ().collect ())),
					TokenTree::Punct (Punct::new (',', Spacing::Alone)),
					TokenTree::Ident (Ident::new ("Err", Span::mixed_site ())),
					TokenTree::Group (Group::new (Delimiter::Parenthesis, [
						TokenTree::Ident (Ident::new ("__err", Span::mixed_site ())),
					].into_iter ().collect ())),
					TokenTree::Punct (Punct::new ('=', Spacing::Joint)),
					TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
					TokenTree::Ident (Ident::new ("Err", Span::mixed_site ())),
					TokenTree::Group (Group::new (Delimiter::Parenthesis, [
						TokenTree::Ident (Ident::new ("__err", Span::mixed_site ())),
					].into_iter ().collect ())),
					TokenTree::Punct (Punct::new (',', Spacing::Alone)),
				].into_iter ().collect ())),
				TokenTree::Punct (Punct::new (',', Spacing::Alone)),
				TokenTree::Ident (Ident::new ("Err", Span::mixed_site ())),
				TokenTree::Group (Group::new (Delimiter::Parenthesis, [
					TokenTree::Ident (Ident::new ("__err", Span::mixed_site ())),
				].into_iter ().collect ())),
				TokenTree::Punct (Punct::new ('=', Spacing::Joint)),
				TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
				TokenTree::Ident (Ident::new ("Err", Span::mixed_site ())),
				TokenTree::Group (Group::new (Delimiter::Parenthesis, [
					TokenTree::Ident (Ident::new ("__err", Span::mixed_site ())),
				].into_iter ().collect ())),
				TokenTree::Punct (Punct::new (',', Spacing::Alone)),
			].into_iter ().collect ())),
		].into_iter ().collect ()
	}
}

impl From <CheckedExpr> for TokenTree {
	fn from (expr: CheckedExpr) -> Self {
		Self::Group (Group::new (Delimiter::None, expr.into ()))
	}
}

impl From <& Box <CheckedExpr>> for TokenTree {
	fn from (expr: & Box <CheckedExpr>) -> Self {
		Self::Group (Group::new (Delimiter::None, expr.into ()))
	}
}

#[ derive (Clone, Debug) ]
struct CheckedItem (Vec <TokenTree>);

impl CheckedItem {
	fn parse (iter: & mut TokenIter) -> Self {
		let mut tokens = Vec::new ();
		let mut first = true;
		loop {
			const ARITH_CHARS: [char; 5] = ['+', '-', '*', '/', '%'];
			match iter.peek () {
				Some (& TokenTree::Punct (ref punct))
						if ! first && ARITH_CHARS.contains (& punct.as_char ()) => {
					assert! (! tokens.is_empty ());
					return Self (tokens);
				},
				None => return Self (tokens),
				_ => tokens.push (iter.next ().unwrap ()),
			}
			first = false;
		}
	}
}

impl IntoIterator for CheckedItem {
	type Item = TokenTree;
	type IntoIter = VecIter <TokenTree>;
	fn into_iter (self) -> VecIter <TokenTree> {
		let Self (tokens) = self;
		tokens.into_iter ()
	}
}

impl From <CheckedItem> for TokenStream {
	fn from (CheckedItem (tokens): CheckedItem) -> Self {
		tokens.into_iter ().collect ()
	}
}

impl From <& CheckedItem> for TokenStream {
	fn from (& CheckedItem (ref tokens): & CheckedItem) -> Self {
		tokens.iter ().cloned ().collect ()
	}
}

impl From <CheckedItem> for TokenTree {
	fn from (item: CheckedItem) -> Self {
		Self::Group (Group::new (Delimiter::None, item.into ()))
	}
}
