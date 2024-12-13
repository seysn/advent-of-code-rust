use regex::Regex;

#[derive(Debug)]
pub struct Machine {
	a: (i64, i64),
	b: (i64, i64),
	prize: (i64, i64),
}

pub fn parse_input(input: &str) -> Vec<Machine> {
	let re = Regex::new(r"A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

	input
		.split("\n\n")
		.map(|m| {
			let caps = re.captures(m).unwrap();
			Machine {
				a: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
				b: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
				prize: (caps[5].parse().unwrap(), caps[6].parse().unwrap()),
			}
		})
		.collect()
}

impl Machine {
	/// For this configuration:
	///  Button A: X+94, Y+34
	///  Button B: X+22, Y+67
	///  Prize: X=8400, Y=5400
	///
	/// We have the equation:
	///  94a + 22b = 8400
	///  34a + 67b = 5400
	///
	/// We solve it using Cramer's Rule
	fn solve(&self) -> Option<(i64, i64)> {
		let det_a = self.a.0 * self.b.1 - self.a.1 * self.b.0;
		if det_a == 0 {
			return None;
		}

		let det_a_x = self.prize.0 * self.b.1 - self.prize.1 * self.b.0;
		let det_a_y = self.a.0 * self.prize.1 - self.a.1 * self.prize.0;

		// If there's a remainder, there is no solution (we cannot press half a button)
		if det_a_x % det_a == 0 && det_a_y % det_a == 0 {
			Some((det_a_x / det_a, det_a_y / det_a))
		} else {
			None
		}
	}
}

pub fn part1(input: &[Machine]) -> i64 {
	input
		.iter()
		.filter_map(|m| {
			m.solve()
				.filter(|(a, b)| *a >= 0 && *a <= 100 && *b >= 0 && *b <= 100)
				.map(|(a, b)| a * 3 + b)
		})
		.sum()
}

pub fn part2(input: &[Machine]) -> i64 {
	input
		.iter()
		.filter_map(|m| {
			let m = Machine {
				a: m.a,
				b: m.b,
				prize: (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
			};

			m.solve().filter(|(a, b)| *a >= 0 && *b >= 0).map(|(a, b)| a * 3 + b)
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 480);
	}
}
