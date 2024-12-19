#![allow(clippy::needless_range_loop)]

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Ingredient([i64; 5]);

pub fn parse_input(input: &str) -> Vec<Ingredient> {
	let re = Regex::new(r".+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();

	input
		.lines()
		.map(|l| {
			let caps = re.captures(l).unwrap();
			let capacity = caps[1].parse().unwrap();
			let durability = caps[2].parse().unwrap();
			let flavor = caps[3].parse().unwrap();
			let texture = caps[4].parse().unwrap();
			let calories = caps[5].parse().unwrap();

			Ingredient([capacity, durability, flavor, texture, calories])
		})
		.collect()
}

pub fn part1(input: &[Ingredient]) -> i64 {
	let mut max = i64::MIN;
	'outer: for perm in (0..101).permutations(input.len()) {
		if perm.iter().sum::<i64>() != 100 {
			continue;
		}

		let mut factors = [0; 4];
		for i in 0..4 {
			for (j, ingredient) in input.iter().enumerate() {
				factors[i] += ingredient.0[i] * perm[j];
			}

			if factors[i] <= 0 {
				continue 'outer;
			}
		}

		let total = factors.iter().product();
		if total > max {
			max = total;
		}
	}

	max
}

pub fn part2(input: &[Ingredient]) -> i64 {
	let mut max = i64::MIN;
	'outer: for perm in (0..101).permutations(input.len()) {
		if perm.iter().sum::<i64>() != 100 {
			continue;
		}

		let mut calories = 0;
		for (j, ingredient) in input.iter().enumerate() {
			calories += ingredient.0[4] * perm[j];
		}

		if calories != 500 {
			continue;
		}

		let mut factors = [0; 4];
		for i in 0..4 {
			for (j, ingredient) in input.iter().enumerate() {
				factors[i] += ingredient.0[i] * perm[j];
			}

			if factors[i] <= 0 {
				continue 'outer;
			}
		}

		let total = factors.iter().product();
		if total > max {
			max = total;
		}
	}

	max
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 62842880);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 57600000);
	}
}
