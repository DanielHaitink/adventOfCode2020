extern crate itertools;

pub mod input;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    loop {
        println!("Which day do you want to run?\nChoose 0 to exit");

        match input::read_input_int() {
            0 => break,
            1 => day_1::day_1(read_part_day()),
            2 => day_2::day_2(read_part_day()),
            3 => day_3::day_3(read_part_day()),
            4 => day_4::day_4(read_part_day()),
            5 => day_5::day_5(read_part_day()),
            _ => (println!("Day not found"))
        }
    }
}

fn read_part_day() -> i8 {
    println!("Which part do you want to run?");
    let part = input::read_input_int() as i8;
    return part;
}