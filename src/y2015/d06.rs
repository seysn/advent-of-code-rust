use regex::Regex;

pub struct Grid<T> {
    lights: Vec<T>
}

impl Grid<bool> {
    fn new() -> Grid<bool> {
        Grid { lights: vec![false; 1_000_000] }
    }

    fn run(&mut self, inst: &Instruction) {
        let f: fn(bool) -> bool = match inst.action {
            Action::Toggle => |x: bool| !x,
            Action::TurnOn => |_| true,
            Action::TurnOff => |_| false,
        };

        for y in inst.from.0..=inst.to.0 {
            for x in inst.from.1..=inst.to.1 {
                let idx = x + 1000 * y;
                self.lights[idx] = f(self.lights[idx]);
            }
        }
    }

    fn count(&self) -> usize {
        self.lights.iter().fold(0, |acc, &b| if b { acc + 1 } else { acc })
    }
}

impl Grid<usize> {
    fn new() -> Grid<usize> {
        Grid { lights: vec![0; 1_000_000] }
    }

    fn run(&mut self, inst: &Instruction) {
        let f: fn(usize) -> usize = match inst.action {
            Action::Toggle => |x| x + 2,
            Action::TurnOn => |x| x + 1,
            Action::TurnOff => |x| x.saturating_sub(1),
        };

        for y in inst.from.0..=inst.to.0 {
            for x in inst.from.1..=inst.to.1 {
                let idx = x + 1000 * y;
                self.lights[idx] = f(self.lights[idx]);
            }
        }
    }

    fn count(&self) -> usize {
        self.lights.iter().sum()
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    action: Action,
    from: (usize, usize),
    to: (usize, usize),
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r".*(on|off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    input.lines().map(|l| {
        let cap = re.captures(l).unwrap();
        let action = match &cap[1] {
            "on" => Action::TurnOn,
            "off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => unreachable!(),
        };

        Instruction {
            action,
            from: (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
            to: (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
        }
    }).collect()
}

pub fn part1(input: &[Instruction]) -> usize {
    let mut grid = Grid::<bool>::new();
    for inst in input {
        grid.run(inst);
    }
    grid.count()
}

pub fn part2(input: &[Instruction]) -> usize {
    let mut grid = Grid::<usize>::new();
    for inst in input {
        grid.run(inst);
    }
    grid.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";

    #[test]
    fn test_parser() {
        let instructions = parse_input(EXAMPLE);
        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0], Instruction { action: Action::TurnOn, from: (0, 0), to: (999, 999)});
        assert_eq!(instructions[1], Instruction { action: Action::Toggle, from: (0, 0), to: (999, 0)});
        assert_eq!(instructions[2], Instruction { action: Action::TurnOff, from: (499, 499), to: (500, 500)});
    }

    #[test]
    fn example_part1() {
        let mut grid = Grid::<bool>::new();
        assert_eq!(grid.count(), 0);
        grid.run(&Instruction { action: Action::TurnOn, from: (0, 0), to: (999, 999)});
        assert_eq!(grid.count(), 1_000_000);
        grid.run(&Instruction { action: Action::TurnOff, from: (0, 0), to: (999, 999)});
        assert_eq!(grid.count(), 0);
        grid.run(&Instruction { action: Action::Toggle, from: (0, 0), to: (999, 0)});
        assert_eq!(grid.count(), 1000);
        grid.run(&Instruction { action: Action::Toggle, from: (0, 0), to: (999, 0)});
        assert_eq!(grid.count(), 0);
        grid.run(&Instruction { action: Action::Toggle, from: (499, 499), to: (500, 500)});
        assert_eq!(grid.count(), 4);
    }

    #[test]
    fn example_part2() {
        let mut grid = Grid::<usize>::new();
        assert_eq!(grid.count(), 0);
        grid.run(&Instruction { action: Action::TurnOn, from: (0, 0), to: (0, 0)});
        assert_eq!(grid.count(), 1);
        grid.run(&Instruction { action: Action::TurnOff, from: (0, 0), to: (0, 0)});
        assert_eq!(grid.count(), 0);
        grid.run(&Instruction { action: Action::Toggle, from: (0, 0), to: (999, 999)});
        assert_eq!(grid.count(), 2000000);
    }
}