use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const MIDDLE: u32 = u32::MAX / 2;
const KNOT_COUNT: usize = 10;

fn position_to_u64(x: u32, y: u32) -> u64 {
    ((x as u64) << 32) | (y as u64)
}

struct Direction {
    dx: i8,
    dy: i8,
}

const DIR_UP: Direction = Direction { dx: 0, dy: -1 };
const DIR_RIGHT: Direction = Direction { dx: 1, dy: 0 };
const DIR_DOWN: Direction = Direction { dx: 0, dy: 1 };
const DIR_LEFT: Direction = Direction { dx: -1, dy: 0 };

#[derive(Copy, Clone)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new() -> Self {
        Position {
            x: MIDDLE,
            y: MIDDLE,
        }
    }

    fn move_to(&mut self, direction: &Direction) {
        self.x = (self.x as i64 + direction.dx as i64) as u32;
        self.y = (self.y as i64 + direction.dy as i64) as u32;
    }

    fn to_u64(&self) -> u64 {
        position_to_u64(self.x, self.y)
    }
}

struct Field {
    visited_positions: HashSet<u64>,
    knots: [Position; KNOT_COUNT],
    min_x: u32,
    min_y: u32,
    max_x: u32,
    max_y: u32,
}

impl Field {
    fn new() -> Self {
        let mut field = Field {
            visited_positions: HashSet::new(),
            knots: [Position::new(); KNOT_COUNT],
            min_x: MIDDLE,
            min_y: MIDDLE,
            max_x: MIDDLE,
            max_y: MIDDLE,
        };
        field
            .visited_positions
            .insert(field.knots[KNOT_COUNT - 1].to_u64());
        field
    }

    fn move_head(&mut self, direction: &Direction) {
        self.knots[0].move_to(&direction);
        if self.knots[0].x < self.min_x {
            self.min_x = self.knots[0].x
        } else if self.knots[0].x > self.max_x {
            self.max_x = self.knots[0].x
        }
        if self.knots[0].y < self.min_y {
            self.min_y = self.knots[0].y
        } else if self.knots[0].y > self.max_y {
            self.max_y = self.knots[0].y
        }

        (1..KNOT_COUNT).for_each(|knot_index| {
            let dx = self.knots[knot_index - 1].x as i64 - self.knots[knot_index].x as i64;
            let dy = self.knots[knot_index - 1].y as i64 - self.knots[knot_index].y as i64;
            if dx > 1 || dx < -1 {
                if dx > 1 {
                    self.knots[knot_index].x += 1;
                } else {
                    self.knots[knot_index].x -= 1;
                }

                if dy > 1 {
                    self.knots[knot_index].y += 1;
                } else if dy < -1 {
                    self.knots[knot_index].y -= 1;
                } else {
                    self.knots[knot_index].y = self.knots[knot_index - 1].y;
                }
            } else if dy > 1 || dy < -1 {
                if dy > 1 {
                    self.knots[knot_index].y += 1;
                } else {
                    self.knots[knot_index].y -= 1;
                }

                if dx > 1 {
                    self.knots[knot_index].x += 1;
                } else if dx < -1 {
                    self.knots[knot_index].x -= 1;
                } else {
                    self.knots[knot_index].x = self.knots[knot_index - 1].x;
                }
            }
        });

        self.visited_positions
            .insert(self.knots[KNOT_COUNT - 1].to_u64());
    }

    fn get_visited_cell_count(&self) -> usize {
        self.visited_positions.len()
    }

    fn print(&self) {
        println!();
        (self.min_y..=self.max_y).for_each(|y| {
            (self.min_x..=self.max_x).for_each(|x| {
                print!(
                    "{}",
                    if x == self.knots[0].x && y == self.knots[0].y {
                        "H".to_string()
                    } else {
                        let knot = (0..KNOT_COUNT)
                            .filter(|knot_index| {
                                x == self.knots[*knot_index].x && y == self.knots[*knot_index].y
                            })
                            .next();
                        match knot {
                            Some(knot_index) => knot_index.to_string(),
                            None => {
                                if self.visited_positions.contains(&position_to_u64(x, y)) {
                                    "X".to_string()
                                } else {
                                    ".".to_string()
                                }
                            }
                        }
                    }
                )
            });
            println!();
        });
        println!();
    }
}

fn main() {
    let lines = read_input("input.txt");
    let mut field = Field::new();
    field.print();

    lines.into_iter().for_each(|l| execute_line(&mut field, &l));
    field.print();

    let visited_cell_count = field.get_visited_cell_count();
    println!("Visited cell count {visited_cell_count}");
}

fn execute_line(field: &mut Field, line: &String) {
    let mut parts = line.split(" ");
    let direction = match parts.next() {
        None => panic!("Expected direction"),
        Some(d) => match d {
            "U" => DIR_UP,
            "R" => DIR_RIGHT,
            "D" => DIR_DOWN,
            "L" => DIR_LEFT,
            _ => panic!("Unknown direction '{d}'"),
        },
    };
    let count = match parts.next() {
        None => panic!("Expected count"),
        Some(c) => match c.parse::<u8>() {
            Err(why) => panic!("Invalid count {}: {}", c, why),
            Ok(count) => count,
        },
    };
    (0..count).for_each(|_| field.move_head(&direction));
}

fn read_input(filename: &str) -> Vec<String> {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    return lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => l,
        })
        .collect();
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
