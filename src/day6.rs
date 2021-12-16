use std::{collections::HashMap, fs};

fn main() {
    let fish = fs::read_to_string("input/day6")
        .expect("Something went wrong reading the file.")
        .split(',')
        .map(|a| a.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    dbg!(part1(&fish));
    dbg!(part2(&fish));
}

fn part1(fishies: &Vec<i64>) -> i64 {
    let mut counts = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);
    for &fish in fishies {
        counts.entry(fish).and_modify(|count| *count += 1).or_insert(1);
    }
    for _day in 0..80 {
        counts = new_update(counts);
    }

    counts.into_values().sum()
}

fn part2(fishies: &Vec<i64>) -> i64 {
    let mut counts = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);
    for &fish in fishies {
        counts.entry(fish).and_modify(|count| *count += 1).or_insert(1);
    }
    for _day in 0..256 {
        counts = new_update(counts);
    }

    counts.into_values().sum()
}

fn new_update(counts: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_counts: HashMap<i64, i64> = HashMap::new();
    let number_of_6s = counts.get(&0).unwrap() + counts.get(&7).unwrap();
    new_counts.insert(0, *counts.get(&1).unwrap());
    new_counts.insert(1, *counts.get(&2).unwrap());
    new_counts.insert(2, *counts.get(&3).unwrap());
    new_counts.insert(3, *counts.get(&4).unwrap());
    new_counts.insert(4, *counts.get(&5).unwrap());
    new_counts.insert(5, *counts.get(&6).unwrap());
    new_counts.insert(6, number_of_6s);
    new_counts.insert(7, *counts.get(&8).unwrap());
    new_counts.insert(8, *counts.get(&0).unwrap());

    new_counts
}
