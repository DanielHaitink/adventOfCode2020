use crate::input::read_input_until_empty_as;
use std::borrow::Borrow;
use std::cmp::{min, max};

#[derive(Clone)]
struct Grid {
    field: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(field: Vec<String>) -> Grid {
        let mut char_vec: Vec<Vec<char>> = Vec::new();
        let height = field.len();
        let width = field[0].len();

        for string in field {
            let mut x_vec = Vec::new();

            for char in string.chars() {
                x_vec.push(char);
            }

            char_vec.push(x_vec);
        }

        return Grid{field: char_vec, width, height};
    }

    fn print(self: &Grid) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.field[y][x]);
            }
            print!("\n");
        }
    }

    fn adjacent_get(self: &Grid, x: isize, y: isize, find_char: char) -> usize {
        let mut adj = 0;

        for d_x in x - 1..x + 2 {
            for d_y in y - 1..y + 2 {
                if d_x < 0 || d_y < 0 || d_x >= self.width as isize || d_y >= self.height as isize {
                    continue;
                }
                if !(d_x == x && d_y == y) && self.field[d_y as usize][d_x as usize] == find_char {
                    adj += 1;
                }
            }
        }

        return adj;
    }

    fn adjacent_free(self: &Grid, x: usize, y: usize) -> bool {
        self.adjacent_occupied(x, y) == 0
    }

    fn adjacent_occupied(self: &Grid, x: usize, y: usize) -> usize {
        self.adjacent_get(x as isize, y as isize, '#')
    }

    fn count_occupied(self: &Grid) -> usize {
        let mut count = 0;

        for line in &self.field {
            for c in line {
                if *c == '#' {
                    count += 1;
                }
            }
        }

        return count;
    }
}

pub fn day_11(part: i8) {
    let mut grid = Grid::new(read_input_until_empty_as());

    if part == 1 {
        part_one(&mut grid);
    } else {
        part_two();
    }
}

fn part_two() {
    unimplemented!()
}

fn part_one(grid: &mut Grid) {
    let mut changed = true;

    while changed {
        changed = false;
        let copy = grid.clone();

        for y in 0..grid.height {
            for x in 0..grid.width {
                match copy.field[y][x] {
                    'L' => {
                        if copy.adjacent_free(x, y) {
                            grid.field[y][x] = '#';
                            changed = true;
                        }
                    },
                    '#' => {
                        if copy.adjacent_occupied(x, y) >= 4 {
                            grid.field[y][x] = 'L';
                            changed = true;
                        }
                    },
                    '.' => {},
                    _ => {print!("NO")}
                }
            }
        }
    }

    println!("Occupied: {}", grid.count_occupied());
}