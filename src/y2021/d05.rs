use std::collections::HashMap;

pub fn parse_input(content: &str) -> Vec<(u32, u32, u32, u32)> {
	content
		.lines()
		.map(|l| {
			let v: Vec<u32> = l
				.split(|x| !char::is_numeric(x))
				.filter(|&x| !x.is_empty())
				.map(|x| x.parse().unwrap())
				.collect();
			(v[0], v[1], v[2], v[3])
		})
		.collect()
}

pub fn part1(input: &[(u32, u32, u32, u32)]) -> usize {
	let mut coordinates: HashMap<(u32, u32), u32> = HashMap::new();

	for &(x1, y1, x2, y2) in input {
		if x1 == x2 {
			for i in y1.min(y2)..=y1.max(y2) {
				let entry = coordinates.entry((x1, i)).or_insert(0);
				*entry += 1;
			}
		}

		if y1 == y2 {
			for i in x1.min(x2)..=x1.max(x2) {
				let entry = coordinates.entry((i, y1)).or_insert(0);
				*entry += 1;
			}
		}
	}

	coordinates.into_values().filter(|&x| x > 1).count()
}

pub fn part2(input: &[(u32, u32, u32, u32)]) -> usize {
	let mut coordinates: HashMap<(u32, u32), u32> = HashMap::new();

	for &(x1, y1, x2, y2) in input {
		if x1 == x2 {
			for i in y1.min(y2)..=y1.max(y2) {
				let entry = coordinates.entry((x1, i)).or_insert(0);
				*entry += 1;
			}
		} else if y1 == y2 {
			for i in x1.min(x2)..=x1.max(x2) {
				let entry = coordinates.entry((i, y1)).or_insert(0);
				*entry += 1;
			}
		} else {
			let range_x: Vec<u32> = if x1 < x2 { (x1..=x2).collect() } else { (x2..=x1).rev().collect() };

			let range_y: Vec<u32> = if y1 < y2 { (y1..=y2).collect() } else { (y2..=y1).rev().collect() };

			for (&x, y) in range_x.iter().zip(range_y) {
				let entry = coordinates.entry((x, y)).or_insert(0);
				*entry += 1;
			}
		}
	}

	coordinates.into_values().filter(|&x| x > 1).count()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 5);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 12);
	}
}
