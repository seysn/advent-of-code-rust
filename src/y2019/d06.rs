use std::collections::{HashMap, HashSet, VecDeque};

pub struct Map<'a> {
	orbits: HashMap<&'a str, HashSet<&'a str>>,
	r_orbits: HashMap<&'a str, HashSet<&'a str>>,
}

pub fn parse_input(input: &str) -> Map {
	let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut r_orbits: HashMap<&str, HashSet<&str>> = HashMap::new();

	for l in input.lines() {
		if let Some((k, v)) = l.split_once(')') {
			orbits.entry(k).or_default().insert(v);
			r_orbits.entry(v).or_default().insert(k);
		}
	}

	Map { orbits, r_orbits }
}

impl<'a> Map<'a> {
	fn count_orbits(&self, start: &str, parents: usize) -> usize {
		let mut res = parents;
		if let Some(set) = self.orbits.get(start) {
			for child in set {
				res += self.count_orbits(child, parents + 1);
			}
		}
		res
	}

	fn minimum_transfers(&self, start: &str, end: &str) -> usize {
		let mut seen: HashSet<&str> = HashSet::new();
		let mut queue = VecDeque::new();
		queue.push_back((start, 0));
		while let Some((p, s)) = queue.pop_front() {
			if p == end {
				return s - 2;
			}

			if let Some(set) = self.orbits.get(p) {
				for child in set {
					if seen.contains(child) {
						continue;
					}
					seen.insert(child);
					queue.push_back((child, s + 1));
				}
			}

			if let Some(set) = self.r_orbits.get(p) {
				for child in set {
					if seen.contains(child) {
						continue;
					}
					seen.insert(child);
					queue.push_back((child, s + 1));
				}
			}
		}

		unreachable!()
	}
}

pub fn part1(input: &Map) -> usize {
	input.count_orbits("COM", 0)
}

pub fn part2(input: &Map) -> usize {
	input.minimum_transfers("YOU", "SAN")
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
	const EXAMPLE2: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 42);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE2)), 4);
	}
}
