use std::fs::File;
use std::io::{self, BufRead};

const MIN: i64 = 0;
const MAX: i64 = 4000000;
//const MAX: i64 = 20;

type Point = (i64, i64);

struct Pair {
    sensor: Point,
    distance: u32,
}

fn get_distance(point1: &Point, point2: &Point) -> u32 {
    ((point1.0 - point2.0).abs() + (point1.1 - point2.1).abs())
        .try_into()
        .unwrap()
}

impl Pair {
    fn new(sensor: Point, beacon: Point) -> Self {
        Pair {
            sensor,
            distance: get_distance(&sensor, &beacon),
        }
    }
}

fn main() {
    let pairs = read_input("input.txt");
    let free_spot = find_free_spot(&pairs);
    println!("Tuning frequency = {}", free_spot.0 * 4000000 + free_spot.1);
}

fn find_free_spot(pairs: &Vec<Pair>) -> Point {
    for y in MIN..=MAX {
        let mut x = MIN;
        while x <= MAX {
            let pair = pairs
                .iter()
                .filter(|p| get_distance(&(x, y), &p.sensor) <= p.distance)
                .next();
            let advance: i64 = match pair {
                None => return (x, y),
                Some(p) => {
                    let distance_to_sensor = get_distance(&(x, y), &p.sensor);
                    let remaining_distance = (p.distance - distance_to_sensor) as i64;
                    if x < p.sensor.0 {
                        2 * (p.sensor.0 - x) + remaining_distance
                    } else {
                        remaining_distance
                    }
                }
            };
            x += advance + 1;
        }
    }
    panic!("No free spot found");
}

fn read_input(filename: &str) -> Vec<Pair> {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    let pairs = lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => l,
        })
        .map(|line| parse_line(&line));
    pairs.collect()
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_line(line: &str) -> Pair {
    let mut parts = line.split(": ");
    let sensor = parse_point(parts.next(), "Sensor at ");
    let beacon = parse_point(parts.next(), "closest beacon is at ");
    Pair::new(sensor, beacon)
}

fn parse_point(part: Option<&str>, expected_prefix: &str) -> Point {
    let text = match part {
        None => panic!("Expected another part"),
        Some(p) => p,
    };
    if !text.starts_with(expected_prefix) {
        panic!("Expected {expected_prefix}");
    }
    let point = &text[expected_prefix.len()..text.len()];
    let mut parts = point.split(", ");
    let x = parse_number(parts.next(), "x=");
    let y = parse_number(parts.next(), "y=");
    (x, y)
}

fn parse_number(part: Option<&str>, expected_prefix: &str) -> i64 {
    let text = match part {
        None => panic!("Expected number"),
        Some(p) => p,
    };
    if !text.starts_with(expected_prefix) {
        panic!("Expected {expected_prefix}");
    }
    let number = &text[expected_prefix.len()..text.len()];
    match number.parse::<i64>() {
        Err(why) => panic!("Expected number {number}: {why}"),
        Ok(n) => n,
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
