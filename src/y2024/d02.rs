#[derive(Debug)]
enum Level {
	Increase(u64),
	Decrease(u64),
}

impl Level {
	fn new(a: u64, b: u64) -> Self {
		if a < b {
			Self::Increase(b - a)
		} else {
			Self::Decrease(a - b)
		}
	}

	fn correct_value(&self) -> bool {
		match self {
			Level::Increase(v) | Level::Decrease(v) => *v >= 1 && *v <= 3,
		}
	}
}

impl PartialEq for Level {
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(Level::Increase(_), Level::Increase(_)) | (Level::Decrease(_), Level::Decrease(_))
		)
	}
}

pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
	input.lines().map(|l| l.split(' ').map(|i| i.parse().unwrap()).collect()).collect()
}

fn solve(l: &[u64]) -> bool {
	l.windows(2)
		.map(|n| Level::new(n[0], n[1]))
		.collect::<Vec<Level>>()
		.windows(2)
		.all(|v| v[0] == v[1] && v[0].correct_value() && v[1].correct_value())
}

pub fn part1(input: &[Vec<u64>]) -> usize {
	input.iter().filter(|l| solve(l)).count()
}

pub fn part2(input: &[Vec<u64>]) -> usize {
	input
		.iter()
		.filter(|l| {
			for i in 0..l.len() {
				let mut v = Vec::new();
				v.extend(&l[..i]);
				v.extend(&l[i + 1..]);
				if solve(&v) {
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

	const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 2);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 4);
	}
}
