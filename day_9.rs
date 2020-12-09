use crate::input::{read_input_until_empty, read_input_until_empty_as};

pub fn day_9(part: i8) {
    let input: Vec<usize> = read_input_until_empty_as();

    if part == 1 {
        part_one(&input);
    } else {
        part_two(&input);
    }
}

fn part_one(input: &Vec<usize>) {
    for i in 25..input.len() {
        let slice = &input[i - 25..i];
        if !has_sum(slice, input[i]) {
            println!("No sum: {}", input[i]);
            return;
        }
    }
}

fn find_contagious(input: &Vec<usize>, goal_index: usize) {
    let goal = input[goal_index];
    for i in 0..goal_index - 2 {
        for j in i + 1..goal_index - 1 {
            let sum: usize = input[i..j].iter().sum();
            if sum == goal {
                let weak: usize = input[i..j].iter().min().unwrap() + input[i..j].iter().max().unwrap();
                println!("Weakness! ==> {}", weak);
            }
        }
    }
}

fn has_sum(vec: &[usize], solution: usize) -> bool {
    for i in 0..vec.len() - 1 {
        for j in i + 1..vec.len() {
            if vec[i] + vec[j] == solution {
                return true;
            }
        }
    }
    return false;
}

fn part_two(input: &Vec<usize>) {
    for i in 25..input.len() {
        let slice = &input[i - 25..i];
        if !has_sum(slice, input[i]) {
            println!("No sum: {}", input[i]);
            find_contagious(input, i);
            return;
        }
    }
}