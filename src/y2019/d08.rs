use std::collections::HashMap;

struct Image<'a> {
	layers: Vec<&'a [u8]>,
	width: usize,
	height: usize,
}

impl<'a> Image<'a> {
	fn new(raw: &'a [u8], width: usize, height: usize) -> Self {
		let mut layers = Vec::with_capacity(raw.len() / width * height);
		for layer in raw.chunks(width * height) {
			layers.push(layer);
		}
		Image { layers, width, height }
	}

	fn decode(&self) -> Vec<u8> {
		let mut res = Vec::new();

		for i in 0..(self.width * self.height) {
			for l in &self.layers {
				if l[i] != 2 {
					res.push(l[i]);
					break;
				}
			}
		}

		res
	}
}

pub fn parse_input(input: &str) -> Vec<u8> {
	input.chars().map(|c| c as u8 - 48).collect()
}

pub fn part1(input: &[u8]) -> usize {
	Image::new(input, 25, 6)
		.layers
		.iter()
		.map(|l| {
			l.iter().fold(HashMap::new(), |mut map: HashMap<u8, usize>, c| {
				*map.entry(*c).or_insert(0) += 1;
				map
			})
		})
		.fold((usize::MAX, 0), |(min, res), map| {
			let zeroes = *map.get(&0).unwrap_or(&0);
			if zeroes < min {
				let ones = *map.get(&1).unwrap_or(&0);
				let twos = *map.get(&2).unwrap_or(&0);
				(zeroes, ones * twos)
			} else {
				(min, res)
			}
		})
		.1
}

pub fn part2(input: &[u8]) -> String {
	let mut res = String::new();
	let img = Image::new(input, 25, 6);
	let decoded = img.decode();

	res.push('\n');
	for line in decoded.chunks(img.width) {
		for c in line {
			res.push(if c == &1 { 'X' } else { ' ' });
		}
		res.push('\n');
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part2() {
		let raw = parse_input("0222112222120000");
		let img = Image::new(&raw, 2, 2);
		let decoded = img.decode();
		assert_eq!(decoded.len(), 4);
		assert_eq!(decoded[0], 0);
		assert_eq!(decoded[1], 1);
		assert_eq!(decoded[2], 1);
		assert_eq!(decoded[3], 0);
	}
}
