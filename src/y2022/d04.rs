use std::ops::RangeInclusive;

fn parse_range(range: &str) -> RangeInclusive<i32> {
	let mut splited = range.split('-');
	let min = splited.next().unwrap().parse().unwrap();
	let max = splited.next().unwrap().parse().unwrap();
	min..=max
}

pub fn parse_input(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
	let mut v = vec![];
	for line in input.lines() {
		let mut splited = line.split(',');
		let fst = splited.next().unwrap();
		let snd = splited.next().unwrap();
		v.push((parse_range(fst), parse_range(snd)));
	}
	v
}

pub fn part1(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> usize {
	input
		.iter()
		.filter(|(fst, snd)| {
			(fst.start() <= snd.start() && fst.end() >= snd.end()) || (fst.start() >= snd.start() && fst.end() <= snd.end())
		})
		.count()
}

pub fn part2(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> usize {
	input
		.iter()
		.filter(|(fst, snd)| (fst.end() >= snd.start() && fst.end() <= snd.end()) || (snd.end() >= fst.start() && snd.end() <= fst.end()))
		.count()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 2);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 4);
	}
}
