#[derive(Debug)]
pub struct Triangle(u32, u32, u32);

pub fn parse_input(input: &str) -> Vec<Triangle> {
	input
		.lines()
		.map(|l| {
			let mut splited = l.split_whitespace();

			let a = splited.next().unwrap().parse().unwrap();
			let b = splited.next().unwrap().parse().unwrap();
			let c = splited.next().unwrap().parse().unwrap();

			Triangle(a, b, c)
		})
		.collect()
}

impl Triangle {
	fn is_possible(&self) -> bool {
		self.0 + self.1 > self.2 && self.1 + self.2 > self.0 && self.0 + self.2 > self.1
	}
}

pub fn part1(input: &[Triangle]) -> usize {
	input.iter().filter(|triangle| triangle.is_possible()).count()
}

pub fn part2(input: &[Triangle]) -> usize {
	let mut count = 0;
	for ts in input.chunks_exact(3) {
		if Triangle(ts[0].0, ts[1].0, ts[2].0).is_possible() {
			count += 1;
		}

		if Triangle(ts[0].1, ts[1].1, ts[2].1).is_possible() {
			count += 1;
		}

		if Triangle(ts[0].2, ts[1].2, ts[2].2).is_possible() {
			count += 1;
		}
	}

	count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert!(!Triangle(5, 10, 25).is_possible());
	}
}
