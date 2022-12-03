use std::fs::File;
use std::io::{self, BufRead};

struct RuckSack {
    compartment1: String,
    compartment2: String,
}

fn main() {
    let rucksacks = read_input("input.txt");
    let priorities_sum: u32 = rucksacks
        .iter()
        .map(|r| get_incorrect_item_priority(r))
        .sum();
    println!("Sum of priorities is {priorities_sum}");
}

fn get_incorrect_item_priority(rucksack: &RuckSack) -> u32 {
    println!("Rucksack {} - {}", rucksack.compartment1, rucksack.compartment2);
    let incorrect_item = rucksack
        .compartment1
        .chars()
        .find(|item| rucksack.compartment2.contains(*item));
    let priority = match incorrect_item {
        None => panic!(
            "No incorrect item found in {} - {}",
            rucksack.compartment1, rucksack.compartment2
        ),
        Some(item) => {
            println!("  Incorrect item is {item}");
            if item >= 'a' && item <= 'z' {
                item as u32 - 'a' as u32 + 1u32
            } else if item >= 'A' && item <= 'Z' {
                item as u32 - 'A' as u32 + 27u32
            } else {
                panic!("Invalid item {item}")
            }
        }
    };
    println!("  Priority is {priority}");
    return priority;
}

fn parse_rucksack(input: &String) -> RuckSack {
    let middle = input.len() / 2;
    return RuckSack {
        compartment1: String::from(&input[..middle]),
        compartment2: String::from(&input[middle..]),
    };
}

fn read_input(filename: &str) -> Vec<RuckSack> {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    return lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => parse_rucksack(&l),
        })
        .collect();
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
