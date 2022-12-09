use std::fs::File;
use std::io::{self, BufRead};
use std::slice::Iter;

const HEADER: &str = "$ cd /";
const CD_PREFIX: &str = "$ cd ";
const DIR_LISTING: &str = "$ ls";
const DIR_PREFIX: &str = "dir ";
const CD_EXIT: &str = "$ cd ..";

struct Counters {
    total_count: u32,
    sum_of_small_directories: u32,
}

impl Counters {
    fn add_size(self, size: u32) -> Counters {
        Counters {
            total_count: self.total_count + size,
            sum_of_small_directories: self.sum_of_small_directories,
        }
    }

    fn add_counters(self, other: Counters) -> Counters {
        Counters {
            total_count: self.total_count + other.total_count,
            sum_of_small_directories: self.sum_of_small_directories
                + other.sum_of_small_directories,
        }
    }
}

fn main() {
    let lines = read_input("input.txt");
    let lines = &mut lines.iter();
    expect(lines, HEADER);
    let counters = process_dir(lines);
    print!(
        "Sum of the total sizes = {}",
        counters.sum_of_small_directories
    );
}

fn process_dir(lines: &mut Iter<String>) -> Counters {
    expect(lines, DIR_LISTING);
    let mut counters = Counters {
        total_count: 0,
        sum_of_small_directories: 0,
    };
    while let Some(line) = lines.next() {
        if line == CD_EXIT {
            break;
        } else if line.starts_with(CD_PREFIX) {
            counters = counters.add_counters(process_dir(lines));
        } else if line.starts_with(DIR_PREFIX) {
            // ignore
        } else {
            counters = counters.add_size(process_file(line));
        }
    }
    if counters.total_count <= 100000 {
        counters = Counters {
            total_count: counters.total_count,
            sum_of_small_directories: counters.sum_of_small_directories + counters.total_count,
        };
    }
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
