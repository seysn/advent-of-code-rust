use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
	input.split("\n\n").map(|elf| elf.lines().map(|l| l.parse().unwrap()).collect()).collect()
}

pub fn part1(input: &[Vec<i32>]) -> i32 {
	input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

pub fn part2(input: &[Vec<i32>]) -> i32 {
	input.iter().map(|elf| elf.iter().sum()).sorted_by(|a: &i32, b: &i32| b.cmp(a)).take(3).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 24000);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 45000);
	}
}
