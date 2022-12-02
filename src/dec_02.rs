use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_02.txt").expect("unable to read file");

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
    //part2(&data);
}

fn part1(data: &String) -> i32 {
    let mut total_score = 0;
    for line in data.lines() {
        let mut round = line.split_whitespace();

        let p1 = round.next();
        let p2 = round.next();

        let mut score = 0;
        match p1 {
            Some("A") => {
                match p2 {
                    Some("X") => score += 4, //+1 for X +3 for draw
                    Some("Y") => score += 8, //+2 for Y +6 for win
                    Some("Z") => score += 3, //+3 for Z +0 for loss
                    _ => print!("Bad data"),
                }
            }
            Some("B") => {
                match p2 {
                    Some("X") => score += 1, //+1 for X +0 for loss
                    Some("Y") => score += 5, //+2 for Y +3 for draw
                    Some("Z") => score += 9, //+3 for Z +6 for win
                    _ => print!("Bad data"),
                }
            }
            Some("C") => {
                match p2 {
                    Some("X") => score += 7, //+1 for X +6 for win
                    Some("Y") => score += 2, //+2 for Y +0 for loss
                    Some("Z") => score += 6, //+3 for Z +3 for draw
                    _ => print!("Bad data"),
                }
            }
            _ => print!("Bad data"),
        }

        total_score += score;
    }
    total_score
}

fn part2(data: &String) -> i32 {
    let mut total_score = 0;
    for line in data.lines() {
        let mut round = line.split_whitespace();

        let p1 = round.next();
        let p2 = round.next();

        let mut score = 0;
        match p1 {
            Some("A") => {
                match p2 {
                    Some("X") => score += 3, //+3 lose via Scissors
                    Some("Y") => score += 4, //+4 draw via Rock
                    Some("Z") => score += 8, //+8 win via Paper
                    _ => print!("Bad data"),
                }
            }
            Some("B") => {
                match p2 {
                    Some("X") => score += 1, //+1 lose via Rock
                    Some("Y") => score += 5, //+5 draw via Paper
                    Some("Z") => score += 9, //+9 win via Scissors
                    _ => print!("Bad data"),
                }
            }
            Some("C") => {
                match p2 {
                    Some("X") => score += 2, //+2 lose via Paper
                    Some("Y") => score += 6, //+6 draw via Scissors
                    Some("Z") => score += 7, //+7 win via Rock
                    _ => print!("Bad data"),
                }
            }
            _ => print!("Bad data"),
        }

        total_score += score;
    }
    total_score
}
