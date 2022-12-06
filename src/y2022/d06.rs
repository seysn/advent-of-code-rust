use std::collections::HashSet;

pub fn parse_input(input: &str) -> String {
	input.to_string()
}

fn find_start(input: &str, n_diff: usize) -> usize {
	for (i, c) in input.chars().collect::<Vec<char>>().windows(n_diff).enumerate() {
		let mut unique: HashSet<char> = HashSet::new();
		unique.extend(c);
		if unique.len() == n_diff {
			return i + n_diff;
		}
	}

	0
}

pub fn part1(input: &str) -> usize {
	find_start(input, 4)
}

pub fn part2(input: &str) -> usize {
	find_start(input, 14)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
		assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
		assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
		assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
		assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
		assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
		assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 23);
		assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
		assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
	}
}
