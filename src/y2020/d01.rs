pub fn parse_input(input: &str) -> Vec<i32> {
	input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &[i32]) -> i32 {
	for i in input.iter() {
		if input.iter().any(|&x| x == (2020 - i)) {
			return (2020 - i) * i;
		}
	}
	unreachable!();
}

pub fn part2(input: &[i32]) -> i32 {
	for i in input.iter() {
		let candidates: Vec<&i32> = input.iter().filter(|&x| x < &(2020 - i) && x != i).collect();

		for &j in candidates.iter() {
			if input.iter().any(|&x| x == (2020 - i - j)) {
				return (2020 - i - j) * j * i;
			}
		}
	}
	unreachable!();
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "1721
979
366
299
675
1456";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 514579);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 241861950);
	}
}
