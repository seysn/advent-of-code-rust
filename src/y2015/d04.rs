use md5;

pub fn parse_input(input: &str) -> String {
	String::from(input)
}

fn md5_starts_with(input: &str, pat: &str) -> u32 {
	for i in u32::MIN..u32::MAX {
		let digest = format!("{}{}", input, i);
		let result = format!("{:x}", md5::compute(digest.as_bytes()));
		if result.starts_with(pat) {
			return i;
		}
	}
	unreachable!();
}

pub fn part1(input: &str) -> u32 {
	md5_starts_with(input, "00000")
}

pub fn part2(input: &str) -> u32 {
	md5_starts_with(input, "000000")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input("abcdef")), 609043);
		assert_eq!(part1(&parse_input("pqrstuv")), 1048970);
	}
}
