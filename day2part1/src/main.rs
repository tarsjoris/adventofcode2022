use std::fs::File;
use std::io::{self, BufRead};

const SHAPE_ROCK: i32 = 1;
const SHAPE_PAPER: i32 = 2;
const SHAPE_SCISSORS: i32 = 3;

struct Round {
    player1: i32,
    player2: i32,
}

fn main() {
    let rounds = read_input("input.txt");
    let score: i32 = rounds.iter().map(|round| get_score(&round)).sum();
    println!("Total score is {score}")
}

fn get_score(round: &Round) -> i32 {
    let score = match round.player1 {
        SHAPE_ROCK => match round.player2 {
            SHAPE_ROCK => 3,
            SHAPE_PAPER => 0,
            SHAPE_SCISSORS => 6,
            _ => panic!("Unknown shape {}", round.player2),
        },
        SHAPE_PAPER => match round.player2 {
            SHAPE_ROCK => 6,
            SHAPE_PAPER => 3,
            SHAPE_SCISSORS => 0,
            _ => panic!("Unknown shape {}", round.player2),
        },
        SHAPE_SCISSORS => match round.player2 {
            SHAPE_ROCK => 0,
            SHAPE_PAPER => 6,
            SHAPE_SCISSORS => 3,
            _ => panic!("Unknown shape {}", round.player2),
        },
        _ => panic!("Unknown shape {}", round.player1),
    };
    return round.player1 + score;
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
    let player1_char = match chars.next() {
        None => panic!("Expected player 1's move"),
        Some(a) => a,
    };

     let player2 = match player2_char {
        'A' => SHAPE_ROCK,
        'B' => SHAPE_PAPER,
        'C' => SHAPE_SCISSORS,
        _ => panic!("Invalid char for player 2 {player2_char}")
     };
     let player1 = match player1_char {
        'X' => SHAPE_ROCK,
        'Y' => SHAPE_PAPER,
        'Z' => SHAPE_SCISSORS,
        _ => panic!("Invalid char for player 1 {player1_char}")
     };
     return Round {
        player1,
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