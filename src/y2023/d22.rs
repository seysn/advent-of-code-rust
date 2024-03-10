use std::collections::{HashMap, HashSet};

use crate::collections::Point3D;

pub struct Tower {
	bricks: Vec<Brick>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Brick {
	start_cube: Point3D,
	end_cube: Point3D,
}

impl Tower {
	fn sort_by_height(&self) -> Vec<Brick> {
		let mut res = self.bricks.clone();
		res.sort_by(|a, b| a.start_cube.z.cmp(&b.start_cube.z));
		res
	}

	fn fall(&self) -> Tower {
		let mut bricks: Vec<Brick> = Vec::new();

		for b in &self.sort_by_height() {
			for dz in 0..b.start_cube.z {
				let new_brick = Brick {
					start_cube: Point3D::new(b.start_cube.x, b.start_cube.y, b.start_cube.z - dz),
					end_cube: Point3D::new(b.end_cube.x, b.end_cube.y, b.end_cube.z - dz),
				};
				if b.start_cube.z - dz == 1 || bricks.iter().any(|bb| bb.is_supporting(&new_brick)) {
					bricks.push(new_brick);
					break;
				}
			}
		}

		Tower { bricks }
	}
}

impl Brick {
	fn is_supporting(&self, other: &Self) -> bool {
		if self.end_cube.z != other.start_cube.z - 1 {
			return false;
		}

		if self.start_cube.x < other.end_cube.x + 1
			&& self.end_cube.x + 1 > other.start_cube.x
			&& self.start_cube.y < other.end_cube.y + 1
			&& self.end_cube.y + 1 > other.start_cube.y
		{
			return true;
		}

		false
	}
}

impl From<&str> for Brick {
	fn from(value: &str) -> Self {
		let [fst, snd]: [&str; 2] = value.split('~').take(2).collect::<Vec<&str>>().try_into().unwrap();

		let [x, y, z] = fst
			.split(',')
			.take(3)
			.map(|i| i.parse().unwrap())
			.collect::<Vec<i32>>()
			.try_into()
			.unwrap();
		let start_cube = Point3D::new(x, y, z);

		let [x, y, z] = snd
			.split(',')
			.take(3)
			.map(|i| i.parse().unwrap())
			.collect::<Vec<i32>>()
			.try_into()
			.unwrap();
		let end_cube = Point3D::new(x, y, z);

		if start_cube.x != end_cube.x && start_cube.y != end_cube.y {
			panic!("no");
		}

		Brick { start_cube, end_cube }
	}
}

pub fn parse_input(input: &str) -> Tower {
	Tower {
		bricks: input.lines().map(Brick::from).collect(),
	}
}

pub fn part1(input: &Tower) -> usize {
	let tower = input.fall();
	let mut needed: HashSet<Brick> = HashSet::new();
	for brick in &tower.bricks {
		let c: Vec<Brick> = tower.bricks.iter().filter(|b| b.is_supporting(brick)).cloned().collect();
		if c.len() == 1 {
			needed.insert(c.first().unwrap().clone());
		}
	}
	tower.bricks.len() - needed.len()
}

#[derive(Clone)]
struct State {
	brick: Brick,
	disintegrated: HashSet<Brick>,
}

fn count_supporting(state: &mut State, supports: &HashMap<Brick, HashSet<Brick>>, supporting: &HashMap<Brick, HashSet<Brick>>) -> usize {
	let mut todo = HashSet::new();
	for b in supporting.get(&state.brick).unwrap() {
		let s = supports.get(b).unwrap();
		if s.is_subset(&state.disintegrated) {
			state.disintegrated.insert(b.clone());
			todo.insert(b.clone());
		}
	}

	for brick in todo {
		let mut new_state = state.clone();
		new_state.brick = brick;
		count_supporting(&mut new_state, supports, supporting);
		state.disintegrated.extend(new_state.disintegrated);
	}

	state.disintegrated.len() - 1
}

pub fn part2(input: &Tower) -> usize {
	let tower = input.fall();

	let mut supports: HashMap<Brick, HashSet<Brick>> = HashMap::new();
	let mut supporting: HashMap<Brick, HashSet<Brick>> = HashMap::new();
	for brick in &tower.bricks {
		let s: HashSet<Brick> = tower.bricks.iter().filter(|b| b.is_supporting(brick)).cloned().collect();
		supports.insert(brick.clone(), s);
		let s: HashSet<Brick> = tower.bricks.iter().filter(|b| brick.is_supporting(b)).cloned().collect();
		supporting.insert(brick.clone(), s);
	}

	let mut res = 0;
	for brick in &tower.bricks {
		res += count_supporting(
			&mut State {
				brick: brick.clone(),
				disintegrated: HashSet::from_iter([brick.clone()]),
			},
			&supports,
			&supporting,
		);
	}

	res
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

	const EXAMPLE2: &str = "0,0,1~0,1,1
1,1,1~1,1,1
0,0,2~0,0,2
0,1,2~1,1,2";

	#[test]
	fn test_is_supporting() {
		assert!(Brick::from("1,0,1~1,2,1").is_supporting(&Brick::from("0,1,2~2,1,2")));
		assert!(Brick::from("0,0,1~0,2,1").is_supporting(&Brick::from("0,0,2~2,0,2")));
		assert!(Brick::from("0,0,1~0,2,1").is_supporting(&Brick::from("0,1,2~2,1,2")));
		assert!(!Brick::from("0,0,1~2,0,1").is_supporting(&Brick::from("0,2,2~2,2,2")));
		assert!(Brick::from("0,0,1~0,1,1").is_supporting(&Brick::from("0,0,2~0,0,2")));
	}

	#[test]
	fn example_part1() {
		assert_eq!(part1(&parse_input(EXAMPLE)), 5);
		assert_eq!(part1(&parse_input(EXAMPLE2)), 3);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&parse_input(EXAMPLE)), 7);
	}
}
