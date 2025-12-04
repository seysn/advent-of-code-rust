#[derive(Debug)]
pub struct Equation {
	result: u64,
	values: Vec<u64>,
}

pub fn parse_input(input: &str) -> Vec<Equation> {
	input
		.lines()
		.map(|l| {
			let (result, values) = l.split_once(": ").unwrap();

			Equation {
				result: result.parse().unwrap(),
				values: values.split(' ').map(|v| v.parse().unwrap()).collect(),
			}
		})
		.collect()
}

fn solve(input: &[Equation], concatenation: bool) -> u64 {
	let mut res = 0;
	for eq in input {
		let mut results = vec![eq.values[0]];

		for v in &eq.values[1..] {
			let mut new_results = Vec::new();
			while let Some(r) = results.pop() {
				new_results.push(r + *v);
				new_results.push(r * *v);
				if concatenation {
					new_results.push(r * 10_u64.pow(v.ilog10() + 1) + *v);
				}
			}
			results = new_results;
		}

		if results.contains(&eq.result) {
			res += eq.result;
		}
	}

	res
}

pub fn part1(input: &[Equation]) -> u64 {
	solve(input, false)
}

pub fn part2(input: &[Equation]) -> u64 {
	solve(input, true)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 3749);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 11387);
	}
}
