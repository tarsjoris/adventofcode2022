use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = read_input("input.txt");
    let best_scenic_score = get_best_scenic_score(&lines);
    println!("Best scenic score: {best_scenic_score}");
}

struct Direction {
    dx: i32,
    dy: i32,
}

const DIRECTIONS: [Direction; 4] = [
    Direction { dx: -1, dy: 0 },
    Direction { dx: 0, dy: -1 },
    Direction { dx: 1, dy: 0 },
    Direction { dx: 0, dy: 1 },
];

fn get_best_scenic_score(lines: &Vec<String>) -> u32 {
    let height = lines.len();
    (0..height)
        .map(|y| get_best_scenic_score_in_row(&lines, y))
        .max()
        .unwrap_or(0)
}

fn get_best_scenic_score_in_row(lines: &Vec<String>, y: usize) -> u32 {
    let width = match lines.iter().nth(y) {
        None => return 0,
        Some(w) => w.len(),
    };
    (0..width)
        .map(|x| get_scenic_score(&lines, x, y))
        .max()
        .unwrap_or(0)
}

fn get_scenic_score(lines: &Vec<String>, x: usize, y: usize) -> u32 {
    println!("Checking tree {x}, {y}");
    let current_tree_height = get_tree_height(&lines, x, y);
    let scenic_score = DIRECTIONS
        .iter()
        .map(|d| get_viewing_distance_in_direction(&lines, x, y, current_tree_height, d))
        .fold(1, |accumulator, element| accumulator * element);
    println!("  Tree scenic score: {scenic_score}");
    scenic_score
}

fn get_viewing_distance_in_direction(
    lines: &Vec<String>,
    x: usize,
    y: usize,
    current_tree_height: u8,
    direction: &Direction,
) -> u32 {
    let mut scooter_x = x as i32 + direction.dx;
    let mut scooter_y = y as i32 + direction.dy;
    let mut viewing_distance: u32 = 0;
    while is_valid_coordinate(&lines, scooter_x, scooter_y) {
        viewing_distance += 1;
        let scooter_tree_height = get_tree_height(&lines, scooter_x as usize, scooter_y as usize);
        if scooter_tree_height >= current_tree_height {
            return viewing_distance;
        }
        scooter_x += direction.dx;
        scooter_y += direction.dy;
    }
    viewing_distance
}

fn get_tree_height(lines: &Vec<String>, x: usize, y: usize) -> u8 {
    match lines.iter().nth(y) {
        None => panic!("Invalid row {y}"),
        Some(line) => match line.chars().nth(x) {
            None => panic!("Invalid column {x} in row {y}"),
            Some(char) => (char as i32 - '0' as i32) as u8,
        },
    }
}

fn is_valid_coordinate(lines: &Vec<String>, x: i32, y: i32) -> bool {
    if x < 0 {
        return false;
    }
    if y < 0 {
        return false;
    }
    match lines.iter().nth(y as usize) {
        None => false,
        Some(line) => match line.chars().nth(x as usize) {
            None => false,
            Some(_) => true,
        },
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
