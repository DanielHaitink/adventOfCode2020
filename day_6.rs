use crate::input::read_input_until_empty;
use std::collections::HashMap;

struct Form {
    map: HashMap<char, u16>,
    group_size: usize,
}

pub fn day_6(part: i8) {
    let input = read_input();

    if part == 1 {
        part_one(&input);
    } else {
        part_two(&input);
    }
}

fn part_one(maps: &Vec<Form>) {
    let mut output: usize = 0;
    for map in maps {
        output += map.map.len();
    }

    println!("Count: {}", output)
}

fn part_two(forms: &Vec<Form>) {
    let mut output: usize = 0;

    for form in forms {
        for item in &form.map {
            if *(item.1) as usize == form.group_size {
                output += 1;
            }
        }
    }

    println!("Count: {}", output)
}

fn read_input() -> Vec<Form> {
    let mut maps = Vec::new();
    let mut input = read_input_until_empty();
    while input.len() > 0 {
        maps.push(parse_form(&input));
        input = read_input_until_empty();
    }

    return maps;
}

fn parse_form(input: &Vec<String>) -> Form {
    let mut map: HashMap<char, u16> = HashMap::new();

    for line in input {
        for char in line.chars() {
            if map.contains_key(&char) {
                *map.get_mut(&char).unwrap() += 1;
            } else {
                map.insert(char, 1);
            }
        }
    }

    return Form{
        map,
        group_size: input.len()
    };
}