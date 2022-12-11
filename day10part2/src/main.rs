use std::fs::File;
use std::io::{self, BufRead};

const WIDTH: u32 = 40;

struct State {
    last_cycle: u32,
    register_x_value: i32,
    crt: String,
}

impl State {
    fn new() -> Self {
        State {
            last_cycle: 0,
            register_x_value: 1,
            crt: String::new(),
        }
    }

    fn process_instruction(self, instruction: &String) -> Self {
        let new_state = self.advance_cycle(0);
        if instruction == "noop" {
            new_state
        } else {
            let increment = get_increment(instruction);
            new_state.advance_cycle(increment)
        }
    }

    fn advance_cycle(self, increment: i32) -> Self {
        let current_cycle = self.last_cycle + 1;
        let current_column = (current_cycle % WIDTH) as i32;
        let pixel =
            if (self.register_x_value..=(self.register_x_value + 2)).contains(&current_column) {
                'X'
            } else {
                '.'
            };
        let mut new_crt = String::new();
        new_crt.push_str(&self.crt);
        new_crt.push(pixel);
        State {
            last_cycle: current_cycle,
            register_x_value: self.register_x_value + increment,
            crt: new_crt,
        }
    }

    fn print(&self) {
        println!();
        (0..self.crt.len()).for_each(|i| {
            print!("{}", self.crt.chars().nth(i).unwrap_or(' '));
            if i as u32 % WIDTH == WIDTH - 1 {
                println!()
            }
        });
        println!();
        (0..WIDTH).for_each(|i| {
            print!(
                "{}",
                if (self.register_x_value..=(self.register_x_value + 2)).contains(&(i as i32 + 1)) {
                    "@"
                } else {
                    "."
                }
            )
        });
        println!();
    }
}

fn get_increment(instruction: &String) -> i32 {
    let mut parts = instruction.split(" ");
    match parts.next() {
        None => panic!("Expected 'addx' but found nothing"),
        Some(p) => {
            if p != "addx" {
                panic!("Expected 'addx' but found '{p}'")
            }
        }
    }
    let increment = match parts.next() {
        None => panic!("Expected value but found nothing"),
        Some(i) => i,
    };
    match increment.parse::<i32>() {
        Err(why) => panic!("Invalid value {increment}: {why}"),
        Ok(i) => i,
    }
}

fn main() {
    let lines = read_input("input.txt");
    let final_state = lines.iter().fold(State::new(), |old_state, instruction| {
        old_state.process_instruction(instruction)
    });
    final_state.print();
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
