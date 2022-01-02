pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Position(i32, i32);

impl Position {
    fn add(&mut self, direction: &Direction) {
        let (x, y) = match direction {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        };

        self.0 += x;
        self.1 += y;
    }
}

pub fn parse_input(input: &str) -> Vec<Direction> {
    input.chars().map(|c| match c {
        '^' => Direction::North,
        'v' => Direction::South,
        '<' => Direction::West,
        '>' => Direction::East,
        _ => unreachable!(),
    }).collect()
}

pub fn part1(input: &[Direction]) -> usize {
    let mut positions: Vec<Position> = vec![Position(0, 0)];
    let mut pos = Position(0, 0);

    for dir in input {
        pos.add(dir);
        if !positions.contains(&pos) {
            positions.push(pos);
        }
    }

    positions.len()
}

pub fn part2(input: &[Direction]) -> usize {
    let mut pos_santa = Position(0, 0);
    let mut pos_robot = Position(0, 0);
    let mut positions: Vec<Position> = vec![Position(0, 0)];

    for (idx, dir) in input.iter().enumerate() {
        if idx % 2 == 0 {
            pos_santa.add(dir);
            if !positions.contains(&pos_santa) {
                positions.push(pos_santa);
            }
        } else {
            pos_robot.add(dir);
            if !positions.contains(&pos_robot) {
                positions.push(pos_robot);
            }
        }
    }

    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(">")), 2);
        assert_eq!(part1(&parse_input("^>v<")), 4);
        assert_eq!(part1(&parse_input("^v^v^v^v^v")), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input("^v")), 3);
        assert_eq!(part2(&parse_input("^>v<")), 3);
        assert_eq!(part2(&parse_input("^v^v^v^v^v")), 11);
    }
}