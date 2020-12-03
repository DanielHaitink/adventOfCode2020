use crate::input::read_input_until_empty;
use std::ops::BitXor;


struct Password {
    password: String,
    min: isize,
    max: isize,
    policy_letter: char,
}

pub fn day_2(part: i8) {
    let lines = read_input_until_empty();

    if part == 1 {
        part_one(lines);
    } else {
        part_two(lines);
    }
}

fn part_one(lines: Vec<String>) {
    let mut incorrect = 0;
    let mut correct = 0;

    let passwords = get_passwords(&lines);
    for password in &passwords {
        let count = count_occurrences(&password.password, &password.policy_letter);

        if count > password.max || count < password.min {
            incorrect += 1;
        } else {
            correct += 1
        }
    }
    println!("Correct:{}\nIncorrect:{}", correct, incorrect);
}

fn get_passwords(lines: &Vec<String>) -> Vec<Password> {
    let mut passwords: Vec<Password> = Vec::new();
    for line in lines {
        let split_line: Vec<&str> = (*line).split(": ").collect();
        let policy_split: Vec<&str> = split_line[0].split(' ').collect();
        let policy_min_max: Vec<&str> = policy_split[0].split('-').collect();

        passwords.push(Password {
            password: String::from(split_line[1]),
            min: policy_min_max[0].trim().parse().unwrap(),
            max: policy_min_max[1].trim().parse().unwrap(),
            policy_letter: policy_split[1].chars().nth(0).unwrap(),
        });
    }

    return passwords;
}

fn count_occurrences(password: &str, letter: &char) -> isize {
    let mut count: isize = 0;
    for l in password.chars() {
        if l == *letter {
            count += 1;
        }
    }

    return count;
}

fn part_two(lines: Vec<String>) {
    let mut incorrect = 0;
    let mut correct = 0;

    let passwords = get_passwords(&lines);
    for password in &passwords {
        let pos_1 = password.password.chars().nth((password.min - 1) as usize).unwrap() == password.policy_letter;
        let mut pos_2 = false;
        if password.password.len() >= (password.max - 1) as usize {
            pos_2 = password.password.chars().nth((password.max - 1) as usize).unwrap() == password.policy_letter;
        }

        if pos_2.bitxor(pos_1) {
            correct += 1;
        } else {
            incorrect += 1;
        }
    }

    println!("Correct:{}\nIncorrect:{}", correct, incorrect);
}