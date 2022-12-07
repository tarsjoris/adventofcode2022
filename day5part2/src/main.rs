use std::fs::File;
use std::io::{self, BufRead};

struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Stack { crates: Vec::new() }
    }

    fn remove_crates(&mut self, count: usize) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();
        for _i in 00..count {
            let moving_crate = match self.crates.pop() {
                None => panic!("Not enough crates on stack"),
                Some(c) => c,
            };
            result.push(moving_crate);
        }
        result
    }

    fn add_crates(&mut self, new_crates: Vec<char>) {
        new_crates.iter().rev().for_each(|c| self.crates.push(*c));
    }
}

struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    fn new(stack_count: usize) -> Stacks {
        let mut stacks: Vec<Stack> = Vec::new();
        for _i in 00..stack_count {
            stacks.push(Stack::new());
        }
        Stacks { stacks }
    }

    fn perform(&mut self, instruction: &Instruction) {
        println!(
            "Move {} from {} to {}",
            instruction.count, instruction.from, instruction.to
        );
        let moving_crates = self.stacks[instruction.from].remove_crates(instruction.count);
        self.stacks[instruction.to].add_crates(moving_crates);
        self.print();
    }

    fn print(&self) {
        self.stacks.iter().for_each(|s| {
            print!("[");
            s.crates.iter().for_each(|c| print!("{c}"));
            print!("]");
        });
        println!();
    }

    fn get_top_stacks(&self) -> String {
        let tops: Vec<String> = self
            .stacks
            .iter()
            .map(|s| s.crates.last())
            .map(|t| match t {
                None => &' ',
                Some(c) => c,
            })
            .map(|c| c.to_string())
            .collect();
        tops.join("")
    }
}
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let (mut state, instructions) = read_input("input.txt");
    state.print();
    instructions.iter().for_each(|i| state.perform(i));
    state.print();
    println!("Top of stacks: {}", state.get_top_stacks());
}

fn parse_start_state(lines: Vec<&String>) -> Stacks {
    let stack_count = match lines.last() {
        None => 0,
        Some(l) => (l.len() + 1) / 4,
    };
    println!("Stack count = {}", stack_count);
    let mut stacks = Stacks::new(stack_count);
    for line in lines.iter().rev() {
        for i in 0..stack_count {
            let char = line.chars().nth(1 + i * 4);
            if let Some(c) = char {
                if c != ' ' {
                    stacks.stacks[i].crates.push(c);
                }
            }
        }
    }
    stacks
}

fn parse_instruction(line: &String) -> Instruction {
    let mut parts = line.split(" ");
    assert_word(parts.next(), "move");
    let count = parse_instruction_part(parts.next(), "count");
    assert_word(parts.next(), "from");
    let from = parse_instruction_part(parts.next(), "from") - 1;
    assert_word(parts.next(), "to");
    let to = parse_instruction_part(parts.next(), "to") - 1;

    Instruction { count, from, to }
}

fn assert_word(word: Option<&str>, expected: &str) {
    match word {
        Some(a) => {
            if a != expected {
                panic!("Expected '{}', but found {}", expected, a)
            }
        }
        None => panic!("Expected 'move''"),
    }
}

fn parse_instruction_part(word: Option<&str>, name: &str) -> usize {
    let number = match word {
        None => panic!("{} is missing", name),
        Some(a) => a.to_string().parse::<usize>(),
    };
    match number {
        Err(why) => panic!("Could not parse {} as usize {}", name, why),
        Ok(i) => i,
    }
}

fn read_input(filename: &str) -> (Stacks, Vec<Instruction>) {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    let unwrapped_lines: Vec<String> = lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => l,
        })
        .collect();

    let start_state_lines: Vec<&String> = unwrapped_lines
        .iter()
        .take_while(|l| !(**l).starts_with(" 1"))
        .collect();
    let start_state = parse_start_state(start_state_lines);
    let instructions = unwrapped_lines
        .iter()
        .skip_while(|l| !l.starts_with("move"))
        .map(parse_instruction)
        .collect();
    (start_state, instructions)
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
