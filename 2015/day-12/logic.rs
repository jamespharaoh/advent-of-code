use super::*;

use input::Input;
use model::Json;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	Ok (calc_sum (& input.json, |_| true) ?)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	Ok (calc_sum (& input.json, filter_red) ?)
}

fn filter_red (value: & Json) -> bool {
	let items = if let Json::Object (ref items) = * value { items } else { return true };
	! items.iter ().any (|& (_, ref value)| {
		matches! (* value, Json::String (ref value) if value == "red")
	})
}

fn calc_sum (value: & Json, filter: fn (& Json) -> bool) -> NumResult <Val> {
	if ! filter (value) { return Ok (Val::ZERO) }
	match * value {
		Json::Array (ref items) => items.iter ()
			.try_fold (Val::ZERO, |sum, item| {
				let item = calc_sum (item, filter) ?;
				Ok (chk! (sum + item) ?)
			}),
		Json::Object (ref items) => items.iter ()
			.try_fold (Val::ZERO, |sum, & (_, ref item)| {
				let item = calc_sum (item, filter) ?;
				Ok (chk! (sum + item) ?)
			}),
		Json::Number (value) => Ok (value),
		Json::String (_) => Ok (Val::ZERO),
	}
}

#[ cfg (test) ]
mod test {

	use super::*;

	fn pj (src: & str) -> Json {
		Json::parse_from_str (src).unwrap ()
	}

	#[ test ]
	fn filter_red () {
		assert! (super::filter_red (& pj ("{\"abc\":[],\"red\":1}")));
		assert! (super::filter_red (& pj ("[{\"abc\":[],\"def\":\"red\"}]")));
		assert! (super::filter_red (& pj ("{\"key\":{\"abc\":[],\"def\":\"red\"}}")));
		assert! (! super::filter_red (& pj ("{\"abc\":[],\"def\":\"red\"}")));
	}

}
