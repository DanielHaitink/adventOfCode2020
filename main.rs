extern crate itertools;

pub mod input;
mod day_1;

fn main() {
    println!("Which day do you want to run?");

    match input::read_input_int() {
        1 => day_1::day_1(read_part_day()),
        _ => (print!("Day not found"))
    }
}

fn read_part_day() -> i8 {
    println!("Which part do you want to run?");
    let part = input::read_input_int() as i8;
    return part;
}