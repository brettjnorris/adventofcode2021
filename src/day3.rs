use std::iter::{Iterator, FromIterator};

pub fn filter_by_common(input: &Vec<Vec<char>>, position: usize, most: bool) -> Vec<Vec<char>> {
    let common_value = match most {
        true => most_common_value(input, position),
        false => least_common_value(input, position)
    };
    let filtered_rows = filter_by_value(input, position, common_value);

    match filtered_rows.len() {
        0 => filtered_rows,
        1 => filtered_rows,
        _ => filter_by_common(&filtered_rows, position + 1, most)
    }
}

pub fn filter_by_value(input: &Vec<Vec<char>>, position: usize, value: char) -> Vec<Vec<char>> {
   input
       .into_iter()
       .filter(|val| {
           val[position] == value
       })
       .cloned()
       .collect::<Vec<Vec<char>>>()
}

pub fn most_common_value(input: &Vec<Vec<char>>, position: usize) -> char {
    let (ones, zeroes): (Vec<char>, Vec<char>) = input
        .into_iter()
        .map(|val| val[position] )
        .partition(|&n| n == '1' );

    match ones.len() >= zeroes.len() {
        true => '1',
        false => '0'
    }
}

pub fn least_common_value(input: &Vec<Vec<char>>, position: usize) -> char {
    let (ones, zeroes): (Vec<char>, Vec<char>) = input
        .into_iter()
        .map(|val| val[position] )
        .partition(|&n| n == '1' );

    match ones.len() >= zeroes.len() {
        true => '0',
        false => '1'
    }
}

pub fn compute_significant_bitmap(input: &Vec<Vec<char>>, most: bool) -> u32 {
    let mut bit_string: Vec<char> = Vec::new();
    let length = input[0].len();

    for i in 0..length {
        let new_value = match most {
            true => most_common_value(input, i),
            false => least_common_value(input, i)
        };

        bit_string.push(new_value)
    }

    u32::from_str_radix(&String::from_iter(bit_string), 2).unwrap()
}

pub fn chars_to_int(input: &Vec<char>) -> u32 {
    u32::from_str_radix(&String::from_iter(input), 2).unwrap()
}


#[aoc(day3, part1)]
pub fn part1(input: &str)  -> u32 {
    let values: Vec<Vec<char>> = input
        .lines()
        .map(|val| { val.chars().collect() })
        .collect::<Vec<Vec<char>>>();

    let gamma = compute_significant_bitmap(&values, true);
    let epsilon = compute_significant_bitmap(&values, false);

    let product = gamma * epsilon;

    product
}

#[aoc(day3, part2)]
pub fn part2(input: &str) ->  u32 {
    let values: Vec<Vec<char>> = input
        .lines()
        .map(|val| { val.chars().collect() })
        .collect::<Vec<Vec<char>>>();

    let oxygen_rating = chars_to_int(&filter_by_common(&values, 0, true)[0]);
    let co2_rating = chars_to_int(&filter_by_common(&values, 0, false)[0]);

    println!("oxygen_rating: {:?}, co2_rating: {:?}", oxygen_rating, co2_rating);

    oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

        assert_eq!(part1(input), 198)
    }

    #[test]
    fn example2() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

        assert_eq!(part2(input), 230)
    }

}