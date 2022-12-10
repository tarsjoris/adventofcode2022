use std::fs::File;
use std::io::{self, BufRead};
use std::slice::Iter;

const HEADER: &str = "$ cd /";
const CD_PREFIX: &str = "$ cd ";
const DIR_LISTING: &str = "$ ls";
const DIR_PREFIX: &str = "dir ";
const CD_EXIT: &str = "$ cd ..";

const AVAILABLE_DISKSPACE: u32 = 70000000;
const FREE_SPACE_NEEDED: u32 = 30000000;

struct Counters {
    total_count: u32,
    size_of_directory_to_delete: u32,
}

impl Counters {
    fn new() -> Self {
        Counters {
            total_count: 0,
            size_of_directory_to_delete: u32::MAX,
        }
    }

    fn add_size(self, size: u32) -> Self {
        Counters {
            total_count: self.total_count + size,
            size_of_directory_to_delete: self.size_of_directory_to_delete,
        }
    }

    fn add_counters(self, other: Counters) -> Self {
        let size_of_directory_to_delete =
            if other.size_of_directory_to_delete < self.size_of_directory_to_delete {
                other.size_of_directory_to_delete
            } else {
                self.size_of_directory_to_delete
            };
        Counters {
            total_count: self.total_count + other.total_count,
            size_of_directory_to_delete,
        }
    }

    fn merge_current_dir(self, exta_space_needed: u32) -> Self {
        let size_of_directory_to_delete = if self.total_count >= exta_space_needed
            && self.total_count < self.size_of_directory_to_delete
        {
            self.total_count
        } else {
            self.size_of_directory_to_delete
        };
        Counters {
            total_count: self.total_count,
            size_of_directory_to_delete,
        }
    }
}

fn main() {
    let lines = read_input("input.txt");
    let counters = process_input(&lines, 0);
    println!("Total used space = {}", counters.total_count);
    let curent_free_space = AVAILABLE_DISKSPACE - counters.total_count;
    let extra_free_space_needed = FREE_SPACE_NEEDED - curent_free_space;
    println!("Extra free space needed = {extra_free_space_needed}");
    let counters = process_input(&lines, extra_free_space_needed);
    print!(
        "Size of dirctory to delete = {}",
        counters.size_of_directory_to_delete
    );
}

fn process_input(lines: &Vec<String>, exta_space_needed: u32) -> Counters {
    let lines = &mut lines.iter();
    expect(lines, HEADER);
    process_dir(lines, exta_space_needed)
}

fn process_dir(lines: &mut Iter<String>, exta_space_needed: u32) -> Counters {
    expect(lines, DIR_LISTING);
    let mut counters = Counters::new();
    while let Some(line) = lines.next() {
        if line == CD_EXIT {
            break;
        } else if line.starts_with(CD_PREFIX) {
            counters = counters.add_counters(process_dir(lines, exta_space_needed));
        } else if line.starts_with(DIR_PREFIX) {
            // ignore
        } else {
            counters = counters.add_size(process_file(line));
        }
    }
    counters = counters.merge_current_dir(exta_space_needed);
    counters
}

fn process_file(line: &String) -> u32 {
    let size = match line.split(" ").next() {
        None => panic!("Expected file size and file name"),
        Some(s) => s.parse::<u32>(),
    };
    let size = match size {
        Err(why) => panic!("Could not parse size {why}"),
        Ok(s) => s,
    };
    size
}

fn expect(lines: &mut Iter<String>, expected: &str) {
    match lines.next() {
        None => panic!("Expected '{expected}'"),
        Some(l) => {
            if l != expected {
                panic!("Expected '{expected}' but found '{l}'");
            }
        }
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
