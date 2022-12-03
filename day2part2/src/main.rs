use std::fs::File;
use std::io::{self, BufRead};

const SHAPE_ROCK: i32 = 1;
const SHAPE_PAPER: i32 = 2;
const SHAPE_SCISSORS: i32 = 3;

const RESULT_LOSE: i32 = 1;
const RESULT_DRAW: i32 = 2;
const RESULT_WIN: i32 = 3;

struct Round {
    result: i32,
    player2: i32,
}

fn main() {
    let rounds = read_input("input.txt");
    let score: i32 = rounds.iter().map(|round| get_score(&round)).sum();
    println!("Total score is {score}")
}

fn get_score(round: &Round) -> i32 {
    return match round.player2 {
        SHAPE_ROCK => match round.result {
            RESULT_LOSE => SHAPE_SCISSORS,
            RESULT_DRAW => SHAPE_ROCK + 3,
            RESULT_WIN => SHAPE_PAPER + 6,
            _ => panic!("Unknown result {}", round.result),
        },
        SHAPE_PAPER => match round.result {
            RESULT_LOSE => SHAPE_ROCK,
            RESULT_DRAW => SHAPE_PAPER + 3,
            RESULT_WIN => SHAPE_SCISSORS + 6,
            _ => panic!("Unknown result {}", round.result),
        },
        SHAPE_SCISSORS => match round.result {
            RESULT_LOSE => SHAPE_PAPER,
            RESULT_DRAW => SHAPE_SCISSORS + 3,
            RESULT_WIN => SHAPE_ROCK + 6,
            _ => panic!("Unknown result {}", round.result),
        },
        _ => panic!("Unknown shape {}", round.player2),
    };
}

fn parse_round(input: &String) -> Round {
    let mut chars = input.chars();
    let player2_char = match chars.next() {
        None => panic!("Expected player 2's move"),
        Some(a) => a,
    };
    let space = match chars.next() {
        None => panic!("Expected space"),
        Some(a) => a,
    };
    if space != ' ' {
        panic!("Expected space but found {space}");
    }
    let result_char = match chars.next() {
        None => panic!("Expected result"),
        Some(a) => a,
    };

     let player2 = match player2_char {
        'A' => SHAPE_ROCK,
        'B' => SHAPE_PAPER,
        'C' => SHAPE_SCISSORS,
        _ => panic!("Invalid char for player 2 {player2_char}")
     };
     let result = match result_char {
        'X' => RESULT_LOSE,
        'Y' => RESULT_DRAW,
        'Z' => RESULT_WIN,
        _ => panic!("Invalid char for result {result_char}")
     };
     return Round {
        result,
        player2,
     };
}

fn read_input(filename: &str) -> Vec<Round> {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    return lines.map(|line|
        match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => parse_round(&l),
        }).collect();
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}