use itertools::Itertools;
use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_03.txt").expect("unable to read file");

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
fn part1(data: &String) -> i32 {
    let mut sum: i32 = 0;

    for line in data.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let mut item = ' ';
        for c in first.chars() {
            match second.find(c) {
                Some(0..=99) => item = c,
                _ => {}
            }
        }
        match item.is_ascii_lowercase() {
            true => {
                sum += item as i32 - 96;
            }
            false => {
                sum += item as i32 - 38;
            }
        }
    }

    sum
}

fn part2(data: &String) -> i32 {
    let mut sum = 0;

    for line in &data.lines().chunks(3) {
        let group: Vec<_> = line.collect();
        let mut item = ' ';

        for c in group[0].chars() {
            match group[1].find(c) {
                Some(0..=99) => match group[2].find(c) {
                    Some(0..=99) => {
                        item = c;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        match item.is_ascii_lowercase() {
            true => {
                sum += item as i32 - 96;
            }
            false => {
                sum += item as i32 - 38;
            }
        }
    }

    sum
}
