use std::ops::Range;

struct ConvertLine {
	destination_range_start: u64,
	source_range_start: u64,
	range_length: u64,
}

struct Map {
	convert_lines: Vec<ConvertLine>,
}

pub struct Almanac {
	seeds: Vec<u64>,
	maps: Vec<Map>,
}

impl ConvertLine {
	fn in_range(&self, value: u64) -> bool {
		(self.source_range_start..self.source_range_start + self.range_length).contains(&value)
	}

	fn convert(&self, value: u64) -> u64 {
		value - self.source_range_start + self.destination_range_start
	}

	fn sub_range(&self, range: &Range<u64>) -> Option<Range<u64>> {
		let src_start = self.source_range_start;
		let src_end = self.source_range_start + self.range_length;

		let sub_start = src_start.max(range.start);
		let sub_end = src_end.min(range.end);

		if sub_start < sub_end {
			Some(sub_start..sub_end)
		} else {
			None
		}
	}
}

impl Map {
	fn convert(&self, value: u64) -> u64 {
		for convert_line in &self.convert_lines {
			if convert_line.in_range(value) {
				return convert_line.convert(value);
			}
		}

		value
	}

	fn convert_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
		let mut res = Vec::new();
		let mut ranges = vec![range];

		'outer: while let Some(range) = ranges.pop() {
			for convert_line in &self.convert_lines {
				if let Some(sub_range) = convert_line.sub_range(&range) {
					res.push(convert_line.convert(sub_range.start)..convert_line.convert(sub_range.end));
					if sub_range.start > range.start {
						ranges.push(range.start..sub_range.start);
					}
					if sub_range.end < range.end {
						ranges.push(sub_range.end..range.end);
					}
					continue 'outer;
				}
			}
			res.push(range);
		}

		res
	}
}

pub fn parse_input(input: &str) -> Almanac {
	let mut splited = input.split("\n\n");
	let seeds = splited
		.next()
		.unwrap()
		.strip_prefix("seeds: ")
		.unwrap()
		.split(' ')
		.map(|n| n.parse::<u64>().unwrap())
		.collect();

	let mut maps = Vec::new();
	for map in splited {
		let convert_lines: Vec<ConvertLine> = map
			.lines()
			.skip(1)
			.map(|l| {
				let mut s = l.split(' ').map(|n| n.parse::<u64>().unwrap());
				let destination_range_start = s.next().unwrap();
				let source_range_start = s.next().unwrap();
				let range_length = s.next().unwrap();
				ConvertLine {
					destination_range_start,
					source_range_start,
					range_length,
				}
			})
			.collect();

		maps.push(Map { convert_lines });
	}

	Almanac { seeds, maps }
}

pub fn part1(input: &Almanac) -> u64 {
	let mut location = u64::MAX;
	for seed in &input.seeds {
		let mut value = *seed;

		for map in &input.maps {
			value = map.convert(value);
		}

		if value < location {
			location = value;
		}
	}

	location
}

pub fn part2(input: &Almanac) -> u64 {
	let mut ranges = Vec::new();
	for seed_pair in input.seeds.chunks(2) {
		ranges.push(seed_pair[0]..seed_pair[0] + seed_pair[1]);
	}

	for map in &input.maps {
		let mut new_ranges = Vec::new();

		for range in ranges {
			// Could be optimized by merging ranges but this is good enough
			new_ranges.extend(map.convert_range(range));
		}

		ranges = new_ranges;
	}

	ranges.iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 35);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 46);
	}
}
