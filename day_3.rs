use crate::input::read_input_until_empty;

#[derive(Clone)]
#[derive(PartialEq)]
enum Square {
    Open,
    Tree,
    Empty,
    Count,
}

struct Area {
    map: Vec<Vec<Square>>,
    height: usize,
    width: usize,
}

impl Area {
    fn print(&self) {
        for line in &self.map {
            for sq in line {
                match sq {
                    Square::Open => print!("."),
                    Square::Tree => print!("#"),
                    Square::Empty => print!("?"),
                    Square::Count => print!("?"),
                }
            }
            print!("\n");
        }
    }
}

pub fn day_3(part: i8) {
    let lines = read_input_until_empty();
    let area = parse_input(&lines);

    area.print();

    if part == 1 {
        part_one(&area);
    } else {
        part_two(&area);
    }
}

fn part_two(area: &Area) {
    let sol = check_steps(area, 1, 1) *
        check_steps(area, 3, 1) *
        check_steps(area, 5, 1) *
        check_steps(area, 7, 1) *
        check_steps(area, 1, 2);

    println!("Solution: {}", sol);
}

fn check_steps(area: &Area, x_inc: usize, y_inc: usize) -> usize {
    let mut y: usize = 0;
    let mut x: usize = 0;
    let mut trees: usize = 0;

    while y != area.height - 1 {
        y += y_inc;
        x += x_inc;

        match area.map[x % (area.width)][y] {
            Square::Open => {}
            Square::Tree => trees += 1,
            Square::Empty => {}
            Square::Count => {}
        }
    }

    return trees;
}

fn part_one(area: &Area) {
    let mut y: usize = 0;
    let mut x: usize = 0;
    let mut trees: usize = 0;

    while y != area.height - 1 {
        y += 1;
        x += 3;

        match area.map[x % (area.width)][y] {
            Square::Open => {}
            Square::Tree => trees += 1,
            Square::Empty => {}
            Square::Count => {}
        }
    }

    println!("Trees:{}", trees);
}

fn parse_input(lines: &Vec<String>) -> Area {
    let mut area = Area {
        map: vec![vec![Square::Empty; lines.len()]; lines[0].len()],
        height: lines.len(),
        width: lines[0].len(),
    };

    let mut idx: usize = 0;
    for line in lines {
        parse_line(line, &mut area, &idx);
        idx += 1;
    }

    return area;
}

fn parse_line(line: &String, area: &mut Area, x: &usize) {
    let mut idx: usize = 0;
    for char in line.chars() {
        match char {
            '#' => area.map[idx][*x] = Square::Tree,
            '.' => area.map[idx][*x] = Square::Open,
            _ => {}
        }
        idx += 1;
    }
}