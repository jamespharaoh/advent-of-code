fn main () {
	let mut num_matches: u64 = 0;
	for password in 271973 ..= 785961 {
		let password = format! ("{}", password);
		if password_matches (& password) {
			num_matches += 1;
		}
	}
	println! ("Num matches: {}", num_matches);
}

fn password_matches (password: & str) -> bool {
	let mut last_ch: Option <char> = None;
	let mut dupe = false;
	let mut dupe_count: usize = 1;
	for ch in password.chars () {
		if let Some (last_ch) = last_ch {
			if last_ch == ch {
				dupe_count += 1;
			} else {
				if dupe_count == 2 { dupe = true; }
				dupe_count = 1;
			}
			if ch < last_ch { return false; }
		}
		last_ch = Some (ch);
	}
	if dupe_count == 2 { dupe = true; }
	dupe
}
