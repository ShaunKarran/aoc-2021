use std::fs;

fn main() {
    let fish = fs::read_to_string("input/day6")
        .expect("Something went wrong reading the file.")
        .split(',')
        .map(|a| a.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    dbg!(part1(&fish));
    dbg!(part2(&fish));
}

fn part1(fish: &Vec<i32>) -> usize {
    let mut part1_fish = fish.clone();

    for _day in 0..80 {
        part1_fish = update(&part1_fish);
    }

    part1_fish.len()
}

fn part2(fish: &Vec<i32>) -> usize {
    0
}

fn update(fish: &Vec<i32>) -> Vec<i32> {
    let mut updated_fish = Vec::new();
    let mut fish_to_add = 0;
    for fish in fish {
        if *fish == 0 {
            updated_fish.push(6);
            fish_to_add += 1;
        }
        else {
            updated_fish.push(fish - 1);
        }
    }
    for _ in 0..fish_to_add {
        updated_fish.push(8);
    }
    updated_fish
}
