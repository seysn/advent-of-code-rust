pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_owned()).collect()
}

fn has_abba(s: &str) -> bool {
	for b in s.as_bytes().windows(4) {
		if b[0] == b[3] && b[1] == b[2] && b[0] != b[1] {
			return true;
		}
	}

	false
}

fn support_tls(s: &str) -> bool {
	let mut found = false;
	for (i, sub) in s.split(['[', ']']).enumerate() {
		if !found && i % 2 == 0 && has_abba(sub) {
			found = true;
		}

		if i % 2 != 0 && has_abba(sub) {
			return false;
		}
	}
	found
}

pub fn part1(input: &[String]) -> usize {
	input.iter().filter(|ip| support_tls(ip)).count()
}

fn find_babs(s: &str) -> Vec<String> {
	let mut babs = Vec::new();
	for b in s.as_bytes().windows(3) {
		if b[0] == b[2] && b[0] != b[1] {
			let bab = vec![b[1], b[0], b[1]];
			babs.push(String::from_utf8(bab).unwrap());
		}
	}

	babs
}

fn support_ssl(s: &str) -> bool {
	let mut inside = Vec::new();
	let mut outside = Vec::new();
	for (i, sub) in s.split(['[', ']']).enumerate() {
		if i % 2 == 0 {
			outside.push(sub.to_owned());
		} else {
			inside.push(sub.to_owned());
		}
	}

	for s in outside {
		for bab in find_babs(&s) {
			for ss in &inside {
				if ss.contains(&bab) {
					return true;
				}
			}
		}
	}

	false
}

pub fn part2(input: &[String]) -> usize {
	input.iter().filter(|ip| support_ssl(ip)).count()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert!(support_tls("abba[mnop]qrst"));
		assert!(!support_tls("abcd[bddb]xyyx"));
		assert!(!support_tls("aaaa[qwer]tyui"));
		assert!(support_tls("ioxxoj[asdfgh]zxcvbn"));
	}

	#[test]
	fn example_part2() {
		assert!(support_ssl("aba[bab]xyz"));
		assert!(!support_ssl("xyx[xyx]xyx"));
		assert!(support_ssl("aaa[kek]eke"));
		assert!(support_ssl("zazbz[bzb]cdb"));
	}
}
