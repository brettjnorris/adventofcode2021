use std::iter::{Iterator, FromIterator};

// #[aoc_generator(day3)]
// pub fn input_generator(input: &str) -> Vec<String> {
//     input
//         .lines().collect().clone()
// }

// pub fn generate_bitmask(position: u8) -> u8 {
//     let bit_string = ""
//     for i in 0..=5 {
//         if i == position {
//
//         }
//     }
//
//     u8::from_str_radix(bit_string, 2).unwrap()
// }

pub fn compute_significant_bitmap(input: &Vec<Vec<char>>, most: bool) -> u16 {
    let mut bit_string: Vec<char> = Vec::new();
    let length = input[0].len();
    for i in 0..length {
        println!("{}", i);

        let (ones, zeroes): (Vec<char>, Vec<char>) = input
            .into_iter()
            .map(|val| val[i] )
            .partition(|&n| n == '1' );

        let mut new_value = '0';

        if most && ones.iter().count() > zeroes.iter().count() || !most && ones.iter().count() < zeroes.iter().count() {
           new_value = '1';
        }

        bit_string.push(new_value)
    }

    println!("{:?}", bit_string);
    u16::from_str_radix(&String::from_iter(bit_string), 2).unwrap()
}


#[aoc(day3, part1)]
pub fn part1(input: &str)  -> u16 {
    let values: Vec<Vec<char>> = input
        .lines()
        .map(|val| { val.chars().collect() })
        .collect::<Vec<Vec<char>>>();

    println!("values: {:?}", values);

    let gamma = compute_significant_bitmap(&values, true);
    let epsilon = compute_significant_bitmap(&values, false);
    println!("gamma: {:?}, epsilon: {:?}", gamma, epsilon);

    return gamma * epsilon;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

        assert_eq!(part1(input), 198)
    }


}