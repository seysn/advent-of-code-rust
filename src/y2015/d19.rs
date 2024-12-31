use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Machine {
	replacements: Vec<(String, String)>,
	molecules: String,
}

pub fn parse_input(input: &str) -> Machine {
	let (replacements, molecules) = input.split_once("\n\n").unwrap();

	Machine {
		replacements: replacements
			.lines()
			.map(|l| {
				let (l, r) = l.split_once(" => ").unwrap();
				(l.to_owned(), r.to_owned())
			})
			.collect(),
		molecules: molecules.to_owned(),
	}
}

pub fn part1(input: &Machine) -> usize {
	let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
	for (from, to) in &input.replacements {
		replacements.entry(from.clone()).or_default().push(to.clone());
	}

	let mut res: HashSet<String> = HashSet::new();
	for (from, rs) in &replacements {
		for to in rs {
			for (idx, s) in input.molecules.match_indices(from) {
				let mut new = String::new();
				new.push_str(input.molecules.get(..idx).unwrap());
				new.push_str(to);
				new.push_str(input.molecules.get(idx + s.len()..).unwrap());
				res.insert(new);
			}
		}
	}

	res.len()
}

pub fn part2(input: &Machine) -> usize {
	let map: HashMap<String, String> = input.replacements.iter().map(|(a, b)| (b.clone(), a.clone())).collect();

	let mut s = input.molecules.clone();
	let mut res = 0;
	while s != "e" {
		for (from, to) in &map {
			if to == "e" {
				if from == &s {
					return res + 1;
				} else {
					continue;
				}
			}

			if let Some((idx, found)) = s.match_indices(from).next() {
				let mut new = String::new();
				new.push_str(s.get(..idx).unwrap());
				new.push_str(to);
				new.push_str(s.get(idx + found.len()..).unwrap());
				s = new;
				res += 1;
				break;
			}
		}
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "H => HO
H => OH
O => HH
e => H
e => O

HOHOHO";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 7);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 6);
	}
}
