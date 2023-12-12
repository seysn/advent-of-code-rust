#[derive(Default)]
struct Set {
	red: u32,
	green: u32,
	blue: u32,
}

pub struct Game {
	id: usize,
	sets: Vec<Set>,
}

pub fn parse_input(input: &str) -> Vec<Game> {
	input
		.lines()
		.enumerate()
		.map(|(i, l)| {
			let st = l.find(':').unwrap();
			let mut sets = Vec::new();
			for s in l[st + 2..].split("; ") {
				let mut set = Set::default();
				for b in s.split(", ") {
					let (n, color) = b.split_once(' ').unwrap();
					match color {
						"red" => set.red = n.parse().unwrap(),
						"blue" => set.blue = n.parse().unwrap(),
						"green" => set.green = n.parse().unwrap(),
						_ => unreachable!(),
					}
				}
				sets.push(set);
			}

			Game { id: i + 1, sets }
		})
		.collect()
}

pub fn part1(input: &[Game]) -> usize {
	input
		.iter()
		.filter(|game| !game.sets.iter().any(|set| set.red > 12 || set.green > 13 || set.blue > 14))
		.map(|game| game.id)
		.sum()
}

pub fn part2(input: &[Game]) -> u32 {
	input
		.iter()
		.map(|game| {
			let mut min_set = Set::default();
			for set in game.sets.iter() {
				if set.blue > min_set.blue {
					min_set.blue = set.blue;
				}
				if set.red > min_set.red {
					min_set.red = set.red;
				}
				if set.green > min_set.green {
					min_set.green = set.green;
				}
			}
			min_set.blue * min_set.red * min_set.green
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 8);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 2286);
	}
}
