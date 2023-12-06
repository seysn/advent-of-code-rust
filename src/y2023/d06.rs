pub struct Races {
	times: Vec<u64>,
	distances: Vec<u64>,
}

pub fn parse_input(input: &str) -> Races {
	let mut it = input.lines();
	let times = it
		.next()
		.unwrap()
		.split(' ')
		.filter(|s| !s.is_empty())
		.skip(1)
		.map(|s| s.parse::<u64>().unwrap())
		.collect();
	let distances = it
		.next()
		.unwrap()
		.split(' ')
		.filter(|s| !s.is_empty())
		.skip(1)
		.map(|s| s.parse::<u64>().unwrap())
		.collect();

	Races { times, distances }
}

fn inequality(time: u64, distance: u64, x: u64) -> bool {
	(time - x) * x > distance
}

fn n_ways(time: u64, distance: u64) -> usize {
	let start = (0..=time).find(|x| inequality(time, distance, *x)).unwrap();
	let end = (0..=time).rev().find(|x| inequality(time, distance, *x)).unwrap();

	(end - start + 1) as usize
}

pub fn part1(input: &Races) -> usize {
	input
		.times
		.iter()
		.zip(&input.distances)
		.map(|(time, distance)| n_ways(*time, *distance))
		.product()
}

fn join_ints(input: &[u64]) -> u64 {
	input
		.iter()
		.map(|t| t.to_string())
		.fold(String::new(), |a, b| a + &b)
		.parse::<u64>()
		.unwrap()
}

pub fn part2(input: &Races) -> usize {
	n_ways(join_ints(&input.times), join_ints(&input.distances))
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 288);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 71503);
	}
}
