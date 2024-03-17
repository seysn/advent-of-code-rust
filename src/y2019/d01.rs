pub fn parse_input(input: &str) -> Vec<i32> {
	input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &[i32]) -> i32 {
	input.iter().map(|i| i / 3 - 2).sum()
}

pub fn part2(input: &[i32]) -> i32 {
	input
		.iter()
		.map(|&i| {
			let mut res = 0;
			let mut current = i;
			loop {
				current = current / 3 - 2;
				if current <= 0 {
					break;
				}
				res += current;
			}
			res
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(part1(&[12]), 2);
		assert_eq!(part1(&[14]), 2);
		assert_eq!(part1(&[1969]), 654);
		assert_eq!(part1(&[100756]), 33583);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&[14]), 2);
		assert_eq!(part2(&[1969]), 966);
		assert_eq!(part2(&[100756]), 50346);
	}
}
