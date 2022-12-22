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
    let player_state = PlayerState {
        minutes_left: 26,
        current_valve_name: &first_node,
        trail_without_opening: &trail_without_opening,
    };
    let most_pressure_release = get_most_pressure_release(
        &scan,
        closed_valve_count,
        &open_valves,
        &player_state,
        &player_state,
    );
    println!("Most pressure release = {most_pressure_release}");
}

struct PlayerState<'a> {
    minutes_left: u8,
    current_valve_name: &'a String,
    trail_without_opening: &'a ValveList<'a>,
}

fn get_most_pressure_release(
    scan: &Scan,
    closed_valve_count: usize,
    open_valves: &ValveList,
    player_a: &PlayerState,
    player_b: &PlayerState,
) -> u32 {
    if closed_valve_count == 0 {
        return 0;
    }
    if player_b.minutes_left > player_a.minutes_left {
        return get_most_pressure_release(
            scan,
            closed_valve_count,
            open_valves,
            player_b,
            player_a,
        );
    }
    if player_a.minutes_left <= 0 {
        return 0;
    }
    let mut most_pressure_release = 0;
    if !contains(open_valves, player_a.current_valve_name) {
        let new_open_valves = ValveList::Node(player_a.current_valve_name, open_valves);
        let new_trail_without_opening = ValveList::Last(vec![player_a.current_valve_name]);
        let child_pressure_release = get_most_pressure_release(
            scan,
            closed_valve_count - 1,
            &new_open_valves,
            &PlayerState {
                minutes_left: player_a.minutes_left - 1,
                current_valve_name: player_a.current_valve_name,
                trail_without_opening: &new_trail_without_opening,
            },
            player_b,
        );
        let extra_release = scan.get(player_a.current_valve_name).unwrap().rate
            * (player_a.minutes_left as u32 - 1);
        most_pressure_release = child_pressure_release + extra_release;
    }
    let max_tunnels_pressure_release = scan
        .get(player_a.current_valve_name)
        .unwrap()
        .tunnels
        .iter()
        .filter(|tunnel| !contains(player_a.trail_without_opening, tunnel))
        .map(|tunnel| {
            let new_trail_without_opening =
                ValveList::Node(tunnel, &player_a.trail_without_opening);
            get_most_pressure_release(
                scan,
                closed_valve_count,
                open_valves,
                &PlayerState {
                    minutes_left: player_a.minutes_left - 1,
                    current_valve_name: tunnel,
                    trail_without_opening: &&new_trail_without_opening,
                },
                player_b,
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
