use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
	winning_numbers: Vec<u32>,
	numbers: Vec<u32>,
}

pub fn parse_input(input: &str) -> Vec<Card> {
	input
		.lines()
		.map(|l| {
			let st = l.find(':').unwrap();
			let mut splited = l[st + 2..].split(" | ");

			let winning_numbers: Vec<u32> = splited
				.next()
				.unwrap()
				.split(' ')
				.filter(|s| !s.is_empty())
				.map(|n| n.parse::<u32>().unwrap())
				.collect();

			let numbers: Vec<u32> = splited
				.next()
				.unwrap()
				.split(' ')
				.filter(|s| !s.is_empty())
				.map(|n| n.parse::<u32>().unwrap())
				.collect();

			Card { winning_numbers, numbers }
		})
		.collect()
}

pub fn part1(input: &[Card]) -> u32 {
	input
		.iter()
		.map(|card| {
			let winning_numbers: HashSet<u32> = HashSet::from_iter(card.winning_numbers.clone());
			let numbers: HashSet<u32> = HashSet::from_iter(card.numbers.clone());

			let count = winning_numbers.intersection(&numbers).count() as u32;

			if count == 0 {
				0
			} else {
				2_u32.pow(count - 1)
			}
		})
		.sum()
}

pub fn part2(input: &[Card]) -> usize {
	let mut res = 0;
	let mut cards = vec![1_usize; input.len()];
	let mut i = 0;
	while i < input.len() {
		let winning_numbers: HashSet<u32> = HashSet::from_iter(input[i].winning_numbers.clone());
		let numbers: HashSet<u32> = HashSet::from_iter(input[i].numbers.clone());

		let count = winning_numbers.intersection(&numbers).count();
		for j in i + 1..(i + 1 + count) {
			cards[j] += cards[i];
		}

		res += cards[i];
		i += 1;
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 13);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 30);
	}
}
