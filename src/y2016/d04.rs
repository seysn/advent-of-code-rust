use std::{cmp::Ordering, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
pub struct Room {
	name: String,
	sector_id: u32,
	checksum: String,
}

pub fn parse_input(input: &str) -> Vec<Room> {
	let re = Regex::new(r"^([a-z\-]+)-(\d+)\[([a-z]+)\]$").unwrap();

	input
		.lines()
		.map(|l| {
			let caps = re.captures(l).unwrap();
			let name = caps[1].to_owned();
			let sector_id = caps[2].parse().unwrap();
			let checksum = caps[3].to_owned();

			Room { name, sector_id, checksum }
		})
		.collect()
}

impl Room {
	fn is_real(&self) -> bool {
		let mut counter: HashMap<char, usize> = HashMap::new();
		for ch in self.name.chars().filter(char::is_ascii_alphabetic) {
			*counter.entry(ch).or_default() += 1;
		}

		for (a, b) in self.checksum.chars().zip(self.checksum.chars().skip(1)) {
			let (Some(aa), Some(bb)) = (counter.get(&a), counter.get(&b)) else {
				return false;
			};

			match aa.cmp(bb) {
				Ordering::Less => {
					return false;
				}
				Ordering::Equal => {
					if a >= b {
						return false;
					}
				}
				Ordering::Greater => (),
			}
		}

		true
	}

	fn decrypt(&self) -> String {
		let mut real_name = String::new();
		for ch in self.name.chars() {
			real_name.push(match ch {
				'-' => ' ',
				_ => ((((ch as u32 - 97) + self.sector_id) % (122 - 97 + 1)) + 97) as u8 as char,
			});
		}

		real_name
	}
}

pub fn part1(input: &[Room]) -> u32 {
	input
		.iter()
		.filter_map(|room| if room.is_real() { Some(room.sector_id) } else { None })
		.sum()
}

pub fn part2(input: &[Room]) -> u32 {
	input
		.iter()
		.find(|room| room.decrypt() == "northpole object storage")
		.map(|room| room.sector_id)
		.unwrap_or_default()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 1514);
	}

	#[test]
	fn example_part2() {
		assert_eq!(
			Room {
				name: "qzmt-zixmtkozy-ivhz".to_owned(),
				sector_id: 343,
				checksum: "".to_owned()
			}
			.decrypt(),
			"very encrypted name".to_owned()
		);
	}
}
