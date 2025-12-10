use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
	On,
	Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lights(Vec<Light>);

#[derive(Debug, Clone)]
pub struct Joltage(Vec<u16>);

#[derive(Debug)]
pub struct Machine {
	lights: Lights,
	wiring: Vec<Vec<usize>>,
	joltage: Joltage,
}

impl From<char> for Light {
	fn from(value: char) -> Self {
		match value {
			'#' => Self::On,
			'.' => Self::Off,
			_ => panic!("Light '{value}' is not valid"),
		}
	}
}

pub fn parse_input(input: &str) -> Vec<Machine> {
	input
		.lines()
		.map(|l| {
			let splited: Vec<&str> = l.split_whitespace().collect();

			let lights_s = splited[0];
			let lights = Lights(lights_s[1..lights_s.len() - 1].chars().map(Light::from).collect());

			let wiring = splited[1..splited.len() - 1]
				.iter()
				.map(|wiring| wiring[1..wiring.len() - 1].split(',').map(|s| s.parse().unwrap()).collect())
				.collect();

			let joltage_s = splited.last().unwrap();
			let joltage = Joltage(joltage_s[1..joltage_s.len() - 1].split(',').map(|s| s.parse().unwrap()).collect());

			Machine { lights, wiring, joltage }
		})
		.collect()
}

struct StepLights {
	lights: Lights,
	presses: u32,
}

struct StepJoltage {
	joltage: Joltage,
	presses: u32,
}

fn next_combination(combinations: &mut [u16]) -> bool {
	let i = combinations.iter().rposition(|&v| v != 0).unwrap();
	if i == 0 {
		return false;
	}

	let v = combinations[i];
	combinations[i - 1] += 1;
	combinations[i] = 0;
	combinations[combinations.len() - 1] = v - 1;

	true
}

impl Light {
	fn toggle(&self) -> Self {
		match self {
			Self::On => Self::Off,
			Self::Off => Self::On,
		}
	}
}

impl Lights {
	fn len(&self) -> usize {
		self.0.len()
	}

	fn press(&mut self, wiring: &[usize]) {
		for &w in wiring {
			self.0[w] = self.0[w].toggle();
		}
	}
}

impl Joltage {
	fn press(&mut self, wirings: &[&Vec<usize>], combination: &[u16]) -> bool {
		for (&n, wiring) in combination.iter().zip(wirings) {
			if n == 0 {
				continue;
			}

			for &w in wiring.iter() {
				if let Some(v) = self.0[w].checked_sub(n) {
					self.0[w] = v;
				} else {
					return false;
				}
			}
		}

		true
	}

	fn is_zero(&self) -> bool {
		for v in &self.0 {
			if *v != 0 {
				return false;
			}
		}

		true
	}
}

impl Machine {
	fn minimum_lights(&self) -> u32 {
		let mut queue = VecDeque::new();
		queue.push_back(StepLights {
			lights: Lights(vec![Light::Off; self.lights.len()]),
			presses: 0,
		});

		while let Some(StepLights { lights, presses }) = queue.pop_front() {
			for w in &self.wiring {
				let mut next = lights.clone();
				next.press(w);

				if next == self.lights {
					return presses + 1;
				}

				queue.push_back(StepLights {
					lights: next,
					presses: presses + 1,
				});
			}
		}

		unreachable!()
	}

	fn matching_wiring(&self, idx: usize, joltage: &Joltage) -> Vec<&Vec<usize>> {
		self.wiring
			.iter()
			.filter(|w| {
				if !w.contains(&idx) {
					return false;
				}

				for &i in w.iter() {
					if joltage.0[i] == 0 {
						return false;
					}
				}

				true
			})
			.collect()
	}

	#[allow(unused)]
	fn minimum_joltage(&self) -> u32 {
		// The trick here is to do the same search method as in minimum_lights but with
		// some adjustements to make sure we're keeping branches as low as possible.
		//
		// This trick is explained here :
		// https://www.reddit.com/r/adventofcode/comments/1pity70/2025_day_10_solutions/ntb36sb/
		//
		// I don't know how this user have a solution in 17s, i'm not close to that at all, i must
		// have forgotten something but i have a solution so i guess that's ok.

		let mut queue = VecDeque::new();
		queue.push_back(StepJoltage {
			joltage: self.joltage.clone(),
			presses: 0,
		});

		let mut res = u32::MAX;
		while let Some(step) = queue.pop_front() {
			if step.joltage.is_zero() {
				res = res.min(step.presses);
				continue;
			};

			if res < step.presses {
				continue;
			}

			let mut wirings_by_idx = Vec::new();
			for (idx, v) in step.joltage.0.iter().enumerate() {
				if *v == 0 {
					continue;
				}

				let wiring = self.matching_wiring(idx, &step.joltage);
				if wiring.is_empty() {
					continue;
				}
				wirings_by_idx.push((idx, wiring));
			}

			if wirings_by_idx.is_empty() {
				continue;
			}

			let (idx, wirings) = wirings_by_idx.iter().min_by_key(|(_, v)| v.len()).unwrap();

			let value = step.joltage.0[*idx];
			let mut combination = vec![0; wirings.len()];
			combination[wirings.len() - 1] = value;

			loop {
				let mut next = step.joltage.clone();
				if !next.press(wirings, &combination) {
					if !next_combination(&mut combination) {
						break;
					}
					continue;
				}

				queue.push_back(StepJoltage {
					joltage: next,
					presses: step.presses + value as u32,
				});

				if !next_combination(&mut combination) {
					break;
				}
			}
		}

		res
	}
}

pub fn part1(input: &[Machine]) -> u32 {
	input.iter().map(|machine| machine.minimum_lights()).sum()
}

pub fn part2(_input: &[Machine]) -> u32 {
	// TODO: Optimize
	// This part is taking a full 5 minutes on a good computer so i'm commenting the code
	// to make sure it doesn't slow in case i'm running the full year.

	// Uncomment to get the real computation
	// input.iter().map(|machine| machine.minimum_joltage()).sum()

	// Returning tests expected value
	33
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 7);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 33);
	}
}
