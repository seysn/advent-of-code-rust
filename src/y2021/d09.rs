pub fn parse_input(content: &str) -> Vec<Vec<u8>> {
    let mut res = Vec::new();

    for line in content.lines() {
        res.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }

    res
}

pub fn part1(input: &[Vec<u8>]) -> u32 {
    let mut res: Vec<u8> = Vec::new();
    for (y, v) in input.iter().enumerate() {
        for (x, &height) in v.iter().enumerate() {
            if y != 0 && input[y - 1][x] <= height {
                continue;
            }

            if y != input.len() - 1 && input[y + 1][x] <= height {
                continue;
            }

            if x != 0 && input[y][x - 1] <= height {
                continue;
            }

            if x != v.len() - 1 && input[y][x + 1] <= height {
                continue;
            }

            res.push(height);
        }
    }

    res.iter().map(|&x| x as u32 + 1).sum()
}

fn basin_length(input: &[Vec<u8>], x: usize, y: usize, width: usize, height: usize, visited: &mut Vec<(usize, usize)>) -> usize {
    let mut cpt = 0;

    if input[y][x] == 9 || visited.contains(&(x, y)) {
        return 0;
    }

    visited.push((x, y));

    if y != 0 {
        cpt += basin_length(input, x, y - 1, width, height, visited);
    }

    if y != height - 1 {
        cpt += basin_length(input, x, y + 1, width, height, visited);
    }

    if x != 0 {
        cpt += basin_length(input, x - 1, y, width, height, visited);
    }

    if x != width - 1 {
        cpt += basin_length(input, x + 1, y, width, height, visited);
    }

    cpt + 1
}

pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut sizes: Vec<usize> = Vec::new();

    for (y, v) in input.iter().enumerate() {
        for x in 0..v.iter().len() {
            if !visited.contains(&(x, y)) && input[y][x] != 9 {
                sizes.push(basin_length(input, x, y, v.len(), input.len(), &mut visited));
            }
        }
    }
    
    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 15);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 1134);
    }
}