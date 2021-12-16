use std::fs;

fn main() {
    let input = fs::read_to_string("input/day3")
        .expect("Something went wrong reading the file.")
        .lines()
        .map(|line| String::from(line))
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut first_str = String::from("");
    let mut second_str = String::from("");
    for index in 0..input[0].len() {
        let (number_of_ones, number_of_zeros) = count_bits(input, index);
        let bit = if number_of_ones > number_of_zeros {
            '1'
        } else {
            '0'
        };
        let not_bit = if bit == '1' { '0' } else { '1' };
        first_str.push(bit);
        second_str.push(not_bit);
    }
    let first = i32::from_str_radix(&first_str, 2).unwrap();
    let second = i32::from_str_radix(&second_str, 2).unwrap();

    println!("Part 1: {}", first * second);
}

fn part2(input: &Vec<String>) {
    let mut oxygen_numbers = input.clone();
    let mut bit_index = 0;
    while oxygen_numbers.len() > 1 {
        let (number_of_ones, number_of_zeros) = count_bits(&oxygen_numbers, bit_index);
        let bit = if number_of_ones >= number_of_zeros {
            '1'
        } else {
            '0'
        };
        oxygen_numbers = filter_on_bit_index(&oxygen_numbers, bit_index, bit);
        bit_index += 1;
    }
    dbg!(&oxygen_numbers);

    let mut co2_scrubber_numbers = input.clone();
    bit_index = 0;
    while co2_scrubber_numbers.len() > 1 {
        let (number_of_ones, number_of_zeros) = count_bits(&co2_scrubber_numbers, bit_index);
        let bit = if number_of_ones < number_of_zeros {
            '1'
        } else {
            '0'
        };
        co2_scrubber_numbers = filter_on_bit_index(&co2_scrubber_numbers, bit_index, bit);
        bit_index += 1;
    }

    let first = i32::from_str_radix(&oxygen_numbers[0], 2).unwrap();
    let second = i32::from_str_radix(&co2_scrubber_numbers[0], 2).unwrap();

    println!("Part 2: {}", first * second);
}

fn count_bits(numbers: &Vec<String>, bit_index: usize) -> (usize, usize) {
    let number_of_ones = numbers
        .iter()
        .filter(|line| line.chars().nth(bit_index).unwrap() == '1')
        .count();
    let number_of_zeros = numbers.len() - number_of_ones;
    (number_of_ones, number_of_zeros)
}

/// Returns only the values in the numbers vec where the value at the bit_index
/// matches the most common value in that index.
fn filter_on_bit_index(numbers: &Vec<String>, bit_index: usize, bit_value: char) -> Vec<String> {
    return numbers
        .iter()
        .filter(|number| number.chars().nth(bit_index).unwrap() == bit_value)
        .map(|number| String::from(number))
        .collect();
}
