use std::collections::HashMap;

use itertools::Itertools;

use crate::collections::Point;

pub fn parse_input(input: &str) -> Vec<String> {
	input.lines().map(|l| l.to_owned()).collect()
}

struct Keypad {
	points: HashMap<char, Point>,
	cache: HashMap<(char, char), u64>,
}

impl Keypad {
	fn new_numeric() -> Self {
		Self {
			points: [
				('A', Point(0, 0)),
				('0', Point(-1, 0)),
				('1', Point(-2, -1)),
				('2', Point(-1, -1)),
				('3', Point(0, -1)),
				('4', Point(-2, -2)),
				('5', Point(-1, -2)),
				('6', Point(0, -2)),
				('7', Point(-2, -3)),
				('8', Point(-1, -3)),
				('9', Point(0, -3)),
			]
			.into_iter()
			.collect(),
			cache: HashMap::new(),
		}
	}

	fn new_directional() -> Self {
		Self {
			points: [
				('A', Point(0, 0)),
				('^', Point(-1, 0)),
				('<', Point(-2, 1)),
				('v', Point(-1, 1)),
				('>', Point(0, 1)),
			]
			.into_iter()
			.collect(),
			cache: HashMap::new(),
		}
	}

	/// Quite ugly, but always gets the best path for both keypads
	/// Just changes the preference of direction based on if path is going to pass
	/// through the empty gap.
	fn sequence(&self, from: char, to: char) -> String {
		let mut s = String::new();
		let current = self.points.get(&from).unwrap();
		let point = self.points.get(&to).unwrap();
		let v = current.vector(point);

		if current.1 == 0 && current.0 + v.0 == -2 {
			if v.1 > 0 {
				for _ in 0..v.1 {
					s.push('v');
				}
			}

			if v.1 < 0 {
				for _ in 0..v.1.abs() {
					s.push('^');
				}
			}

			if v.0 < 0 {
				for _ in 0..v.0.abs() {
					s.push('<');
				}
			}
		} else if current.0 == -2 && current.1 + v.1 == 0 {
			if v.0 < 0 {
				for _ in 0..v.0.abs() {
					s.push('<');
				}
			}

			if v.0 > 0 {
				for _ in 0..v.0 {
					s.push('>');
				}
			}

			if v.1 > 0 {
				for _ in 0..v.1 {
					s.push('v');
				}
			}

			if v.1 < 0 {
				for _ in 0..v.1.abs() {
					s.push('^');
				}
			}
		} else {
			if v.0 < 0 {
				for _ in 0..v.0.abs() {
					s.push('<');
				}
			}

			if v.1 > 0 {
				for _ in 0..v.1 {
					s.push('v');
				}
			}

			if v.1 < 0 {
				for _ in 0..v.1.abs() {
					s.push('^');
				}
			}

			if v.0 > 0 {
				for _ in 0..v.0 {
					s.push('>');
				}
			}
		}

		s.push('A');
		s
	}

	fn sequence_full(&self, code: &str) -> String {
		"A".chars()
			.chain(code.chars())
			.tuple_windows()
			.map(|(a, b)| self.sequence(a, b))
			.collect()
	}
}

fn compute_length(code: &str, keypads: &mut [Keypad]) -> u64 {
	if keypads.len() == 1 {
		return keypads[0].sequence_full(code).len() as u64;
	}

	let mut length = 0;
	for (a, b) in "A".chars().chain(code.chars()).tuple_windows() {
		if let Some(l) = keypads[0].cache.get(&(a, b)) {
			length += l;
		} else {
			let sub = keypads[0].sequence(a, b);
			let l = compute_length(&sub, &mut keypads[1..]);
			keypads[0].cache.insert((a, b), l);
			length += l;
		}
	}

	length
}

fn solve(input: &[String], n: u64) -> u64 {
	let numeric = Keypad::new_numeric();
	let mut keypads = Vec::new();
	for _ in 0..n {
		keypads.push(Keypad::new_directional());
	}

	let mut res = 0;
	for code in input {
		let subcode = numeric.sequence_full(code);
		let length = compute_length(&subcode, &mut keypads);
		let number: u64 = code.chars().filter(char::is_ascii_digit).collect::<String>().parse().unwrap();
		res += number * length;
	}

	res
}

pub fn part1(input: &[String]) -> u64 {
	solve(input, 2)
}

pub fn part2(input: &[String]) -> u64 {
	solve(input, 25)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "029A
980A
179A
456A
379A";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 126384);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 154115708116294);
	}
}
