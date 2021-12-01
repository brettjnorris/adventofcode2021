use std::iter::Iterator;
use std::convert::TryInto;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut last = 0;

    input
        .lines()
        .map(|val| val.parse::<usize>().unwrap() )
        .for_each(|val| {
            if last > 0 && val > last {
                count += 1;
            }

            last = val;
        });

    return count;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut last = 0;

    let values = input
        .lines()
        .map(|val| val.parse::<usize>().unwrap() )
        .collect::<Vec<usize>>();

    values.windows(3).for_each(|values| {
        let sum = values.iter().sum();

        if last > 0 && sum > last {
            count += 1;
        }

        last = sum;
    });

    return count;
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn example1() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

        assert_eq!(part1(input), 7)
    }

    #[test]
    fn example2() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

        assert_eq!(part2(input), 5)
    }
}