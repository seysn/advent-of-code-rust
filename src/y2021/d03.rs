pub fn parse_input(content: &str) -> Vec<String> {
	content.lines().map(|s| s.to_owned()).collect()
}

fn count(line: String) -> (u32, u32) {
	line.chars()
		.fold((0, 0), |(z, o), c| if c == '0' { (z + 1, o) } else { (z, o + 1) })
}

pub fn part1(input: &[String]) -> i32 {
	// Init Vec
	let mut vec: Vec<String> = Vec::new();
	for _ in 0..input[0].len() {
		vec.push("".to_owned());
	}

	// Reverse dimension of input
	for line in input.iter() {
		for (i, c) in line.chars().enumerate() {
			vec[i].push(c);
		}
	}

	let mut bits = String::new();
	let mut bits_reversed = String::new();
	for column in vec.iter() {
		let (zeros, ones) = count(column.clone());
		if zeros > ones {
			bits.push('0');
			bits_reversed.push('1');
		} else {
			bits.push('1');
			bits_reversed.push('0');
		}
	}

	let gamma = isize::from_str_radix(&bits, 2).unwrap() as i32;
	let epsilon = isize::from_str_radix(&bits_reversed, 2).unwrap() as i32;

	gamma * epsilon
}

fn count_column(input: &[String], idx: usize) -> (usize, usize) {
	let (mut zeroes, mut ones) = (0, 0);
	for line in input.iter() {
		if line.chars().nth(idx).unwrap() == '0' {
			zeroes += 1;
		} else {
			ones += 1;
		}
	}

	(zeroes, ones)
}

pub fn part2(input: &[String]) -> i32 {
	let mut o2_bits = Vec::from(input);
	let mut co2_bits = Vec::from(input);
	for idx in 0..input.len() {
		if o2_bits.len() > 1 {
			let (zeros, ones) = count_column(&o2_bits, idx);
			let c_o2 = if zeros <= ones { '1' } else { '0' };
			let mut tmp_o2 = Vec::new();
			for line in o2_bits.iter() {
				if line.chars().nth(idx).unwrap() == c_o2 {
					tmp_o2.push(line.clone())
				}
			}
			o2_bits = tmp_o2;
		}

		if co2_bits.len() > 1 {
			let (zeros, ones) = count_column(&co2_bits, idx);
			let c_co2 = if ones >= zeros { '0' } else { '1' };
			let mut tmp_co2 = Vec::new();
			for line in co2_bits.iter() {
				if line.chars().nth(idx).unwrap() == c_co2 {
					tmp_co2.push(line.clone())
				}
			}
			co2_bits = tmp_co2;
		}
	}

	let oxygen_rating = isize::from_str_radix(&o2_bits[0], 2).unwrap() as i32;
	let co2_rating = isize::from_str_radix(&co2_bits[0], 2).unwrap() as i32;

	oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part1() {
		assert_eq!(
			part1(&[
				String::from("00100"),
				String::from("11110"),
				String::from("10110"),
				String::from("10111"),
				String::from("10101"),
				String::from("01111"),
				String::from("00111"),
				String::from("11100"),
				String::from("10000"),
				String::from("11001"),
				String::from("00010"),
				String::from("01010")
			]),
			198
		);
	}

	#[test]
	fn example_part2() {
		assert_eq!(
			part2(&[
				String::from("00100"),
				String::from("11110"),
				String::from("10110"),
				String::from("10111"),
				String::from("10101"),
				String::from("01111"),
				String::from("00111"),
				String::from("11100"),
				String::from("10000"),
				String::from("11001"),
				String::from("00010"),
				String::from("01010")
			]),
			230
		);
	}
}
