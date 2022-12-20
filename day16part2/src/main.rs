use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Valve {
    rate: u32,
    tunnels: Vec<String>,
}

type Scan = HashMap<String, Valve>;

enum ValveList<'a> {
    Last(Vec<&'a String>),
    Node(&'a String, &'a ValveList<'a>),
}

fn main() {
    let scan = read_input("input.txt");
    let useless_valve_names: Vec<&String> = scan
        .iter()
        .filter(|(_, valve)| valve.rate == 0)
        .map(|(name, _)| name)
        .collect();
    let closed_valve_count = scan.len() - useless_valve_names.len();
    let open_valves = ValveList::Last(useless_valve_names);
    let first_node = "AA".to_string();
    let trail_without_opening = ValveList::Last(vec![&first_node]);
    let most_pressure_release = get_most_pressure_release(
        &scan,
        26,
        false,
        closed_valve_count,
        &first_node,
        &first_node,
        &open_valves,
        &trail_without_opening,
        &trail_without_opening,
    );
    println!("Most pressure release = {most_pressure_release}");
}

fn get_most_pressure_release(
    scan: &Scan,
    minutes_left: u8,
    advance_time: bool,
    closed_valve_count: usize,
    current_valve_name_a: &String,
    current_valve_name_b: &String,
    open_valves: &ValveList,
    trail_without_opening_a: &ValveList,
    trail_without_opening_b: &ValveList,
) -> u32 {
    if minutes_left == 0 || closed_valve_count == 0 {
        return 0;
    }
    let remaining_minutes = if advance_time {
        minutes_left - 1
    } else {
        minutes_left
    };
    let mut most_pressure_release = 0;
    if !contains(open_valves, current_valve_name_a) {
        let new_open_valves = ValveList::Node(current_valve_name_a, open_valves);
        let new_trail_without_opening = ValveList::Last(vec![current_valve_name_a]);
        let child_pressure_release = get_most_pressure_release(
            scan,
            remaining_minutes,
            !advance_time,
            closed_valve_count - 1,
            current_valve_name_b,
            current_valve_name_a,
            &new_open_valves,
            &trail_without_opening_b,
            &new_trail_without_opening,
        );
        let extra_release =
            scan.get(current_valve_name_a).unwrap().rate * (minutes_left as u32 - 1);
        most_pressure_release = child_pressure_release + extra_release;
    }
    let max_tunnels_pressure_release = scan
        .get(current_valve_name_a)
        .unwrap()
        .tunnels
        .iter()
        .filter(|tunnel| !contains(trail_without_opening_a, tunnel))
        .map(|tunnel| {
            let new_trail_without_opening = ValveList::Node(tunnel, &trail_without_opening_a);
            get_most_pressure_release(
                scan,
                remaining_minutes,
                !advance_time,
                closed_valve_count,
                current_valve_name_b,
                tunnel,
                open_valves,
                &trail_without_opening_b,
                &new_trail_without_opening,
            )
        })
        .max()
        .unwrap_or(0);
    most_pressure_release = cmp::max(most_pressure_release, max_tunnels_pressure_release);
    most_pressure_release
}

fn contains(valve_list: &ValveList, valve_name: &String) -> bool {
    match valve_list {
        ValveList::Last(names) => names.contains(&valve_name),
        ValveList::Node(name, next) => {
            if valve_name == *name {
                true
            } else {
                contains(next, valve_name)
            }
        }
    }
}

fn read_input(filename: &str) -> Scan {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => l,
        })
        .map(|line| parse_valve(&line))
        .collect()
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn parse_valve(line: &String) -> (String, Valve) {
    let mut parts = line.split(&[';', '=', ',', ' ']);
    assert_part(parts.next(), "Valve");
    let name = parts.next().unwrap().to_string();
    assert_part(parts.next(), "has");
    assert_part(parts.next(), "flow");
    assert_part(parts.next(), "rate");
    let rate = parse_number(parts.next());
    assert_part(parts.next(), "");
    parts.next(); // tunnels/tunnel
    parts.next(); // leads/lead
    assert_part(parts.next(), "to");
    parts.next(); // valves/valve
    let tunnels = parts
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    (name, Valve { rate, tunnels })
}

fn assert_part(part: Option<&str>, expected_literal: &str) {
    match part {
        None => panic!("Expected another part"),
        Some(p) => {
            if p != expected_literal {
                panic!("Expected '{expected_literal}' but found '{p}'")
            }
        }
    };
}

fn parse_number(part: Option<&str>) -> u32 {
    let text = match part {
        None => panic!("Expected number"),
        Some(p) => p,
    };
    match text.parse::<u32>() {
        Err(why) => panic!("Expected number, but found {text}: {why}"),
        Ok(n) => n,
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
