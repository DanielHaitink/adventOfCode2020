use crate::input::read_input_until_empty_as;
use std::time;

pub fn day_10(part: i8) {
    let mut jolts: Vec<usize> = read_input_until_empty_as();
    let time = std::time::SystemTime::now();
    jolts.sort();

    if part == 1 {
        part_one(&jolts);
    } else {
        part_two(&mut jolts);
    }

    println!("TIME:{:?}", time.elapsed());
}

fn part_one(jolts: &Vec<usize>) {
    let target = jolts[jolts.len() - 1] + 3;
    let mut delta_jolts = vec![0, 0, 0];

    for i in 0..jolts.len() {
        let mut jolt_diff = jolts[i];
        if i > 0 {
            jolt_diff -= jolts[i - 1];
        }

        delta_jolts[jolt_diff - 1] += 1;
    }

    delta_jolts[target - jolts[jolts.len() - 1] - 1] += 1;
    let solution = delta_jolts[0] * delta_jolts[2];

    println!("Diffs{:?}\nSolution:{}", delta_jolts, solution);
}

fn part_two(jolts: &mut Vec<usize>) {
    let target = jolts[jolts.len() - 1] + 3;
    jolts.push(target);
    jolts.insert(0, 0);

    let arrangements = find_arrangement(&jolts);
    println!("Arrangements:{}", arrangements);
}

fn find_arrangement(adapters: &Vec<usize>) -> usize {
    let mut arrangements: Vec<usize> = vec![0; adapters.len()];
    arrangements[0] = 1;

    for i in 1..adapters.len() {
        let mut i_max: usize = 0;
        if i > 4 {
            i_max = i - 4;
        }

        for j in i_max..i {
            if adapters[i] - adapters[j] <= 3 {
                arrangements[i] += arrangements[j];
            }
        }
    }

    return arrangements[arrangements.len() - 1];
}

fn find_arrangement_rec(index: usize, adapters: &Vec<usize>) -> usize {
    if index >= adapters.len() - 1 {
        println!("{}", index);
        return 1;
    }
    let mut value = 0;
    for i in index + 1..index + 4 {
        if i >= adapters.len() {
            return value;
        }

        if adapters[i] - adapters[index] <= 3 {
            value += find_arrangement_rec(i, adapters);
        }
    }

    return value;
}

