use ::std::fs;

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
    let half_input_length = input.len() / 2;

    let mut first_str = String::from("");
    let mut second_str = String::from("");
    for index in 0..input[0].len() {
        let number_of_ones = input.iter()
            .filter(|line| line.chars().nth(index).unwrap() == '1')
            .count();
        let bit = if number_of_ones > half_input_length {'1'} else {'0'};
        let not_bit = if number_of_ones > half_input_length {'0'} else {'1'};
        first_str.push(bit);
        second_str.push(not_bit);
    }
    let first = i32::from_str_radix(&first_str, 2).unwrap();
    let second = i32::from_str_radix(&second_str, 2).unwrap();

    println!("Part 1: {}", first * second);
}

fn part2(input: &Vec<String>) {
    println!("Part 2: {}", "TODO");
}
