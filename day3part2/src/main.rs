use std::fs::File;
use std::io::{self, BufRead};

struct Group {
    rucksacks: [String; 3],
}

fn main() {
    let groups = read_input("input.txt");
    let priorities_sum: u32 = groups.iter().map(get_group_badge_priority).sum();
    println!("Sum of priorities is {priorities_sum}");
}

fn get_group_badge_priority(group: &Group) -> u32 {
    let badge = group.rucksacks[0]
        .chars()
        .find(|item| group.rucksacks[1].contains(*item) && group.rucksacks[2].contains(*item));
    return match badge {
        None => panic!(
            "No badge found in {} - {} - {}",
            group.rucksacks[0], group.rucksacks[1], group.rucksacks[2],
        ),
        Some(item) => {
            println!("  Badge is {item}");
            if item >= 'a' && item <= 'z' {
                item as u32 - 'a' as u32 + 1u32
            } else if item >= 'A' && item <= 'Z' {
                item as u32 - 'A' as u32 + 27u32
            } else {
                panic!("Invalid item {item}")
            }
        }
    };
}

fn read_input(filename: &str) -> Vec<Group> {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    let mut unwrapped_lines = lines.map(|line| match line {
        Err(why) => panic!("couldn't read line: {why}"),
        Ok(l) => l,
    });
    let mut groups = Vec::new();
    loop {
        let rucksack1 = match unwrapped_lines.next() {
            None => return groups,
            Some(r) => r,
        };
        let rucksack2 = match unwrapped_lines.next() {
            None => panic!("Last group only has 1 elve"),
            Some(r) => r,
        };
        let rucksack3 = match unwrapped_lines.next() {
            None => panic!("Last group only has 2 elves"),
            Some(r) => r,
        };
        groups.push(Group {
            rucksacks: [rucksack1, rucksack2, rucksack3],
        });
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
