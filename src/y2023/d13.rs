pub struct Grid {
	lines: Vec<String>,
	rows: Vec<String>,
}

impl Grid {
	fn score(&self, differences: usize) -> usize {
		if let Some(i) = pattern_notes(&self.lines, differences) {
			i * 100
		} else if let Some(i) = pattern_notes(&self.rows, differences) {
			i
		} else {
			0
		}
	}
}

fn line_differences(a: &str, b: &str) -> usize {
	a.chars()
		.zip(b.chars())
		.fold(0, |acc, (aa, bb)| if aa != bb { acc + 1 } else { acc })
}

fn mirror_differences(a: &[String], b: &[String]) -> usize {
	a.iter().rev().zip(b.iter()).map(|(aa, bb)| line_differences(aa, bb)).sum()
}

fn pattern_notes(lines: &[String], differences: usize) -> Option<usize> {
	(1..lines.len())
		.find(|&i| line_differences(&lines[i - 1], &lines[i]) <= differences && mirror_differences(&lines[..i], &lines[i..]) == differences)
}

pub fn parse_input(input: &str) -> Vec<Grid> {
	input
		.split("\n\n")
		.map(|grid| {
			let lines: Vec<String> = grid.lines().map(|l| l.to_string()).collect();
			let mut rows = vec!["".to_string(); lines[0].len()];
			for l in &lines {
				for (i, c) in l.chars().enumerate() {
					rows[i].push(c);
				}
			}

			Grid { lines, rows }
		})
		.collect()
}

pub fn part1(input: &[Grid]) -> usize {
	input.iter().map(|grid| grid.score(0)).sum()
}

pub fn part2(input: &[Grid]) -> usize {
	input.iter().map(|grid| grid.score(1)).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 405);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 400);
	}
}
