use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let calories_per_elve = read_input("input.txt");
    let sum_of_3_most = get_sum_of_3_most_calories(calories_per_elve);
    println!("Answer: {sum_of_3_most}");
}

fn get_sum_of_3_most_calories(calories_per_elve: Vec<Vec<i32>>) -> i32 {
    let mut sums: Vec<i32> = calories_per_elve.iter()
        .map(|cals| cals.iter().sum())
        .collect();
    sums.sort_by(|a, b| b.cmp(a));
    return sums.iter().take(3).sum();
}

fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };

    let mut calories_per_elve = Vec::new();
    let mut calories = Vec::new();
    for line_result in lines {
        let line = match line_result {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(line) => line,
        };
        if line == "" {
            calories_per_elve.push(calories);
            calories = Vec::new();
        } else {
            let val = match line.parse::<i32>() {
                Err(why) => panic!("not a number ({line}): {why}"),
                Ok(val) => val,
            };
            calories.push(val);
        }
    }
    calories_per_elve.push(calories);
    return calories_per_elve;
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}