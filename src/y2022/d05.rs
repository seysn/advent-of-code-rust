use std::collections::VecDeque;

use regex::Regex;

#[derive(Debug)]
pub struct Cargo {
	stacks: [VecDeque<char>; 9],
	moves: Vec<(usize, usize, usize)>,
}

pub fn parse_input(input: &str) -> Cargo {
	let mut stacks = [(); 9].map(|_| VecDeque::new());
	let mut moves = Vec::new();
	let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

	for line in input.lines() {
		if line.starts_with("move") {
			let cap = re.captures(line).unwrap();
			moves.push((cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()))
		} else if !line.starts_with(" 1") {
			for (i, n) in (1..line.len()).step_by(4).enumerate() {
				match line.chars().nth(n) {
					Some(' ') => continue,
					Some(c) => stacks[i].push_front(c),
					_ => continue,
				}
			}
		}
	}

	Cargo { stacks, moves }
}

pub fn top_string(stacks: &[VecDeque<char>]) -> String {
	stacks.iter().filter(|s| !s.is_empty()).map(|s| s.back().unwrap()).collect()
}

pub fn part1(input: &Cargo) -> String {
	let mut stacks = input.stacks.clone();

	for &(times, from, to) in input.moves.iter() {
		for _ in 0..times {
			let c = stacks[from - 1].pop_back().unwrap();
			stacks[to - 1].push_back(c);
		}
	}

	top_string(&stacks)
}

pub fn part2(input: &Cargo) -> String {
	let mut stacks = input.stacks.clone();

	for &(times, from, to) in input.moves.iter() {
		let mut v = Vec::new();
		for _ in 0..times {
			v.push(stacks[from - 1].pop_back().unwrap());
		}
		for &c in v.iter().rev() {
			stacks[to - 1].push_back(c);
		}
	}

	top_string(&stacks)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), String::from("CMZ"));
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), String::from("MCD"));
	}
}
