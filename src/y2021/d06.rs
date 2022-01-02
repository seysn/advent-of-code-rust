use std::collections::HashMap;

pub fn parse_input(content: &str) -> Vec<u32> {
    content.split(',').map(|x| x.parse().unwrap()).collect()
}

pub fn part1(input: &[u32]) -> usize {
    let mut state: Vec<u32> = Vec::from(input);
    let mut new_fishes = 0;

    for _ in 0..80 {
        let mut tmp = Vec::new();

        for fish in state {
            match fish {
                0 => {
                    new_fishes += 1;
                    tmp.push(6);
                },
                i => tmp.push(i - 1),
            }
        }

        for _ in 0..new_fishes {
            tmp.push(8);
        }

        new_fishes = 0;
        state = tmp;
    }

    state.iter().len()
}

pub fn part2(input: &[u32]) -> u64 {
    let mut fishes: HashMap<u32, u64> = HashMap::new();

    for &i in input {
        let entry = fishes.entry(i).or_insert(0);
        *entry += 1;
    }

    for _ in 0..256 {
        let mut last = 0;
        for i in (0..=8).rev() {
            let entry = fishes.entry(i).or_insert(0);
            let value = entry.clone();
            *entry = last;
            if i == 0 {
                let six = fishes.entry(6).or_insert(0);
                *six += value;

                let new_fishes = fishes.entry(8).or_insert(0);
                *new_fishes += value;
            } else {
                last = value;
            }
        }
    }

    let mut sum = 0;
    for i in fishes.values() {
        sum += i;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "3,4,3,1,2";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 5934);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 26984457539);
    }
}