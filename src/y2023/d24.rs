use itertools::Itertools;
use std::ops::RangeInclusive;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Position(f64, f64, f64);
#[derive(Debug, Clone, Copy)]
struct Velocity(f64, f64, f64);

#[derive(Debug, Clone)]
pub struct Hailstone {
	position: Position,
	velocity: Velocity,
}

static HAILSTONE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+),\s*(\d+),\s*(\d+)\s*@\s*([-\d]+),\s*([-\d]+),\s*([-\d]+)").unwrap());

impl From<&str> for Hailstone {
	fn from(value: &str) -> Self {
		let caps = HAILSTONE_RE.captures(value).unwrap();
		Hailstone {
			position: Position(caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap()),
			velocity: Velocity(caps[4].parse().unwrap(), caps[5].parse().unwrap(), caps[6].parse().unwrap()),
		}
	}
}

impl Hailstone {
	fn intersect(&self, other: &Hailstone, area: &RangeInclusive<f64>) -> bool {
		let self_slope = self.velocity.1 / self.velocity.0;
		let other_slope = other.velocity.1 / other.velocity.0;

		let self_intercept = self.position.1 - self_slope * self.position.0;
		let other_intercept = other.position.1 - other_slope * other.position.0;

		let x = (other_intercept - self_intercept) / (self_slope - other_slope);
		let y = self_slope * x + self_intercept;

		if self.velocity.0 < 0.0 && x > self.position.0 || self.velocity.0 > 0.0 && x < self.position.0 {
			return false;
		}
		if self.velocity.1 < 0.0 && y > self.position.1 || self.velocity.1 > 0.0 && y < self.position.1 {
			return false;
		}
		if other.velocity.0 < 0.0 && x > other.position.0 || other.velocity.0 > 0.0 && x < other.position.0 {
			return false;
		}
		if other.velocity.1 < 0.0 && y > other.position.1 || other.velocity.1 > 0.0 && y < other.position.1 {
			return false;
		}

		area.contains(&x) && area.contains(&y)
	}
}

pub fn parse_input(input: &str) -> Vec<Hailstone> {
	input.lines().map(Hailstone::from).collect()
}

fn count_intersections(input: &[Hailstone], area: RangeInclusive<f64>) -> usize {
	input
		.iter()
		.tuple_combinations::<(&Hailstone, &Hailstone)>()
		.filter(|(a, b)| a.intersect(b, &area))
		.count()
}

pub fn part1(input: &[Hailstone]) -> usize {
	count_intersections(input, 200000000000000.0..=400000000000000.0)
}

fn gaussian_elimination(eqs: &mut [[f64; 7]; 6]) -> (f64, f64, f64) {
	// Perform gaussian elimination (taken from LinAGKar on Github because maths are hard)
	// Iterate diagonally from top left, to turn matrix into reduced row echelon form
	for i in 0..6 {
		// Find non-zero item in current column, from current row or after
		let non_zero_row = (i..6).find(|&row| eqs[row][i] != 0.0).unwrap();

		// Swap current row with first non-zero row
		if non_zero_row != i {
			(eqs[i], eqs[non_zero_row]) = (eqs[non_zero_row], eqs[i]);
		}

		// Divide row by value at current pos, to turn value into 1
		let curr_val = eqs[i][i];
		eqs[i][i] = 1.0;
		for item in &mut eqs[i][i + 1..] {
			*item /= curr_val;
		}

		// Subtract multiple of current row from lower rows, to turn column below current item to 0
		for row in i + 1..6 {
			let multiple = eqs[row][i];
			eqs[row][i] = 0.0;
			if multiple != 0.0 {
				for col in i + 1..7 {
					eqs[row][col] -= eqs[i][col] * multiple;
				}
			}
		}
	}

	// Iterate diagonally from bottom right, to turn matrix (except last column) into unit matrix.
	for i in (0..6).rev() {
		for row in 0..i {
			eqs[row][6] -= eqs[i][6] * eqs[row][i];
			eqs[row][i] = 0.0;
		}
	}

	(eqs[0][6].round(), eqs[1][6].round(), eqs[2][6].round())
}

#[allow(unused_variables)]
pub fn part2(input: &[Hailstone]) -> usize {
	let mut it = input.iter();
	let hailstone0 = it.next().unwrap();
	let hailstone1 = it.next().unwrap();
	let hailstone2 = it.next().unwrap();

	let mut eqs: [[f64; 7]; 6] = [[0.0; 7]; 6];
	for (i, (a, b)) in [(hailstone0, hailstone1), (hailstone1, hailstone2)].iter().enumerate() {
		// (dy'-dy) X + (dx-dx') Y + (y-y') DX + (x'-x) DY = x' dy' - y' dx' - x dy + y dx
		// (dz'-dz) X + (dx-dx') Z + (z-z') DX + (x'-x) DZ = x' dz' - z' dx' - x dz + z dx
		// (dy'-dy) Z + (dz-dz') Y + (y-y') DZ + (z'-z) DY = z' dy' - y' dz' - z dy + y dz
		eqs[i * 3] = [
			b.velocity.1 - a.velocity.1,
			a.velocity.0 - b.velocity.0,
			0.0,
			a.position.1 - b.position.1,
			b.position.0 - a.position.0,
			0.0,
			b.position.0 * b.velocity.1 - b.position.1 * b.velocity.0 - a.position.0 * a.velocity.1 + a.position.1 * a.velocity.0,
		];
		eqs[i * 3 + 1] = [
			b.velocity.2 - a.velocity.2,
			0.0,
			a.velocity.0 - b.velocity.0,
			a.position.2 - b.position.2,
			0.0,
			b.position.0 - a.position.0,
			b.position.0 * b.velocity.2 - b.position.2 * b.velocity.0 - a.position.0 * a.velocity.2 + a.position.2 * a.velocity.0,
		];
		eqs[i * 3 + 2] = [
			0.0,
			a.velocity.2 - b.velocity.2,
			b.velocity.1 - a.velocity.1,
			0.0,
			b.position.2 - a.position.2,
			a.position.1 - b.position.1,
			b.position.2 * b.velocity.1 - b.position.1 * b.velocity.2 - a.position.2 * a.velocity.1 + a.position.1 * a.velocity.2,
		];
	}
	let (x, y, z) = gaussian_elimination(&mut eqs);
	println!("{x} {y} {z}");

	x as usize + y as usize + z as usize
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

	#[test]
	fn test_intersect() {
		let a = Hailstone {
			position: Position(19.0, 13.0, 30.0),
			velocity: Velocity(-2.0, 1.0, -2.0),
		};
		let b = Hailstone {
			position: Position(18.0, 19.0, 22.0),
			velocity: Velocity(-1.0, -1.0, -2.0),
		};
		let c = Hailstone {
			position: Position(20.0, 25.0, 34.0),
			velocity: Velocity(-2.0, -2.0, -4.0),
		};
		let d = Hailstone {
			position: Position(12.0, 31.0, 28.0),
			velocity: Velocity(-1.0, -2.0, -1.0),
		};
		let e = Hailstone {
			position: Position(20.0, 19.0, 15.0),
			velocity: Velocity(1.0, -5.0, -3.0),
		};
		let area = &(7.0..=27.0);
		assert!(a.intersect(&b, area));
		assert!(a.intersect(&c, area));
		assert!(!a.intersect(&d, area));
		assert!(!a.intersect(&e, area));
	}

	#[test]
	fn example_part1() {
		assert_eq!(count_intersections(&parse_input(EXAMPLE), 7.0..=27.0), 2);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 47);
	}
}
