use std::io;
use std::io::BufRead;
// use std::fs::read;

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
    }}

fn read_input() -> Vec<isize>{
    let stdin = io::stdin();

    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let ln = line.unwrap();

        if ln.is_empty() {
        //     return
            return lines;
        }

        lines.push(ln.trim().parse().unwrap());
        // println!("{}", line.unwrap())
    }

    return lines
}