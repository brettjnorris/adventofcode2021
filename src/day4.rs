use std::collections::HashSet;
use std::iter::FromIterator;

type ProblemInput = (Vec<u64>, Vec<Board>);

#[derive(Clone)]
pub struct Board {
    id: u64,
    squares: Vec<u64>
}

impl Board {
    fn groups(&self) -> Vec<HashSet<u64>> {
        let mut groups: Vec<HashSet<u64>> = vec![];

        for i in 0..5 {
            let mut column: HashSet<u64>= HashSet::new();
            let mut row: HashSet<u64> = HashSet::new();

            for j in 0..5 {
                let column_index: usize = (i * 5) + j;
                let row_index: usize = i + (j * 5);
                column.insert(self.squares[column_index]);
                row.insert(self.squares[row_index]);
            }

            groups.push(column);
            groups.push(row);
        }

        groups
    }

    fn check_solution(&self, marked: &HashSet<u64>) -> bool {
        self.groups().iter().any(|group| {
            group.intersection(&marked).collect::<Vec<&u64>>().len() == 5
        })
    }

    fn score(&self, marked: &HashSet<u64>) -> u64 {
        self.squares_set().difference(&marked).collect::<Vec<&u64>>().into_iter().sum()
    }

    fn squares_set(&self) -> HashSet<u64> {
        HashSet::from_iter(self.squares.iter().cloned())
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> ProblemInput {
    let parts: Vec<&str> = input
        .split("\n\n")
        .collect::<Vec<&str>>();

    let (head, tail) = parts.split_at(1);
    let moves = parse_moves(head.first().unwrap());
    let boards: Vec<Board> = tail
        .to_vec()
        .into_iter()
        .enumerate()
        .map(|(i, board_string)| {
            parse_board(i as u64, board_string)
        })
        .collect::<Vec<Board>>();

    (moves, boards)
}

pub fn parse_moves(moves: &str) -> Vec<u64> {
    moves
        .split(',')
        .map(|val| {
            val.parse::<u64>().unwrap()
        }).collect::<Vec<u64>>()
}

pub fn parse_board(id: u64, input: &str) -> Board {
    let input_str = input
        .trim()
        .replace("\n", " ");

    let squares = input_str
        .split_whitespace()
        .map(|n| {
            n.trim().parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();

    Board { id, squares }
}


#[aoc(day4, part1)]
pub fn part1(input: &ProblemInput)  -> u64 {
    let (moves, boards) = input;

    let mut score = 0;
    let mut marked: HashSet<u64> = HashSet::new();

    'outer: for i in moves.into_iter() {
        marked.insert(*i);

        '_inner: for board in boards.iter() {
            if board.check_solution(&marked) {
                score = board.score(&marked) * *i;
                break 'outer;
            }
        }
    }

    score
}

#[aoc(day4, part2)]
pub fn part2(input: &ProblemInput)  -> u64 {
    let (moves, mut boards) = input.clone();

    let mut score = 0;
    let mut marked: HashSet<u64> = HashSet::new();

    'outer: for i in moves.into_iter() {
        marked.insert(i);

        let mut solved_board_ids: Vec<u64> = vec![];

        '_inner: for board in boards.clone().iter() {
            if board.check_solution(&marked) {
                if boards.len() == 1 {
                    score = board.score(&marked) * i;
                    break 'outer;
                } else {
                    // boards.remove(position);
                    solved_board_ids.push(board.id);
                }
            }

            boards.retain(|elem| {
                !solved_board_ids.contains(&elem.id)
            })
        }


    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7";

        assert_eq!(part1(&input_generator(input)), 4512)
    }

    #[test]
    fn test_part2() {
        let input = "\
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7";

        assert_eq!(part2(&input_generator(input)), 1924)
    }

    #[test]
    fn test_parse_board() {
        let input = "            22 13 17 11  0\n             8  2 23  4 24\n            21  9 14 16  7\n             6 10  3 18  5\n             1 12 20 15 19";

        let board = parse_board(1, input);
        assert_eq!(board.squares, vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19])
    }

    #[test]
    fn test_board_groups() {
        let board = Board {
            id: 1,
            squares: vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19]
        };

        let groups = board.groups();
        println!("{:?}", groups);
        assert_eq!(board.groups().len(), 10)
    }

    #[test]
    fn test_board_check_solution() {
        let marked: HashSet<u64> = HashSet::from_iter(vec![22, 13, 17, 11, 0]);
        let board = Board {
            id: 1,
            squares: vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19]
        };

        assert_eq!(board.check_solution(&marked), true)

    }

    #[test]
    fn test_board_score() {
        let marked: HashSet<u64> = HashSet::from_iter(vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12]);
        let board = Board {
            id: 1,
            squares: vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19]
        };

        assert_eq!(board.score(&marked), 54)
    }
}