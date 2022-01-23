pub struct Bingo {
	numbers: Vec<u32>,
	boards: Vec<Board>,
}

#[derive(Clone)]
pub struct Board {
	board: Vec<Vec<u32>>,
	found: Vec<Vec<bool>>,
	won: bool,
}

impl Board {
	fn new(board: Vec<Vec<u32>>) -> Board {
		Board {
			board: board,
			found: vec![vec![false; 5]; 5],
			won: false,
		}
	}

	fn unmarked_sum(&self) -> u32 {
		let mut unmarked_numbers: Vec<u32> = Vec::new();
		for (y_found, y_number) in self.found.iter().zip(self.board.iter()) {
			for (&x_found, &x_number) in y_found.iter().zip(y_number.iter()) {
				if !x_found {
					unmarked_numbers.push(x_number);
				}
			}
		}

		unmarked_numbers.iter().sum()
	}

	fn compute_win(&mut self) -> Option<u32> {
		// Horizontal
		for row_found in self.found.iter() {
			if row_found.iter().all(|&b| b) {
				self.won = true;
				return Some(self.unmarked_sum());
			}
		}

		// Vertical
		for x in 0..5 {
			let mut buffer_found: Vec<bool> = Vec::new();

			for y in 0..5 {
				buffer_found.push(self.found[y][x]);
			}

			if buffer_found.iter().all(|&b| b) {
				self.won = true;
				return Some(self.unmarked_sum());
			}
		}

		None
	}

	fn find(&mut self, number: u32) -> bool {
		for (y, v) in self.board.iter().enumerate() {
			match v.iter().position(|&x| x == number) {
				Some(x) => {
					self.found[y][x] = true;
					return true;
				}
				None => continue,
			}
		}
		false
	}
}

pub fn parse_input(content: &str) -> Bingo {
	let mut lines = content.lines();
	let numbers: Vec<u32> = lines.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();

	let mut boards: Vec<Board> = Vec::new();
	let mut buffer: Vec<Vec<u32>> = Vec::new();
	for l in lines.into_iter().skip(1) {
		if l.is_empty() {
			boards.push(Board::new(buffer));
			buffer = Vec::new();
		} else {
			let numbers: Vec<u32> = l.split(' ').filter(|n| !n.is_empty()).map(|n| n.parse().unwrap()).collect();
			buffer.push(numbers)
		}
	}

	boards.push(Board::new(buffer));
	Bingo {
		numbers: numbers,
		boards: boards,
	}
}

pub fn part1(input: &Bingo) -> u32 {
	let mut boards = input.boards.clone();

	for number in input.numbers.clone() {
		for board in boards.iter_mut() {
			board.find(number);

			let res = board.compute_win();
			if res.is_some() {
				return res.unwrap() * number;
			}
		}
	}

	unreachable!();
}

pub fn part2(input: &Bingo) -> u32 {
	let mut boards = input.boards.clone();
	let mut won_cpt = 0;
	let boards_len = boards.len();

	for number in input.numbers.clone() {
		for board in boards.iter_mut() {
			if board.won {
				continue;
			}

			board.find(number);

			// This is unefficient, as we compute a summary of all unmarked numbers, even when we don't want them,
			// but i'm too lazy to optimize it. I've got an answer under 1 second, that's way enough for me !
			let res = board.compute_win();
			if res.is_some() {
				won_cpt += 1;
				if won_cpt == boards_len {
					return res.unwrap() * number;
				}
			}
		}
	}

	unreachable!();
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &'static str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 4512);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 1924);
	}
}
