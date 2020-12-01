use std::io;
use std::io::BufRead;

pub fn read_input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

pub fn read_input_int() -> isize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

pub fn read_input_indefinitely(function: fn(str: String)) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        function(line.unwrap());
    }
}