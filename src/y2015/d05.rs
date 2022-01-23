use regex::Regex;

const VOWELS: &[char; 5] = &['a', 'e', 'i', 'o', 'u'];

pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_string()).collect()
}

pub fn part1(input: &[String]) -> usize {
	let re = Regex::new("(ab|cd|pq|xy)").unwrap();
	input
		.iter()
		.filter(|&string| {
			let count_vowels = string.chars().filter(|c| VOWELS.contains(c)).count();
			if count_vowels < 3 {
				return false;
			}

			if re.is_match(string) {
				return false;
			}

			for (a, b) in string.chars().zip(string.chars().skip(1)) {
				if a == b {
					return true;
				}
			}

			false
		})
		.count()
}

pub fn part2(input: &[String]) -> usize {
	input
		.iter()
		.filter(|&string| {
			let mut res = false;
			for chars in string.chars().collect::<Vec<char>>().windows(2) {
				if string.matches(&String::from_iter(chars)).collect::<Vec<&str>>().len() > 1 {
					res = true;
					break;
				}
			}

			if !res {
				return false;
			}

			for (a, b) in string.chars().zip(string.chars().skip(2)) {
				if a == b {
					return true;
				}
			}

			false
		})
		.count()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input("ugknbfddgicrmopn")), 1);
		assert_eq!(part1(&parse_input("aaa")), 1);
		assert_eq!(part1(&parse_input("jchzalrnumimnmhp")), 0);
		assert_eq!(part1(&parse_input("haegwjzuvuyypxyu")), 0);
		assert_eq!(part1(&parse_input("dvszwmarrgswjxmb")), 0);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input("qjhvhtzxzqqjkmpb")), 1);
		assert_eq!(part2(&parse_input("xxyxx")), 1);
		assert_eq!(part2(&parse_input("uurcxstgmygtbstg")), 0);
		assert_eq!(part2(&parse_input("ieodomkazucvgmuy")), 0);
	}
}
