use ::std::fs;

fn main() {
    let input = fs::read_to_string("input/day1")
        .expect("Something went wrong reading the file.")
        .lines()
        .map(|line| line.parse::<i32>().expect("Line was not a valid integer."))
        .collect::<Vec<i32>>();

    part_a(&input);
    part_b(&input);
}

fn part_a(input: &Vec<i32>) {
    let number_of_increases =
        input.windows(2).fold(
            0,
            |acc, window| if window[1] > window[0] { acc + 1 } else { acc },
        );

    println!("Part A: {}", number_of_increases);
}

fn part_b(input: &Vec<i32>) {
    let number_of_increases = input.windows(4).fold(0, |acc, w| {
        if w[0..3].iter().sum::<i32>() < w[1..4].iter().sum() {
            acc + 1
        } else {
            acc
        }
    });

    println!("Part B: {}", number_of_increases);
}
