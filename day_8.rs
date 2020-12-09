use crate::input::read_input_until_empty;


#[derive(Copy, Clone)]
enum Command {
    Accumulate(i32),
    Jump(i32),
    Nop(i32)
}

#[derive(Clone)]
struct Code {
    code: Vec<Command>,
    index: usize,
}

pub fn day_8(part: i8) {
    let mut code = read_code(read_input_until_empty());

    if part == 1 {
        part_one(&mut code);
    } else {
        part_two(&mut code);
    }
}

fn part_one(code: &mut Code) {
    run_code_one(code);
}

fn read_code(code_text: Vec<String>) -> Code {
    let mut code: Vec<Command> = Vec::new();

    for line in code_text {
        let split: Vec<&str> = line.split(" ").collect();
        let number: i32 = split[1].parse().unwrap();

        match split[0] {
            "acc" => code.push(Command::Accumulate(number)),
            "jmp" => code.push(Command::Jump(number)),
            "nop" => code.push(Command::Nop(number)),
            &_ => {}
        }
    }

    return Code{code, index: 0};
}

fn run_code_one(code: &mut Code) {
    let mut accumulator = 0;
    let mut accessed_lines: Vec<usize> = Vec::new();

    loop {
        if accessed_lines.contains(&code.index) {
            println!("ACC:{}", accumulator);
            break;
        }

        accessed_lines.push(code.index);

        let operation = &code.code[code.index];
        match operation {
            Command::Accumulate(num) => {accumulator += num; code.index += 1}
            Command::Jump(num) => {code.index = ((code.index) as i32 + num) as usize}
            Command::Nop(_) => {code.index += 1}
        }
    }
}

fn find_fix(code: &mut Code) {
    for i in 0..code.code.len() {
        let command = &code.code[i];
        match command {
            Command::Accumulate(_) => {}
            Command::Jump(val) => {
                let mut code_copy = code.clone();
                let _ = std::mem::replace(&mut code_copy.code[i], Command::Nop(*val));
                if run_code_two(&mut code_copy) {
                    return;
                }
            }
            Command::Nop(val) => {
                let mut code_copy = code.clone();
                let _ = std::mem::replace(&mut code_copy.code[i], Command::Jump(*val));
                if run_code_two(&mut code_copy) {
                    return;
                }
            }
        }
    }
}

fn run_code_two(code: &mut Code) -> bool {
    let mut accumulator = 0;
    let mut accessed_lines: Vec<usize> = Vec::new();

    loop {
        if code.index >= code.code.len() {
            println!("Stopped Peacefully!\nAcc:{}", accumulator);
            return true;
        }
        if accessed_lines.contains(&code.index) {
            return false;
        }

        accessed_lines.push(code.index);

        let operation = &code.code[code.index];
        match operation {
            Command::Accumulate(num) => {accumulator += num; code.index += 1}
            Command::Jump(num) => {code.index = ((code.index) as i32 + num) as usize}
            Command::Nop(_) => {code.index += 1}
        }
    }
}

fn part_two(code: &mut Code) {
    find_fix(code);
}