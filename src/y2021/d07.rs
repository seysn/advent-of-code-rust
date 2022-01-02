pub fn parse_input(content: &str) -> Vec<i32> {
    content.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &[i32]) -> i32 {
    let &max = input.iter().max().unwrap();

    let mut fuels: Vec<i32> = Vec::new();
    for fuel in 0..=max {
        fuels.push(input.iter().fold(0, |acc, &x| acc + x.max(fuel) - x.min(fuel)));
    }

    *fuels.iter().min().unwrap()
}

pub fn part2(input: &[i32]) -> i32 {
    let &max = input.iter().max().unwrap();

    let mut fuels: Vec<i32> = Vec::new();
    for fuel in 0..=max {
        fuels.push(input.iter().fold(0, |acc, &x| {
            let diff = x.max(fuel) - x.min(fuel);
            acc + ((diff + 1) * diff / 2)
        }));
    }

    *fuels.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 37);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 168);
    }
}