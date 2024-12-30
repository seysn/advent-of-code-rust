use std::collections::HashMap;

#[derive(Debug)]
pub struct Aunt {
	compounds: HashMap<String, u32>,
}

pub fn parse_input(input: &str) -> Vec<Aunt> {
	input
		.lines()
		.map(|l| {
			let (_, compounds) = l.split_once(": ").unwrap();

			Aunt {
				compounds: compounds
					.split(", ")
					.map(|s| {
						let (compound, n) = s.split_once(": ").unwrap();
						(compound.to_owned(), n.parse().unwrap())
					})
					.collect(),
			}
		})
		.collect()
}

pub fn part1(input: &[Aunt]) -> usize {
	let tape: HashMap<&str, u32> = [
		("children", 3),
		("cats", 7),
		("samoyeds", 2),
		("pomeranians", 3),
		("akitas", 0),
		("vizslas", 0),
		("goldfish", 5),
		("trees", 3),
		("cars", 2),
		("perfumes", 1),
	]
	.into_iter()
	.collect();

	'outer: for (i, aunt) in input.iter().enumerate() {
		for (compound, v) in &aunt.compounds {
			if tape.get(compound.as_str()) != Some(v) {
				continue 'outer;
			}
		}

		return i + 1;
	}

	0
}

pub fn part2(input: &[Aunt]) -> usize {
	let tape: HashMap<&str, u32> = [
		("children", 3),
		("cats", 7),
		("samoyeds", 2),
		("pomeranians", 3),
		("akitas", 0),
		("vizslas", 0),
		("goldfish", 5),
		("trees", 3),
		("cars", 2),
		("perfumes", 1),
	]
	.into_iter()
	.collect();

	'outer: for (i, aunt) in input.iter().enumerate() {
		for (compound, v) in &aunt.compounds {
			let vv = tape.get(compound.as_str()).unwrap();
			match compound.as_str() {
				"cats" | "trees" => {
					if vv >= v {
						continue 'outer;
					}
				}
				"pomeranians" | "goldfish" => {
					if vv <= v {
						continue 'outer;
					}
				}
				_ => {
					if vv != v {
						continue 'outer;
					}
				}
			}
		}

		return i + 1;
	}

	0
}
