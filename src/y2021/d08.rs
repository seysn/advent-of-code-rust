pub fn parse_input(content: &str) -> Vec<Vec<&str>> {
	let mut res: Vec<Vec<&str>> = Vec::new();
	for line in content.lines() {
		res.push(line.split(' ').filter(|&x| x != "|").collect())
	}
	res
}

pub fn part1(input: &[Vec<&str>]) -> usize {
	let mut cpt = 0;

	for line in input.iter() {
		cpt += line.iter().skip(10).filter(|&&x| matches!(x.len(), 2 | 3 | 4 | 7)).count();
	}

	cpt
}

pub fn part2(input: &[Vec<&str>]) -> u64 {
	let mut cpt = 0;

	for line in input.iter() {
		let uniques: Vec<&str> = line.iter().take(10).copied().collect();
		let digits: Vec<&str> = line.iter().skip(10).take(4).copied().collect();

		let digit_one = *uniques.iter().find(|&&x| x.len() == 2).unwrap();
		let digit_four = *uniques.iter().find(|&&x| x.len() == 4).unwrap();

		let mut res = String::new();
		for s in digits {
			let similarity_one = digit_one.chars().map(|x| s.find(x).is_some()).filter(|&x| x).count();
			let similarity_four = digit_four.chars().map(|x| s.find(x).is_some()).filter(|&x| x).count();

			res.push(match s.len() {
				2 => '1',
				3 => '7',
				4 => '4',
				7 => '8',
				5 => {
					if similarity_one == 2 {
						'3'
					} else if similarity_four == 2 {
						'2'
					} else {
						'5'
					}
				}
				6 => {
					if similarity_one == 1 {
						'6'
					} else if similarity_four == 3 {
						'0'
					} else {
						'9'
					}
				}
				_ => unreachable!(),
			});
		}
		cpt += res.parse::<u64>().unwrap();
	}

	cpt
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE1: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
	const EXAMPLE2: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE1)), 0);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 26);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE1)), 5353);
		assert_eq!(part2(&parse_input(EXAMPLE2)), 61229);
	}
}
