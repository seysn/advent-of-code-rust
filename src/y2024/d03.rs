use regex::Regex;

pub fn parse_input(input: &str) -> String {
	input.to_owned()
}

fn solve(input: &str) -> u64 {
	let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

	re.captures_iter(input)
		.map(|cap| cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap())
		.sum()
}

pub fn part1(input: &str) -> u64 {
	solve(input)
}

pub fn part2(input: &str) -> u64 {
	let mut catches = true;
	let mut inp = input;

	let mut res = 0;
	while !inp.is_empty() {
		if catches {
			if let Some(idx) = inp.find("don't()") {
				res += solve(&inp[..idx]);
				inp = &inp[idx + 7..];
				catches = false;
			} else {
				res += solve(inp);
				break;
			}
		} else if let Some(idx) = inp.find("do()") {
			inp = &inp[idx + 4..];
			catches = true;
		} else {
			break;
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(
			part1(&parse_input(
				"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
			)),
			161
		);
	}

	#[test]
	fn example_part2() {
		assert_eq!(
			part2(&parse_input(
				"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
			)),
			48
		);
	}
}
