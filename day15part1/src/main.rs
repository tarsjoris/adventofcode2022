use std::fs::File;
use std::io::{self, BufRead};

type Point = (i64, i64);

struct Pair {
    sensor: Point,
    beacon: Point,
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
            beacon,
            distance: get_distance(&sensor, &beacon),
        }
    }
}

fn main() {
    let pairs = read_input("input.txt");
    let min_x = pairs.iter().map(|p| p.sensor.0 - p.distance as i64).min().unwrap_or(0);
    let max_x = pairs.iter().map(|p| p.sensor.0 + p.distance as i64).max().unwrap_or(0);
    const Y: i64 = 10;
    //const Y: i64 = 2000000;
    let no_beacon_count = (min_x..=max_x)
        .filter(|x| no_beacon(&(*x, Y), &pairs))
        .count();
    println!();
    println!("No beacon count {no_beacon_count}");
}

// ####B######################
fn no_beacon(point: &Point, pairs: &Vec<Pair>) -> bool {
    !point_has_beacon(point, pairs) && within_sensor_reach(point, pairs)
}

fn point_has_beacon(point: &Point, pairs: &Vec<Pair>) -> bool {
    pairs
        .iter()
        .any(|pair| point.0 == pair.beacon.0 && point.1 == pair.beacon.1)
}

fn within_sensor_reach(point: &Point, pairs: &Vec<Pair>) -> bool {
    pairs
        .iter()
        .any(|pair| get_distance(point, &pair.sensor) <= pair.distance)
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
