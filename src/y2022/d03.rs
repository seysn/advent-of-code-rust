use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_string()).collect()
}

fn priority_char(c: char) -> i32 {
	match c {
		'a'..='z' => c as i32 - 96,
		'A'..='Z' => c as i32 - 38,
		_ => panic!("no"),
	}
}

pub fn part1(input: &[String]) -> i32 {
	let mut res = 0;
	for rucksack in input {
		let (fst, snd) = (&rucksack[..rucksack.len() / 2], &rucksack[rucksack.len() / 2..]);
		for c in fst.chars() {
			if snd.contains(c) {
				res += priority_char(c);
				break;
			}
		}
	}

	res
}

fn find_common(rucksacks: &[String]) -> char {
	let mut candidates: HashSet<char> = HashSet::new();
	candidates.extend(rucksacks[0].chars());
	for rucksack in &rucksacks[1..] {
		for candidate in candidates.clone() {
			if !rucksack.contains(candidate) {
				candidates.remove(&candidate);
			}
		}
	}

	assert!(candidates.len() == 1);
	*candidates.iter().next().unwrap()
}

pub fn part2(input: &[String]) -> i32 {
	input.chunks(3).map(|group| priority_char(find_common(group))).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 157);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 70);
	}
}
