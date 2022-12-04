use std::fs;

pub fn run() {
    let data = fs::read_to_string("dec_04.txt").expect("unable to read file");

    let mut overlaps = 0;
    let mut contains = 0;
    for line in data.lines() {
        let pairs: Vec<&str> = line.split(|c| c == '-' || c == ',').collect();

        let a1 = pairs[0].parse::<i32>().unwrap();
        let a2 = pairs[1].parse::<i32>().unwrap();
        let b1 = pairs[2].parse::<i32>().unwrap();
        let b2 = pairs[3].parse::<i32>().unwrap();

        if overlaps_at_all(a1, a2, b1, b2) {
            overlaps += 1;
        }

        if fully_contain(a1, a2, b1, b2) {
            contains += 1;
        }
    }
    println!("Part1 = {}", contains);
    println!("Part2 = {}", overlaps);
}

fn overlaps_at_all(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    //If a ends in range b
    //Or a begins in range b
    //Or a is fully contained by b
    if a2 >= b1 && b2 >= a2 {
        return true;
    } else if a1 >= b1 && b2 >= a1 {
        return true;
    } else if a1 <= b1 && b2 <= a2 {
        return true;
    } else {
        return false;
    }
}
fn fully_contain(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    if a1 <= b1 && b2 <= a2 {
        return true;
    } else if b1 <= a1 && a2 <= b2 {
        return true;
    } else {
        return false;
    }
}
