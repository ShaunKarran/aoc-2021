use std::{
    collections::HashMap,
    fs,
};

#[derive(Debug, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// struct Points {
//     start: Point,
//     end: Point,

// }

// impl Iterator for Points {
//     type Item = Point;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    // fn points(self) -> Points {
    // }
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

    part1(&lines);
    // part2(&lines);
}

fn part1(lines: &Vec<Line>) {
    let part_1_lines = lines
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect::<Vec<_>>();
    // dbg!(&part_1_lines);
    
    let mut counts: HashMap<Point, i32> = HashMap::new();
    for line in part_1_lines {
        for point in point_on_line(line) {
            counts.entry(point).and_modify(|count| *count += 1).or_insert(1);
        }
    }

    dbg!(counts.into_iter()
        .filter(|(_point, count)| *count >= 2)
        .count()
    );
}

fn part2(lines: &Vec<Line>) {
    dbg!();
}

fn point_on_line(line: &Line) -> Vec<Point> {
    enum Direction {
        Horizontal,
        Vertical,
    }

    let iteration_direction = if line.start.x == line.end.x { Direction::Horizontal } else { Direction::Vertical };
    let mut points = Vec::new();
    let range = match iteration_direction {
        Direction::Horizontal => {
            let start = line.start.y;
            let end = line.end.y;
            if start < end { start..end + 1 } else { end..start + 1 }
        }
        Direction::Vertical => {
            let start = line.start.x;
            let end = line.end.x;
            if start < end { start..end + 1 } else { end..start + 1 }
        }
    };
    for v in range {
        match iteration_direction {
            Direction::Horizontal => points.push(Point { x: line.start.x, y: v }),
            Direction::Vertical => points.push(Point { x: v, y: line.start.y }),
        }
    }
    points
}
