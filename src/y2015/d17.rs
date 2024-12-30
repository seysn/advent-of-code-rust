use std::cmp::Ordering;

pub fn parse_input(input: &str) -> Vec<u32> {
	input.lines().filter_map(|l| l.parse().ok()).collect()
}

fn solve(containers: &[u32], liters: u32) -> Vec<Vec<u32>> {
	if containers.is_empty() {
		return vec![vec![]];
	}

	let container = containers[0];
	match container.cmp(&liters) {
		Ordering::Less => {
			let mut res = Vec::new();

			for mut v in solve(&containers[1..], liters - container) {
				if v.iter().sum::<u32>() == liters - container {
					v.insert(0, container);
					res.push(v);
				}
			}
			for v in solve(&containers[1..], liters) {
				if v.iter().sum::<u32>() == liters {
					res.push(v);
				}
			}

			res
		}
		Ordering::Equal => {
			let mut res = vec![vec![container]];

			for v in solve(&containers[1..], liters) {
				if v.iter().sum::<u32>() == liters {
					res.push(v);
				}
			}

			res
		}
		Ordering::Greater => {
			let mut res = Vec::new();
			for v in solve(&containers[1..], liters) {
				if v.iter().sum::<u32>() == liters {
					res.push(v);
				}
			}

			res
		}
	}
}

pub fn part1(input: &[u32]) -> usize {
	solve(input, 150).len()
}

pub fn part2(input: &[u32]) -> usize {
	let res = solve(input, 150);
	let min = res.iter().map(|v| v.len()).min().unwrap();
	res.iter().filter(|v| v.len() == min).count()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "20
15
10
5
5";

	#[test]
	fn example_part1() {
		assert_eq!(solve(&parse_input(EXAMPLE), 25).len(), 4);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 0);
	}
}
