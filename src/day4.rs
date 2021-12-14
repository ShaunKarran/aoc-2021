use ::std::fs;

/// A 5 by 5 board of numbers. Stored row-wise.
#[derive(Clone, Copy, Debug)]
struct Board {
    rowwise: [[i32; 5]; 5],
    colwise: [[i32; 5]; 5],
}

impl Board {
    fn new() -> Self {
        Board { rowwise: [[0; 5]; 5], colwise: [[0; 5]; 5] }
    }

    fn is_winner(self, number_draw: &Vec<i32>) -> bool {
        self.rowwise.iter().any(|row| row.iter().all(|number| number_draw.contains(&number)))
        || self.colwise.iter().any(|column| column.iter().all(|number| number_draw.contains(&number)))
    }

    fn calculate_score(self, number_draw: &Vec<i32>) -> i32 {
        let mut score = 0;
        for row in self.rowwise {
            for number in row {
                if !number_draw.contains(&number) {
                    score += number;
                }
            }
        }
        score
    }
}

fn main() {
    let input = fs::read_to_string("input/day4")
        .expect("Something went wrong reading the file.")
        .lines()
        .map(|line| String::from(line))
        .collect::<Vec<String>>();

    let numbers = input[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = Vec::new();
    for lines in input[2..].chunks(6) {
        let mut board = Board::new();
        for (i, line) in lines[..5].iter().enumerate() {
            let numbers = line.split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<i32>>();
            for (j, number) in numbers.iter().enumerate() {
                board.rowwise[i][j] = *number;
                board.colwise[j][i] = *number;
            }
        }
        boards.push(board);
    }

    part1(&numbers, &boards);
    // part2(&input);
}

fn part1(numbers: &Vec<i32>, boards: &Vec<Board>) {
    let mut number_draw = Vec::new();
    for number in numbers {
        number_draw.push(*number);

        let winning_board = boards.iter().find(|board| board.is_winner(&number_draw));
        dbg!(winning_board);
        let score: Option<i32> = match winning_board {
            Some(board) => Some(board.calculate_score(&number_draw)),
            None => None,
        };

        if score.is_some() {
            dbg!(score.unwrap() * number_draw.last().unwrap());
            return;
        }
    }
}

// fn part2(number_draw: &Vec<i32>, boards: &Vec<Board>) {
//     println!("Part 2: {}", );
// }
