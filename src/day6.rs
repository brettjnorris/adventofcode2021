#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    run_via_rotate(input, 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    run_via_rotate(input, 256)
}

pub fn run_via_rotate(input: &str, days: usize) -> usize {
    let mut lifetimes = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    input
        .split(",")
        .for_each(|val| {
            let n: usize = val.parse::<usize>().unwrap();
            lifetimes[n] += 1;
        });

    for _i in 0..days {
        lifetimes.rotate_left(1);
        lifetimes[6] += lifetimes.last().unwrap().clone();
    }

    lifetimes.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3,4,3,1,2";

        assert_eq!(part1(input), 5934)
    }

    #[test]
    fn test_part2() {
        let input = "3,4,3,1,2";

        assert_eq!(part2(input), 26984457539)
    }

    #[test]
    fn test_rotate() {
        let input = "3,4,3,1,2";

        assert_eq!(run_via_rotate(input, 80), 5934)
    }
}