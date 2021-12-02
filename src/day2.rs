use std::iter::Iterator;

pub struct Command  {
    direction: String,
    magnitude: usize
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.trim().split(' ');
            let direction: String = parts.next().unwrap().to_string();
            let magnitude: usize = parts.next().unwrap().parse::<usize>().unwrap();

            return Command {
                direction,
                magnitude,
            }
        }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> usize {
    let mut depth = 0;
    let mut position = 0;

    input
        .iter()
        .for_each(|command| {
            match command.direction.as_str() {
                "forward" => {
                    position += command.magnitude
                },
                "up" => {
                    depth -= command.magnitude
                },
                "down" => {
                    depth += command.magnitude
                },
                _ => {}
            }
        });

    return depth * position;
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> usize {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    input
        .iter()
        .for_each(|command| {
            match command.direction.as_str() {
                "forward" => {
                    position += command.magnitude;
                    depth += command.magnitude * aim;
                },
                "up" => {
                    aim -= command.magnitude
                },
                "down" => {
                    aim += command.magnitude
                },
                _ => {}
            }
        });

    return depth * position;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

        assert_eq!(part1(&input_generator(input)), 150)
    }

    #[test]
    fn example2() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

        assert_eq!(part2(&input_generator(input)), 900)
    }
}