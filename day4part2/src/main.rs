use std::fs::File;
use std::io::{self, BufRead};

struct Assignment {
    lower_bound_inclusive: i32,
    upper_bound_inclusive: i32,
}

struct Pair {
    elves_assignments: [Assignment; 2],
}

fn main() {
    let pairs = read_input("input.txt");
    pairs.iter().for_each(|p| {
        println!(
            "Pair {}-{},{}-{}",
            p.elves_assignments[0].lower_bound_inclusive,
            p.elves_assignments[0].upper_bound_inclusive,
            p.elves_assignments[1].lower_bound_inclusive,
            p.elves_assignments[1].upper_bound_inclusive
        )
    });
    let count = pairs.iter().filter(|p| has_assignment_overlap(*p)).count();
    println!("Pairs with overlap: {count}");
}

fn has_assignment_overlap(pair: &Pair) -> bool {
    for i in 0..pair.elves_assignments.len() - 1 {
        for j in i + 1..pair.elves_assignments.len() {
            if assignment_overlaps(&pair.elves_assignments[i], &pair.elves_assignments[j])
                || assignment_overlaps(&pair.elves_assignments[j], &pair.elves_assignments[i])
            {
                return true;
            }
        }
    }
    return false;
}

fn assignment_overlaps(assignment1: &Assignment, assignment2: &Assignment) -> bool {
    return assignment1.lower_bound_inclusive <= assignment2.upper_bound_inclusive
        && assignment1.upper_bound_inclusive >= assignment2.lower_bound_inclusive;
}

fn parse_assignments(assignments: String) -> Assignment {
    let mut parts = assignments.split("-");
    let lower_bound = match parts.next() {
        None => panic!("Assignment is missing lower bound"),
        Some(a) => a.to_string().parse::<i32>(),
    };
    let lower_bound_unwrapped = match lower_bound {
        Err(why) => panic!("Could not parse lower bound as i32 {why}"),
        Ok(i) => i,
    };
    let upper_bound = match parts.next() {
        None => panic!("Assignment is missing upper bound"),
        Some(a) => a.to_string().parse::<i32>(),
    };
    let upper_bound_unwrapped = match upper_bound {
        Err(why) => panic!("Could not parse upper bound as i32 {why}"),
        Ok(i) => i,
    };
    return Assignment {
        lower_bound_inclusive: lower_bound_unwrapped,
        upper_bound_inclusive: upper_bound_unwrapped,
    };
}

fn parse_pair(line: String) -> Pair {
    let mut parts = line.split(",");
    let elve1_assignment = match parts.next() {
        None => panic!("Elve 1 assignments missing"),
        Some(a) => parse_assignments(a.to_string()),
    };
    let elve2_assignment = match parts.next() {
        None => panic!("Elve 2 assignments missing"),
        Some(a) => parse_assignments(a.to_string()),
    };
    return Pair {
        elves_assignments: [elve1_assignment, elve2_assignment],
    };
}

fn read_input(filename: &str) -> Vec<Pair> {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    let unwrapped_lines = lines.map(|line| match line {
        Err(why) => panic!("couldn't read line: {why}"),
        Ok(l) => l,
    });
    return unwrapped_lines.map(parse_pair).collect();
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
