pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
	input
		.lines()
		.map(|l| l.split(' ').map(|i| i.parse::<i64>().unwrap()).collect())
		.collect()
}

fn find_last(seq: &[i64]) -> i64 {
	let next: Vec<i64> = seq.iter().zip(seq.iter().skip(1)).map(|(a, b)| b - a).collect();
	seq.last().unwrap() + if next.iter().all(|&e| e == 0) { 0 } else { find_last(&next) }
}

fn find_first(seq: &[i64]) -> i64 {
	let next: Vec<i64> = seq.iter().zip(seq.iter().skip(1)).map(|(a, b)| b - a).collect();
	seq.first().unwrap() - if next.iter().all(|&e| e == 0) { 0 } else { find_first(&next) }
}

pub fn part1(input: &[Vec<i64>]) -> i64 {
	input.iter().map(|seq| find_last(seq)).sum()
}

pub fn part2(input: &[Vec<i64>]) -> i64 {
	input.iter().map(|seq| find_first(seq)).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 114);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 2);
	}
}
