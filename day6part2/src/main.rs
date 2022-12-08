use std::fs::File;
use std::io::{self, BufRead};

const PACKET_SIZE: usize = 14;

fn main() {
    let lines = read_input("input.txt");
    lines
        .iter()
        .map(solve)
        .for_each(|s| println!("Marker ends at {s}"));
}

fn solve(line: &String) -> i32 {
    let mut chars = line.chars();
    let mut buffer = ['-'; PACKET_SIZE - 1];
    let mut counter: i32 = 0;
    while let Some(c) = chars.next() {
        if counter >= PACKET_SIZE as i32 - 1 {
            if all_different(&buffer) && buffer.iter().all(|b| *b != c) {
                return counter + 1;
            }
        }
        let index = (counter % (PACKET_SIZE as i32 - 1)) as usize;
        buffer[index] = c;
        counter += 1;
    }
    -1
}

fn all_different(buffer: &[char; PACKET_SIZE - 1]) -> bool {
    for i in 0..(PACKET_SIZE - 2) {
        for j in (i + 1)..(PACKET_SIZE - 1) {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    true
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
