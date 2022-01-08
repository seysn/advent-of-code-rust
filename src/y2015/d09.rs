use std::collections::HashMap;

use regex::Regex;
use itertools::Itertools;

pub struct Cities {
    cities: Vec<String>,
    distances: HashMap<(String, String), u32>
}

pub fn parse_input(input: &str) -> Cities {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut distances = HashMap::new();
    let mut cities = Vec::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps[1].to_string();
        let b = caps[2].to_string();
        distances.insert((a.clone(), b.clone()), caps[3].parse().unwrap());
        if !cities.contains(&a) {
            cities.push(a);
        }
        if !cities.contains(&b) {
            cities.push(b);
        }
    }

    Cities{ cities, distances }
}

fn route_distance(route: &[&String], distances: &HashMap<(String, String), u32>) -> u32 {
    let mut res = 0;
    for pair in route.windows(2) {
        let a = (pair[0].clone(), pair[1].clone());
        if distances.contains_key(&a) {
            res += distances.get(&a).unwrap();
        } else {
            let b = (pair[1].clone(), pair[0].clone());
            res += distances.get(&b).unwrap();
        }
    }
    res
}

pub fn part1(input: &Cities) -> u32 {
    let mut distances: Vec<u32> = Vec::new();
    for perm in input.cities.iter().permutations(input.cities.len()) {
        distances.push(route_distance(&perm, &input.distances));
    }

    *distances.iter().min().unwrap()
}

pub fn part2(input: &Cities) -> u32 {
    let mut distances: Vec<u32> = Vec::new();
    for perm in input.cities.iter().permutations(input.cities.len()) {
        distances.push(route_distance(&perm, &input.distances));
    }

    *distances.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn example_part1() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 605);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 982);
    }
}