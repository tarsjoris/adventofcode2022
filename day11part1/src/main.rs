use std::fs::File;
use std::io::{self, BufRead};

const MONKEY_PREFIX: &str = "Monkey ";
const ITEMS_PREFIX: &str = "  Starting items: ";
const OPERATION_PREFIX: &str = "  Operation: new = old ";
const DIVISIBLE_PREFIX: &str = "  Test: divisible by ";
const TRUE_PREFIX: &str = "    If true: throw to monkey ";
const FALSE_PREFIX: &str = "    If false: throw to monkey ";

const ROUNDS: usize = 20;
const BORE_FACTOR: u32 = 3;

type Operation = Box<dyn Fn(u32) -> u32>;

struct MonkeyDescription {
    operation: Operation,
    divisible_by: u32,
    throw_to_monkey_when_divisible: usize,
    throw_to_monkey_when_not_divisible: usize,
}

struct MonkeyState {
    item_worry_levels: Vec<u32>,
    inspect_count: u32,
}

impl MonkeyState {
    fn remove_first_item(&self) -> Self {
        let mut new_item_worry_levels = self.item_worry_levels.clone();
        new_item_worry_levels.remove(0);
        MonkeyState {
            item_worry_levels: new_item_worry_levels,
            inspect_count: self.inspect_count + 1,
        }
    }

    fn add_item(&self, item_worry_level: u32) -> Self {
        let mut new_item_worry_levels = self.item_worry_levels.clone();
        new_item_worry_levels.push(item_worry_level);
        MonkeyState {
            item_worry_levels: new_item_worry_levels,
            inspect_count: self.inspect_count,
        }
    }

    fn copy(&self) -> Self {
        MonkeyState {
            item_worry_levels: self.item_worry_levels.clone(),
            inspect_count: self.inspect_count,
        }
    }
}

struct Description {
    monkey_descriptions: Vec<MonkeyDescription>,
}

struct State {
    monkey_states: Vec<MonkeyState>,
}

impl State {
    fn perform_round(self, description: &Description) -> Self {
        let monkey_count = self.monkey_states.len();
        (0..monkey_count).fold(self, |old_state, i| {
            old_state.perform_round_for_monkey(description, i)
        })
    }

    fn perform_round_for_monkey(self, description: &Description, monkey_index: usize) -> Self {
        let item_count = self.monkey_states[monkey_index].item_worry_levels.len();
        (0..item_count).fold(self, |old_state, _| {
            old_state.perform_round_for_first_item(description, monkey_index)
        })
    }

    fn perform_round_for_first_item(self, description: &Description, monkey_index: usize) -> Self {
        let monkey_state = &self.monkey_states[monkey_index];
        let monkey_description = &description.monkey_descriptions[monkey_index];
        let item_worry_level = monkey_state.item_worry_levels[0];
        let worry_level = (monkey_description.operation)(item_worry_level);
        let worry_level = worry_level / BORE_FACTOR;
        let new_monkey_index = if worry_level % monkey_description.divisible_by == 0 {
            monkey_description.throw_to_monkey_when_divisible
        } else {
            monkey_description.throw_to_monkey_when_not_divisible
        };
        println!("Moving {item_worry_level} of monkey {monkey_index} to {new_monkey_index}. It now inspected {} items", monkey_state.inspect_count + 1);
        let monkey_count = self.monkey_states.len();
        let new_monkey_states = (0..monkey_count)
            .map(|i| {
                if i == monkey_index {
                    self.monkey_states[i].remove_first_item()
                } else if i == new_monkey_index {
                    self.monkey_states[i].add_item(worry_level)
                } else {
                    self.monkey_states[i].copy()
                }
            })
            .collect();
            State {
            monkey_states: new_monkey_states,
        }
    }

    fn print(&self) {
        self.monkey_states.iter().for_each(|m| {
            let items = m.item_worry_levels.iter().map(|i| i.to_string());
            let items: Vec<String> = items.collect();
            let items = items.join(", ");
            println!("Monkey holds items {} and inspected {} items", items, m.inspect_count);
        });
    }
}

