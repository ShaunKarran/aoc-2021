use ::std::fs;

enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn main() {
    let movements = fs::read_to_string("input/day2")
        .expect("Something went wrong reading the file.")
        .lines()
        .map(|line| line.splitn(2, ' ').collect::<Vec<&str>>())
        .map(|pair| match pair[0] {
            "up" => Movement::Up(pair[1].parse::<i32>().unwrap()),
            "down" => Movement::Down(pair[1].parse::<i32>().unwrap()),
            "forward" => Movement::Forward(pair[1].parse::<i32>().unwrap()),
            _ => panic!("Invalid direction."),
        })
        .collect::<Vec<Movement>>();

    let mut horizontal = 0;
    let mut vertical = 0;
    for movement in movements {
        match movement {
            Movement::Up(distance) => vertical -= distance,
            Movement::Down(distance) => vertical += distance,
            Movement::Forward(distance) => horizontal += distance,
        }
    }
    println!("Part A: {}", horizontal * vertical);
}
