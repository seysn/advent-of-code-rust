use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
struct Card {
	label: char,
}

#[derive(Clone)]
pub struct Hand {
	cards: Vec<Card>,
	bid: u64,
}

impl Card {
	fn value(&self) -> u64 {
		match self.label {
			'A' => 13,
			'K' => 12,
			'Q' => 11,
			'J' => 10,
			'T' => 9,
			'9' => 8,
			'8' => 7,
			'7' => 6,
			'6' => 5,
			'5' => 4,
			'4' => 3,
			'3' => 2,
			'2' => 1,
			_ => 0,
		}
	}

	fn value_joker(&self) -> u64 {
		match self.label {
			'A' => 13,
			'K' => 12,
			'Q' => 11,
			'T' => 10,
			'9' => 9,
			'8' => 8,
			'7' => 7,
			'6' => 6,
			'5' => 5,
			'4' => 4,
			'3' => 3,
			'2' => 2,
			'J' => 1,
			_ => 0,
		}
	}

	fn is_joker(&self) -> bool {
		matches!(self.label, 'J')
	}
}

impl Hash for Card {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.label.hash(state);
	}
}

impl Eq for Card {}

impl PartialEq for Card {
	fn eq(&self, other: &Self) -> bool {
		self.value() == other.value()
	}
}

impl Hand {
	fn strength(&self) -> u64 {
		let mut map: HashMap<Card, usize> = HashMap::new();
		for c in &self.cards {
			map.entry(c.clone()).and_modify(|cpt| *cpt += 1).or_insert(1);
		}

		let max = *map.values().max().unwrap();
		match map.len() {
			1 => 6,
			2 => {
				if max == 4 {
					5
				} else {
					4
				}
			}
			3 => {
				if max == 3 {
					3
				} else {
					2
				}
			}
			4 => 1,
			_ => 0,
		}
	}

	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.strength() == other.strength() {
			for (a, b) in self.cards.iter().zip(&other.cards) {
				if a.value() != b.value() {
					return a.value().cmp(&b.value());
				}
			}
		}

		self.strength().cmp(&other.strength())
	}

	fn strength_joker(&self) -> u64 {
		let mut map: HashMap<Card, usize> = HashMap::new();
		let mut jokers = 0;
		for c in &self.cards {
			if c.is_joker() {
				jokers += 1;
			} else {
				map.entry(c.clone()).and_modify(|cpt| *cpt += 1).or_insert(1);
			}
		}

		if jokers == 5 {
			map.entry(Card { label: 'A' }).or_insert(1);
			jokers = 4;
		}

		let mut max = *map.values().max().unwrap();
		*map.values_mut().find(|v| **v == max).unwrap() += jokers;
		max += jokers;

		match map.len() {
			1 => 6,
			2 => {
				if max == 4 {
					5
				} else {
					4
				}
			}
			3 => {
				if max == 3 {
					3
				} else {
					2
				}
			}
			4 => 1,
			_ => 0,
		}
	}

	fn cmp_joker(&self, other: &Self) -> std::cmp::Ordering {
		let res = self.strength_joker().cmp(&other.strength_joker());

		if res.is_eq() {
			for (a, b) in self.cards.iter().zip(&other.cards) {
				if a.value_joker() != b.value_joker() {
					return a.value_joker().cmp(&b.value_joker());
				}
			}
		}

		res
	}
}

pub fn parse_input(input: &str) -> Vec<Hand> {
	input
		.lines()
		.map(|l| {
			let mut splited = l.split(' ');
			let cards = splited.next().unwrap().chars().map(|label| Card { label }).collect();
			let bid = splited.next().unwrap().parse().unwrap();
			Hand { cards, bid }
		})
		.collect()
}

fn solve<F>(input: &[Hand], compare: F) -> u64
where
	F: FnMut(&Hand, &Hand) -> std::cmp::Ordering,
{
	let mut hands = input.to_vec();
	hands.sort_by(compare);

	hands.iter().enumerate().map(|(i, hand)| (i + 1) as u64 * hand.bid).sum()
}

pub fn part1(input: &[Hand]) -> u64 {
	solve(input, |a, b| a.cmp(b))
}

pub fn part2(input: &[Hand]) -> u64 {
	solve(input, |a, b| a.cmp_joker(b))
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 6440);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 5905);
	}
}
