use std::collections::HashMap;
use itertools::zip;

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point(usize, usize);

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn is_straight(&self) -> bool {
       self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn intersecting_points(&self) -> Vec<Point> {
        let range_x: Vec<usize> = Line::generate_range(self.start.0, self.end.0);
        let range_y: Vec<usize> = Line::generate_range(self.start.1, self.end.1);

        match (range_x.len(), range_y.len()) {
            (1, _) => Line::expand(range_y, self.start.0, Direction::Horizontal),
            (_, 1) => Line::expand(range_x, self.start.1, Direction::Vertical),
            (_, _) => Line::expand_diagonal(range_x, range_y)
        }
    }

    fn expand(range: Vec<usize>, n: usize, direction: Direction) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for i in range {
            let point = match direction {
                Direction::Horizontal => Point(n, i),
                Direction::Vertical => Point(i, n)
            };
            points.push(point)
        }

        points
    }

    fn expand_diagonal(range_x: Vec<usize>, range_y: Vec<usize>) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for (a, b) in zip(range_x, range_y) {
            points.push(Point(a, b))
        }

        points
    }

    fn generate_range(start: usize, end: usize) -> Vec<usize> {
        match end > start {
            true => {
                (start..=end).map(usize::from).collect::<Vec<usize>>()
            },
            false => {
                (end..=start).rev().map(usize::from).collect::<Vec<usize>>()
            }
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let parts = l.trim().split(" -> ").collect::<Vec<&str>>();

            Line {
                start: parse_point(parts[0]),
                end: parse_point(parts[1])
            }
        })
        .collect::<Vec<Line>>()
}

fn parse_point(input: &str) -> Point {
    let parts = input
        .split(',')
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    Point(parts[0], parts[1])
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Line>)  -> usize {
    let mut points_map: HashMap<Point, i32> = HashMap::new();

    input
        .iter()
        .filter(|line| line.is_straight())
        .for_each(|line| {
            line
                .intersecting_points()
                .iter()
                .for_each(|point| {
                    let count = points_map.entry(point.clone()).or_insert(0);
                    *count += 1;
                });
        });

    points_map
        .into_iter()
        .map(|(_key, val)| val)
        .filter(|val| {
            val > &1
        })
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Line>)  -> usize {
    let mut points_map: HashMap<Point, i32> = HashMap::new();

    input
        .iter()
        .for_each(|line| {
            line
                .intersecting_points()
                .iter()
                .for_each(|point| {
                    let count = points_map.entry(point.clone()).or_insert(0);
                    *count += 1;
                });
        });

    points_map
        .into_iter()
        .map(|(_key, val)| val)
        .filter(|val| {
            val > &1
        })
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2";

        assert_eq!(part1(&input_generator(input)), 5)
    }

    #[test]
    fn test_part2() {
        let input = "\
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2";

        assert_eq!(part2(&input_generator(input)), 12)
    }

    #[test]
    fn test_line_is_straight() {
        let line = Line {
            start: Point(1, 1),
            end: Point(1, 3)
        };

        assert_eq!(line.is_straight(), true)
    }

    #[test]
    fn test_intersecting_points_x() {
        let line = Line {
            start: Point(1, 1),
            end: Point(3, 1)
        };

        assert_eq!(line.intersecting_points(), vec![Point(1, 1), Point(2, 1), Point(3, 1)])
    }

    #[test]
    fn test_intersecting_points_y() {
        let line = Line {
            start: Point(1, 1),
            end: Point(1, 3)
        };

        assert_eq!(line.intersecting_points(), vec![Point(1, 1), Point(1, 2), Point(1, 3)])
    }

    #[test]
    fn test_intersecting_points_misc() {
        let line = Line {
            start: Point(0, 9),
            end: Point(2, 9)
        };

        assert_eq!(line.intersecting_points(), vec![Point(0, 9), Point(1, 9), Point(2, 9)])
    }

    #[test]
    fn test_intersecting_diagonal() {
        let line = Line {
            start: Point(1, 1),
            end: Point(3, 3)
        };


        assert_eq!(line.intersecting_points(), vec![Point(1, 1), Point(2, 2), Point(3, 3)])
    }

    #[test]
    fn test_intersecting_diagonal_2() {
        let line = Line {
            start: Point(9, 7),
            end: Point(7, 9)
        };

        assert_eq!(line.intersecting_points(), vec![Point(9, 7), Point(8, 8), Point(7, 9)])
    }

    #[test]
    fn point_index() {
        let point_a = Point(0, 0);
        let point_b = Point(5, 0);
    }

    #[test]
    fn generate_range() {
        let start = 1;
        let end = 5;

        assert_eq!(Line::generate_range(start, end), vec![1, 2, 3, 4, 5])
    }
}
