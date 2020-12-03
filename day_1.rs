use std::io;
use std::io::BufRead;
use std::process::exit;

pub fn day_1(part: i8) {
    let lines = read_input();

    if part == 1 {
        part_one(lines);
    } else {
        part_two(lines);
    }
}

fn part_one(lines: Vec<isize>) {
    for line in &lines {
        for line2 in &lines {
            if *line == *line2 {
                continue;
            }

            if *line + *line2 == 2020 {
                let solution = *line * *line2;
                println!("{}", solution);
                return;
            }
        }
    }
}

fn part_two(lines: Vec<isize>) {
    for line in &lines {
        for line2 in &lines {
            for line3 in &lines {
                if *line == *line2 || *line == *line3 || *line2 == *line3 {
                    continue;
                }

                if *line + *line2 + *line3 == 2020 {
                    let solution = *line * *line2 * *line3;
                    println!("{}", solution);
                    return;
                }
            }
        }
    }
}

fn read_input() -> Vec<isize> {
    let stdin = io::stdin();

    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let ln = line.unwrap_or_default();

        if ln.is_empty() {
            return lines;
        }

        let value = ln.trim().parse();
        if value.is_err() {
            println!("Input error!");
            exit(-1);
        }

        lines.push(value.unwrap());
    }

    return lines;
}