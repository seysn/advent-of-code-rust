use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<u64> {
	input.lines().map(|l| l.parse().unwrap()).collect()
}

fn solve(input: &[u64], groups: u64) -> u64 {
	let group_weight = input.iter().sum::<u64>() / groups;

	for i in 2..input.len() - 1 {
		let mut min = None;
		for v in input.iter().combinations(i) {
			let sum: u64 = v.iter().cloned().sum();
			if sum == group_weight {
				let product: u64 = v.iter().cloned().product();
				min = Some(min.unwrap_or(u64::MAX).min(product));
			}
		}

		if let Some(res) = min {
			return res;
		}
	}

	0
}

pub fn part1(input: &[u64]) -> u64 {
	solve(input, 3)
}

pub fn part2(input: &[u64]) -> u64 {
	solve(input, 4)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 99);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 44);
	}
}
