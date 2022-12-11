use std::fs::File;
use std::io::{self, BufRead};

struct State {
    last_cycle: u32,
    register_x_value: i32,
    sum_of_signal_strengths: i32,
}

impl State {
    fn new() -> Self {
        State {
            last_cycle: 0,
            register_x_value: 1,
            sum_of_signal_strengths: 0,
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
        let signal_strength = match current_cycle % 40 {
            20 => self.register_x_value * (current_cycle as i32),
            _ => 0,
        };
        println!("During cycle {}, X = {} (strength = {})", current_cycle, self.register_x_value, signal_strength);
        let new_state = State {
            last_cycle: current_cycle,
            register_x_value: self.register_x_value + increment,
            sum_of_signal_strengths: self.sum_of_signal_strengths + signal_strength,
        };
        new_state
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
    println!("Sum of signal strengths = {}", final_state.sum_of_signal_strengths);
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
