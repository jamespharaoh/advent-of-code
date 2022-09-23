#![ allow (clippy::wildcard_enum_match_arm) ]

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
use std::iter;
use std::iter::Peekable;
use std::vec::IntoIter as VecIter;

type TokenIter = Peekable <TokenStreamIter>;

/*
top = item arith-assign expr
item = non-arithmetic +
expr = item | expr arith expr | (expr)

item -> Ok (item)
left.op (right) => left.and_then (|left| right.and_then (|right| left.checked_op (right)))
*/

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
			CheckedTop::Assign (left, punct_0, punct_1, right) => {
				let fn_name = match punct_0.as_char () {
					'+' => "try_add",
					'/' => "try_div",
					'*' => "try_mul",
					'%' => "try_rem",
					'-' => "try_sub",
					_ => unreachable! (),
				};
				[
					// {
					//   let __val: Result <_, Overflow> =
					//     $right.and_then (|__right| $left.$fn_name (__right));
					//   if let Ok (__val) == __val {
					//     $left $punct_1 __val;
					//   } else {
					//     Err (Overflow)
					//   }
					// }
					TokenTree::Group (Group::new (Delimiter::Brace, [
						TokenTree::Ident (Ident::new ("let", Span::mixed_site ())),
						TokenTree::Ident (Ident::new ("__val", Span::mixed_site ())),
						TokenTree::Punct (Punct::new (':', Spacing::Alone)),
						TokenTree::Ident (Ident::new ("Result", Span::mixed_site ())),
						TokenTree::Punct (Punct::new ('<', Spacing::Alone)),
						TokenTree::Ident (Ident::new ("_", Span::mixed_site ())),
						TokenTree::Punct (Punct::new (',', Spacing::Alone)),
						TokenTree::Ident (Ident::new ("Overflow", Span::mixed_site ())),
						TokenTree::Punct (Punct::new ('>', Spacing::Alone)),
						TokenTree::Punct (Punct::new ('=', Spacing::Alone)),
						TokenTree::Group (Group::new (Delimiter::None, right.into ())),
						TokenTree::Punct (Punct::new ('.', Spacing::Alone)),
						TokenTree::Ident (Ident::new ("and_then", Span::mixed_site ())),
						TokenTree::Group (Group::new (Delimiter::Parenthesis, [
							TokenTree::Punct (Punct::new ('|', Spacing::Alone)),
							TokenTree::Ident (Ident::new ("__right", Span::mixed_site ())),
							TokenTree::Punct (Punct::new ('|', Spacing::Alone)),
							TokenTree::Group (Group::new (Delimiter::None, (& left).into ())),
							TokenTree::Punct (Punct::new ('.', Spacing::Alone)),
							TokenTree::Ident (Ident::new (fn_name, Span::mixed_site ())),
							TokenTree::Group (Group::new (Delimiter::Parenthesis, [
								TokenTree::Ident (Ident::new ("__right", Span::mixed_site ())),
							].into_iter ().collect ())),
						].into_iter ().collect ())),
						TokenTree::Punct (Punct::new (';', Spacing::Alone)),
						TokenTree::Ident (Ident::new ("if", Span::mixed_site ())),
						TokenTree::Ident (Ident::new ("let", Span::mixed_site ())),
						TokenTree::Ident (Ident::new ("Ok", Span::mixed_site ())),
						TokenTree::Group (Group::new (Delimiter::Parenthesis, [
							TokenTree::Ident (Ident::new ("__val", Span::mixed_site ())),
						].into_iter ().collect ())),
						TokenTree::Punct (Punct::new ('=', Spacing::Alone)),
						TokenTree::Ident (Ident::new ("__val", Span::mixed_site ())),
						TokenTree::Group (Group::new (Delimiter::Brace, iter::empty ()
							.chain (left.into_iter ())
							.chain ([
								TokenTree::Punct (punct_1),
								TokenTree::Ident (Ident::new ("__val", Span::mixed_site ())),
								TokenTree::Punct (Punct::new (';', Spacing::Alone)),
								TokenTree::Ident (Ident::new ("Ok", Span::mixed_site ())),
								TokenTree::Group (Group::new (Delimiter::Parenthesis, [
									TokenTree::Group (Group::new (Delimiter::Parenthesis,
										Self::new ())),
								].into_iter ().collect ())),
							].into_iter ())
							.collect ())),
						TokenTree::Ident (Ident::new ("else", Span::mixed_site ())),
						TokenTree::Group (Group::new (Delimiter::Brace, [
							TokenTree::Ident (Ident::new ("Err", Span::mixed_site ())),
							TokenTree::Group (Group::new (Delimiter::Parenthesis, [
								TokenTree::Ident (Ident::new ("Overflow", Span::mixed_site ())),
							].into_iter ().collect ())),
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
			let punct = match iter.peek ().unwrap () {
				& TokenTree::Punct (ref punct) => punct,
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
		let (fn_name, left, punct, right) = match * expr {
			CheckedExpr::Item (ref item) => {
				// Ok ($item)
				return [
					TokenTree::Ident (Ident::new ("Ok", Span::mixed_site ())),
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
			// $left.and_then (|__left| $right.and_then (|__right| __left.$fn_name (__right)))
			TokenTree::Group (Group::new (Delimiter::None, left.into ())),
			TokenTree::Punct (Punct::new ('.', Spacing::Alone)),
			TokenTree::Ident (Ident::new ("and_then", punct.span ())),
			TokenTree::Group (Group::new (Delimiter::Parenthesis, [
				TokenTree::Punct (Punct::new ('|', Spacing::Alone)),
				TokenTree::Ident (Ident::new ("__left", punct.span ())),
				TokenTree::Punct (Punct::new ('|', Spacing::Alone)),
				TokenTree::Group (Group::new (Delimiter::None, right.into ())),
				TokenTree::Punct (Punct::new ('.', Spacing::Alone)),
				TokenTree::Ident (Ident::new ("and_then", punct.span ())),
				TokenTree::Group (Group::new (Delimiter::Parenthesis, [
					TokenTree::Punct (Punct::new ('|', Spacing::Alone)),
					TokenTree::Ident (Ident::new ("__right", punct.span ())),
					TokenTree::Punct (Punct::new ('|', Spacing::Alone)),
					TokenTree::Ident (Ident::new ("__left", punct.span ())),
					TokenTree::Punct (Punct::new ('.', Spacing::Alone)),
					TokenTree::Ident (Ident::new (fn_name, punct.span ())),
					TokenTree::Group (Group::new (Delimiter::Parenthesis, [
						TokenTree::Ident (Ident::new ("__right", punct.span ())),
					].into_iter ().collect ())),
				].into_iter ().collect ())),
			].into_iter ().collect ())),
		].into_iter ().collect ()
	}
}

#[ derive (Clone, Debug) ]
struct CheckedItem (Vec <TokenTree>);

impl CheckedItem {
	fn parse (iter: & mut TokenIter) -> Self {
		let mut tokens = Vec::new ();
		loop {
			const ARITH_CHARS: [char; 5] = ['+', '-', '*', '/', '%'];
			match iter.peek () {
				Some (& TokenTree::Punct (ref punct))
						if ARITH_CHARS.contains (& punct.as_char ()) => {
					assert! (! tokens.is_empty ());
					return Self (tokens);
				},
				None => return Self (tokens),
				_ => tokens.push (iter.next ().unwrap ()),
			}
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

/*
#[ macro_export ]
macro_rules! checked {
	( $left:tt += $right:tt ) => {
		(|| { $left = $left.try_add (checked_eval! ($right) ?) ?; Ok::<_, Overflow> (()) }) ()
	};
	( $left:tt /= $right:tt ) => {
		(|| { $left = $left.try_div (checked_eval! ($right) ?) ?; Ok::<_, Overflow> (()) }) ()
	};
	( $left:tt *= $right:tt ) => {
		(|| { $left = $left.try_mul (checked_eval! ($right) ?) ?; Ok::<_, Overflow> (()) }) ()
	};
	( $left:tt %= $right:tt ) => {
		(|| { $left = $left.try_rem (checked_eval! ($right) ?) ?; Ok::<_, Overflow> (()) }) ()
	};
	( $left:tt -= $right:tt ) => {
		(|| { $left = $left.try_sub (checked_eval! ($right) ?) ?; Ok::<_, Overflow> (()) }) ()
	};
	( $($arg:tt)* ) => { (|| checked_eval! ($($arg)*)) () };
}

#[ macro_export ]
macro_rules! checked_eval {
	( $arg:ident ) => { Ok::<_, Overflow> ($arg) };
	( ( $($args:tt)* ) ) => { checked_eval! ($($args)*) };
	( $left:tt . $right:ident ) => { Ok::<_, Overflow> (checked_eval! ($left) ?.$right) };
	( $left_0:tt + $($right:tt)* ) => {
		checked_eval! ($left_0) ?.try_add (checked_eval! ($($right)*) ?)
	};
	( $left_0:tt / $($right:tt)* ) => {
		checked_eval! ($left_0) ?.try_div (checked_eval! ($($right)*) ?)
	};
	( $left_0:tt * $($right:tt)* ) => {
		checked_eval! ($left_0) ?.try_mul (checked_eval! ($($right)*) ?)
	};
	( $left_0:tt % $($right:tt)* ) => {
		checked_eval! ($left_0) ?.try_rem (checked_eval! ($($right)*) ?)
	};
	( $left_0:tt - $($right:tt)* ) => {
		checked_eval! ($left_0) ?.try_sub (checked_eval! ($($right)*) ?)
	};
}
*/
