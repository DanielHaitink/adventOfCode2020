use crate::input::read_input_until_empty;
use std::collections::HashMap;

struct Seat {
    id: i16,
}

pub fn day_5(part: i8) {
    let input = read_input_until_empty();
    let seats = parse_input(&input);

    if part == 1 {
        part_one(seats);
    } else {
        part_two(seats);
    }
}

fn part_two(seats: Vec<Seat>) {
    let mut map: HashMap<i16, Seat> = HashMap::new();

    for seat in seats {
        map.insert(seat.id, seat);
    }

    for row in 1..126 {
        for col in 0..7 {
            let id = row * 8 + col;

            if !map.contains_key(&id) {
                println!("Missing id:{}\t{},{}", id, row, col);
            }
        }
    }
}

fn part_one(seats: Vec<Seat>) {
    let mut highest_id = 0;

    for seat in seats {
        if seat.id > highest_id {
            highest_id = seat.id;
        }
    }

    println!("Highest:{}", highest_id);
}

fn parse_input(input: &Vec<String>) -> Vec<Seat> {
    let mut seats = Vec::new();
    for line in input {
        let split = line.split_at(7);
        let row = string_to_number(split.0);
        let column = string_to_number(split.1);
        let id: i16 = (row) as i16 * 8 + (column) as i16;

        seats.push(Seat{id});
    }

    return seats;
}

fn string_to_number(input: &str) -> i8 {
    let bin_number = input.replace("F", "0").
        replace("B", "1").
        replace("R", "1").
        replace("L", "0");
    return i8::from_str_radix(&*bin_number, 2).unwrap();
}