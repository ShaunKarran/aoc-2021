use std::{
    collections::HashMap,
    fs,
};

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Points {
    start: Point,
    end: Point,
    // The previous point that was returned.
    previous: Option<Point>,
}

impl Points {
    fn new(line: Line) -> Self {
        Self { 
            start: line.start,
            end: line.end,
            previous: None,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match &self.previous {
            Some(previous) => {
                if *previous == self.end { return None }
                let x_increment = if self.start.x == self.end.x {
                    0
                } else {
                    if self.start.x < self.end.x { 1 } else { -1 }
                };
                let y_increment = if self.start.y == self.end.y {
                    0
                } else {
                    if self.start.y < self.end.y { 1 } else { -1 }
                };
                Some(Point { x: previous.x + x_increment, y: previous.y + y_increment })
            },
            None => Some(self.start),
        };
        self.previous = next;
        next
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points(self) -> Points {
        Points::new(self)
    }
}

fn main() {
    let lines = fs::read_to_string("input/day5")
        .expect("Something went wrong reading the file.")
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(a, b)| (String::from(a), String::from(b)))
        .map(|(start_str, end_str)| {
            let (x, y) = start_str.split_once(',').unwrap();
            let start: Point = Point { x: x.parse::<i32>().unwrap(), y: y.parse::<i32>().unwrap() };
    
            let (x, y) = end_str.split_once(',').unwrap();
            let end: Point = Point { x: x.parse::<i32>().unwrap(), y: y.parse::<i32>().unwrap() };
            Line { start, end }
        })
        .collect::<Vec<Line>>();

    dbg!(part1(&lines));
    dbg!(part2(&lines));
}

fn part1(lines: &Vec<Line>) -> usize {
    let part_1_lines = lines
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect::<Vec<_>>();
    // dbg!(&part_1_lines);
    
    let mut counts: HashMap<Point, i32> = HashMap::new();
    for line in part_1_lines {
        for point in line.points() {
            counts.entry(point).and_modify(|count| *count += 1).or_insert(1);
        }
    }

    counts.into_iter()
        .filter(|(_point, count)| *count >= 2)
        .count()
}

fn part2(lines: &Vec<Line>) -> usize {
    let mut counts: HashMap<Point, i32> = HashMap::new();
    for line in lines {
        for point in line.points() {
            counts.entry(point).and_modify(|count| *count += 1).or_insert(1);
        }
    }

    counts.into_iter()
        .filter(|(_point, count)| *count >= 2)
        .count()
}
