pub fn parse_input(input: &str) -> String {
	input.to_owned()
}

enum Block {
	Empty,
	File(u64),
}

fn blocks(input: &str) -> Vec<Block> {
	let mut blocks: Vec<Block> = Vec::new();
	for (i, ch) in input.chars().enumerate() {
		let value = ch as u32 - 48;
		for _ in 0..value {
			if i % 2 == 0 {
				blocks.push(Block::File(i as u64 / 2));
			} else {
				blocks.push(Block::Empty);
			}
		}
	}
	blocks
}

fn checksum(blocks: &[Block]) -> u64 {
	blocks
		.iter()
		.enumerate()
		.filter_map(|(i, b)| match b {
			Block::Empty => None,
			Block::File(v) => Some(i as u64 * *v),
		})
		.sum()
}

pub fn part1(input: &str) -> u64 {
	let mut blocks = blocks(input);

	let mut left = 0;
	let mut right = blocks.len() - 1;
	loop {
		if left >= right {
			break;
		}

		if matches!(&blocks[left], &Block::File(_)) {
			left += 1;
		}

		if matches!(&blocks[right], &Block::Empty) {
			right -= 1;
		}

		if matches!((&blocks[left], &blocks[right]), (&Block::Empty, &Block::File(_))) {
			blocks.swap(left, right);
		}
	}

	checksum(&blocks)
}

#[derive(Debug, Clone)]
enum BlockGroup {
	Empty { size: usize },
	File { id: u64, size: usize },
}

impl BlockGroup {
	fn id(&self) -> u64 {
		match self {
			BlockGroup::Empty { .. } => unreachable!(),
			BlockGroup::File { id, .. } => *id,
		}
	}

	fn size(&self) -> usize {
		match self {
			BlockGroup::Empty { size } => *size,
			BlockGroup::File { size, .. } => *size,
		}
	}

	fn is_empty(&self) -> bool {
		match self {
			BlockGroup::Empty { .. } => true,
			BlockGroup::File { .. } => false,
		}
	}
}

#[derive(Debug)]
struct FileSystem {
	groups: Vec<BlockGroup>,
	files: Vec<usize>,
}

impl FileSystem {
	fn new(input: &str) -> Self {
		let mut groups: Vec<BlockGroup> = Vec::new();
		let mut files = Vec::new();
		for (i, ch) in input.chars().enumerate() {
			let size = ch as usize - 48;
			if i % 2 == 0 {
				files.push(groups.len());
				groups.push(BlockGroup::File { id: i as u64 / 2, size });
			} else if size != 0 {
				groups.push(BlockGroup::Empty { size });
			}
		}

		Self { groups, files }
	}

	fn move_file(&mut self, ei: usize, fi: usize) {
		let es = self.groups[ei].size();
		let f = self.groups[fi].clone();

		// Edge case
		if ei + 1 == fi {
			self.groups[fi] = BlockGroup::Empty { size: f.size() };
			self.groups[ei] = f;
			return;
		}

		// Set right part
		let mut space_left = self.groups[fi].size();
		if fi + 1 < self.groups.len() && matches!(self.groups[fi + 1], BlockGroup::Empty { .. }) {
			space_left += self.groups[fi + 1].size();
			self.groups.remove(fi + 1);

			for ff in &mut self.files {
				if *ff > fi + 1 {
					*ff -= 1;
				}
			}
		}

		if matches!(self.groups[fi - 1], BlockGroup::Empty { .. }) {
			self.groups[fi - 1] = BlockGroup::Empty {
				size: self.groups[fi - 1].size() + space_left,
			};
			self.groups.remove(fi);

			for ff in &mut self.files {
				if *ff > fi {
					*ff -= 1;
				}
			}
		} else {
			self.groups[fi] = BlockGroup::Empty { size: space_left }
		}

		// Set left part
		self.files[f.id() as usize] = ei;
		if es == f.size() {
			self.groups[ei] = f;
		} else {
			self.groups[ei] = BlockGroup::Empty {
				size: self.groups[ei].size() - f.size(),
			};
			self.groups.insert(ei, f);

			for ff in &mut self.files {
				if *ff > ei {
					*ff += 1;
				}
			}
		}
	}

	fn compact(&mut self) {
		for id in (0..self.files.len()).rev() {
			if let Some(ei) = self
				.groups
				.iter()
				.enumerate()
				.find(|(idx, b)| b.is_empty() && self.files[id] > *idx && b.size() >= self.groups[self.files[id]].size())
				.map(|(i, _)| i)
			{
				self.move_file(ei, self.files[id]);
			}
		}
	}

	fn checksum(&self) -> u64 {
		let mut blocks = Vec::new();
		for b in &self.groups {
			for _ in 0..b.size() {
				blocks.push(match b {
					BlockGroup::Empty { .. } => Block::Empty,
					BlockGroup::File { id, .. } => Block::File(*id),
				})
			}
		}

		checksum(&blocks)
	}
}

pub fn part2(input: &str) -> u64 {
	let mut fs = FileSystem::new(input);
	fs.compact();

	fs.checksum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "2333133121414131402";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 1928);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 2858);
		assert_eq!(part2(&parse_input("252")), 5);
		assert_eq!(part2(&parse_input("12345")), 132);
		assert_eq!(part2(&parse_input("354631466260")), 1325);
		assert_eq!(part2(&parse_input("1010101010101010101010")), 385);
		assert_eq!(part2(&parse_input("171010402")), 88);
		assert_eq!(part2(&parse_input("111000000000000000001")), 12);
		assert_eq!(part2(&parse_input("14113")), 16);
		assert_eq!(part2(&parse_input("2333133121414131401")), 2746);
	}
}