fn main() {
    let (state, description) = read_initial_state();
    state.print();

    let mut final_state =
        (0..ROUNDS).fold(state, |old_state, _| old_state.perform_round(&description));
    final_state.print();

    final_state
        .monkey_states
        .sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    let monkey_business = final_state
        .monkey_states
        .iter()
        .take(2)
        .fold(1, |old_value, monkey_state| {
            old_value * monkey_state.inspect_count
        });
    println!("Monkey business is {monkey_business}");
}

fn read_initial_state() -> (State, Description) {
    let lines = read_input("input.txt");
    let monkey_states = lines.split(|l| l == "").map(parse_monkey_state).collect();
    let monkey_descriptions = lines
        .split(|l| l == "")
        .map(parse_monkey_description)
        .collect();
    (
        State { monkey_states },
        Description {
            monkey_descriptions,
        },
    )
}

fn parse_monkey_state(lines: &[String]) -> MonkeyState {
    let item_worry_levels = parse_items(&lines[1]);
    MonkeyState {
        item_worry_levels,
        inspect_count: 0,
    }
}

fn parse_items(line: &String) -> Vec<u32> {
    match line.strip_prefix(ITEMS_PREFIX) {
        None => panic!(
            "Items line should start with '{}', but was '{}'",
            ITEMS_PREFIX, line
        ),
        Some(part) => part.split(", ").map(parse_number).collect(),
    }
}

fn parse_monkey_description(lines: &[String]) -> MonkeyDescription {
    if !lines[0].starts_with(MONKEY_PREFIX) {
        panic!(
            "Monkey should start with {}, but was {}",
            MONKEY_PREFIX, lines[0]
        );
    }
    let operation = parse_operation(&lines[2]);
    let divisible_by = parse_line_with_number(&lines[3], DIVISIBLE_PREFIX);
    let throw_to_monkey_when_divisible = parse_line_with_number(&lines[4], TRUE_PREFIX) as usize;
    let throw_to_monkey_when_not_divisible =
        parse_line_with_number(&lines[5], FALSE_PREFIX) as usize;
    MonkeyDescription {
        operation,
        divisible_by,
        throw_to_monkey_when_divisible,
        throw_to_monkey_when_not_divisible,
    }
}

fn parse_operation(line: &String) -> Operation {
    match line.strip_prefix(OPERATION_PREFIX) {
        None => panic!(
            "Items line should start with '{}', but was '{}'",
            OPERATION_PREFIX, line
        ),
        Some(part) => parse_operator_and_operand(part),
    }
}

fn parse_operator_and_operand(part: &str) -> Operation {
    let mut parts = part.split(" ");
    let operator: String = match parts.next() {
        None => panic!("Expected operator"),
        Some(o) => o.to_string(),
    };
    match parts.next() {
        None => panic!("Expected operand"),
        Some(o) => {
            if o == "old" {
                match operator.as_str() {
                    "*" => Box::new(move |old_value| old_value * old_value),
                    "+" => Box::new(move |old_value| old_value + old_value),
                    _ => panic!("Invalid operation {}", operator),
                }
            } else {
                let operand = parse_number(o);
                match operator.as_str() {
                    "*" => Box::new(move |old_value| old_value * operand),
                    "+" => Box::new(move |old_value| old_value + operand),
                    _ => panic!("Invalid operation {}", operator),
                }
            }
        }
    }
}

fn parse_line_with_number(line: &str, prefix: &str) -> u32 {
    match line.strip_prefix(prefix) {
        None => panic!(
            "Items line should start with '{}', but was '{}'",
            prefix, line
        ),
        Some(part) => parse_number(part),
    }
}

fn parse_number(item: &str) -> u32 {
    match item.trim().parse::<u32>() {
        Err(why) => panic!("Invalid number {}: {}", item, why),
        Ok(i) => i,
    }
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
