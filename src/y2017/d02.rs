pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
	input
		.lines()
		.map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect())
		.collect()
}

pub fn part1(input: &[Vec<u32>]) -> u32 {
	let mut res = 0;

	for row in input {
		let mut smallest = u32::MAX;
		let mut largest = u32::MIN;

		for &n in row {
			if n < smallest {
				smallest = n;
			}

			if n > largest {
				largest = n;
			}
		}

		res += largest - smallest;
	}

	res
}

pub fn part2(input: &[Vec<u32>]) -> u32 {
	let mut res = 0;

	for row in input {
		for (i, &a) in row.iter().take(row.len() - 1).enumerate() {
			for &b in &row[i + 1..] {
				if a % b == 0 {
					res += a / b;
					break;
				}

				if b % a == 0 {
					res += b / a;
					break;
				}
			}
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		const EXAMPLE: &str = "5 1 9 5\n7 5 3\n2 4 6 8";
		assert_eq!(part1(&parse_input(EXAMPLE)), 18);
	}

	#[test]
	fn example_part2() {
		const EXAMPLE: &str = "5 9 2 8\n9 4 7 3\n3 8 6 5";
		assert_eq!(part2(&parse_input(EXAMPLE)), 9);
	}
}
