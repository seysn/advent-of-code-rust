use num::Integer;
use regex::Regex;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Reindeer {
	name: String,
	speed: u32,
	speed_time: u32,
	rest_time: u32,
}

pub fn parse_input(input: &str) -> Vec<Reindeer> {
	let re = Regex::new(r"(.+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
	input
		.lines()
		.map(|l| {
			let caps = re.captures(l).unwrap();

			Reindeer {
				name: caps[1].to_owned(),
				speed: caps[2].parse().unwrap(),
				speed_time: caps[3].parse().unwrap(),
				rest_time: caps[4].parse().unwrap(),
			}
		})
		.collect()
}

fn winner1(time: u32, input: &[Reindeer]) -> u32 {
	input
		.iter()
		.map(|r| {
			let round_time = r.speed_time + r.rest_time;
			let (rounds, rem) = time.div_rem(&round_time);

			r.speed * r.speed_time * rounds + r.speed * rem.min(r.speed_time)
		})
		.max()
		.unwrap()
}

pub fn part1(input: &[Reindeer]) -> u32 {
	winner1(2503, input)
}

#[derive(Debug)]
enum ReindeerState {
	Flying(u32),
	Resting(u32),
}

#[derive(Debug)]
struct ReindeerRunner {
	reindeer: Reindeer,
	state: ReindeerState,
	distance: u32,
	score: u32,
}

impl ReindeerRunner {
	fn step(&mut self) {
		match &self.state {
			ReindeerState::Flying(i) => {
				self.distance += self.reindeer.speed;
				if i == &1 {
					self.state = ReindeerState::Resting(self.reindeer.rest_time);
				} else {
					self.state = ReindeerState::Flying(i - 1);
				}
			}
			ReindeerState::Resting(i) => {
				if i == &1 {
					self.state = ReindeerState::Flying(self.reindeer.speed_time);
				} else {
					self.state = ReindeerState::Resting(i - 1);
				}
			}
		}
	}
}

fn winner2(time: u32, input: &[Reindeer]) -> u32 {
	let mut runners: Vec<ReindeerRunner> = input
		.iter()
		.map(|r| ReindeerRunner {
			reindeer: r.clone(),
			state: ReindeerState::Flying(r.speed_time),
			distance: 0,
			score: 0,
		})
		.collect();

	for _ in 0..time {
		for r in &mut runners {
			r.step();
		}

		let max = runners.iter().map(|r| r.distance).max().unwrap();
		for r in &mut runners {
			if r.distance == max {
				r.score += 1;
			}
		}
	}

	runners.iter().map(|r| r.score).max().unwrap()
}

pub fn part2(input: &[Reindeer]) -> u32 {
	winner2(2503, input)
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

	#[test]
	fn example_part1() {
		assert_eq!(winner1(1000, &parse_input(EXAMPLE)), 1120);
	}

	#[test]
	fn example_part2() {
		// assert_eq!(winner2(140, &parse_input(EXAMPLE)), 0);
		assert_eq!(winner2(1000, &parse_input(EXAMPLE)), 689);
	}
}
