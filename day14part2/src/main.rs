use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

type Coordinates = (u64, u64);

const SAND_ENTRY_POINT: Coordinates = (500, 0);

struct Cave {
    blocked_tiles: HashSet<u64>,
    running_height: u64,
    bottom: u64,
}

impl Cave {
    fn new() -> Self {
        Cave {
            blocked_tiles: HashSet::new(),
            running_height: 0,
            bottom: 0,
        }
    }

    fn block(&mut self, coordinates: &Coordinates) {
        if coordinates.1 > self.running_height {
            self.running_height = coordinates.1;
        }
        self.blocked_tiles
            .insert(Cave::coordinates_to_u64(coordinates));
    }

    fn mark_bottom(&mut self) {
        self.bottom = self.running_height + 2;
    }

    fn is_blocked(&self, coordinates: &Coordinates) -> bool {
        coordinates.1 == self.bottom
            || self
                .blocked_tiles
                .contains(&Cave::coordinates_to_u64(coordinates))
    }

    fn coordinates_to_u64(coordinates: &Coordinates) -> u64 {
        (coordinates.0 << 32) | coordinates.1
    }
}

fn main() {
    let mut cave = Cave::new();
    read_input("input.txt", &mut cave);
    cave.mark_bottom();
    let sand_units = count_sand_units(&mut cave);
    println!("Number of sand units = {sand_units}");
}

fn count_sand_units(cave: &mut Cave) -> usize {
    let mut count: usize = 0;
    loop {
        count += 1;
        let rest_coordinates = find_rest_coordinates(cave);
        if rest_coordinates.0 == SAND_ENTRY_POINT.0 && rest_coordinates.1 == SAND_ENTRY_POINT.1 {
            return count;
        }
        cave.block(&rest_coordinates);
    }
}

fn find_rest_coordinates(cave: &Cave) -> Coordinates {
    let mut falling_coordinates: Coordinates = (SAND_ENTRY_POINT.0, SAND_ENTRY_POINT.1);
    loop {
        let mut next: Coordinates = (falling_coordinates.0, falling_coordinates.1 + 1);
        if cave.is_blocked(&next) {
            next = (falling_coordinates.0 - 1, falling_coordinates.1 + 1);
            if cave.is_blocked(&next) {
                next = (falling_coordinates.0 + 1, falling_coordinates.1 + 1);
                if cave.is_blocked(&next) {
                    return falling_coordinates;
                }
            }
        }
        falling_coordinates = next;
    }
}

fn read_input(filename: &str, cave: &mut Cave) {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => l,
        })
        .for_each(|line| mark_path(&line, cave));
}

fn mark_path(line: &String, cave: &mut Cave) {
    let coordinates: Vec<Coordinates> = line.split(" -> ").map(parse_coordinates).collect();
    for i in 0..(coordinates.len() - 1) {
        mark_line(&coordinates[i], &coordinates[i + 1], cave);
    }
    if let Some(c) = coordinates.last() {
        cave.block(c);
    }
}

fn parse_coordinates(coordinate: &str) -> Coordinates {
    let mut parts = coordinate.split(",");
    let x = parse_number(parts.next());
    let y = parse_number(parts.next());
    (x, y)
}

fn parse_number(input: Option<&str>) -> u64 {
    match input {
        None => panic!("Expected x coordinate"),
        Some(n) => match n.parse::<u64>() {
            Err(why) => panic!("Expected coordinate {n}: {why}"),
            Ok(number) => number,
        },
    }
}

fn mark_line(coordinate1: &Coordinates, coordinate2: &Coordinates, cave: &mut Cave) {
    if coordinate1.0 == coordinate2.0 {
        let x = coordinate1.0;
        if coordinate1.1 <= coordinate2.1 {
            (coordinate1.1..coordinate2.1).for_each(|y| cave.block(&(x, y)));
        } else {
            (coordinate2.1 + 1..=coordinate1.1).for_each(|y| cave.block(&(x, y)));
        }
    } else {
        let y = coordinate1.1;
        if coordinate1.0 <= coordinate2.0 {
            (coordinate1.0..coordinate2.0).for_each(|x| cave.block(&(x, y)));
        } else {
            (coordinate2.0 + 1..=coordinate1.0).for_each(|x| cave.block(&(x, y)));
        }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
