use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
	rules: Vec<(u32, u32)>,
	pages: Vec<Vec<u32>>,
}

pub fn parse_input(input: &str) -> Input {
	let (rules, pages) = input.split_once("\n\n").unwrap();

	Input {
		rules: rules
			.lines()
			.map(|l| {
				let (a, b) = l.split_once('|').unwrap();
				(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
			})
			.collect(),
		pages: pages
			.lines()
			.map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
			.collect(),
	}
}

pub fn part1(input: &Input) -> u32 {
	let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
	for (k, v) in &input.rules {
		rules.entry(*k).or_default().push(*v);
	}

	input
		.pages
		.iter()
		.filter_map(|p| {
			for (i, v) in p.iter().enumerate() {
				if let Some(vec) = rules.get(v) {
					for vv in vec {
						if p[..i].contains(vv) {
							return None;
						}
					}
				}
			}

			Some(p[p.len() / 2])
		})
		.sum()
}

pub fn part2(input: &Input) -> u32 {
	let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
	for (k, v) in &input.rules {
		rules.entry(*k).or_default().push(*v);
	}

	let mut reversed: HashMap<u32, Vec<u32>> = HashMap::new();
	for (k, v) in &input.rules {
		reversed.entry(*v).or_default().push(*k);
	}

	input
		.pages
		.iter()
		.filter_map(|p| {
			let mut modified = false;
			let mut sorted = p.clone();

			loop {
				let mut swapped = false;
				for i in 0..p.len() {
					let v = &sorted[i];
					if let Some(vec) = reversed.get(v) {
						for vv in vec {
							if let Some(idx) = sorted[i + 1..].iter().position(|a| a == vv) {
								modified = true;
								swapped = true;

								sorted.swap(i, i + 1 + idx);
							}
						}
					}
				}

				if !swapped {
					break;
				}
			}

			if modified {
				Some(sorted[p.len() / 2])
			} else {
				None
			}
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 143);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 123);
	}
}
