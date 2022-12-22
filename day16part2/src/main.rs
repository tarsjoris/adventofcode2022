use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

type Tunnel = (String, i8);

struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<Tunnel>,
}

impl Valve {
    fn bypass_valve(&self, valve_to_bypass: &Valve) -> Self {
        let mut new_tunnels = Vec::new();
        self.tunnels
            .iter()
            .filter(|(tunnel_name, _)| valve_to_bypass.name != *tunnel_name)
            .for_each(|(tunnel_name, length)| new_tunnels.push((tunnel_name.to_string(), *length)));
        let old_tunnel = self
            .tunnels
            .iter()
            .filter(|(tunnel_name, _)| *tunnel_name == valve_to_bypass.name)
            .next();
        if let Some(old_tunnel) = old_tunnel {
            for (new_tunnel_name, length) in valve_to_bypass.tunnels.iter() {
                if *new_tunnel_name != self.name {
                    let new_tunnel_length = old_tunnel.1 + length;
                    if let Some(position) = new_tunnels
                        .iter()
                        .position(|(name, _)| name == new_tunnel_name)
                    {
                        let (_, existing_tunnel_length) = new_tunnels.get(position).unwrap();
                        if new_tunnel_length < *existing_tunnel_length {
                            new_tunnels.remove(position);
                            new_tunnels.push((new_tunnel_name.to_string(), new_tunnel_length));
                        }
                    } else {
                        new_tunnels.push((new_tunnel_name.to_string(), new_tunnel_length));
                    }
                }
            }
        }
        Valve {
            name: self.name.to_string(),
            rate: self.rate,
            tunnels: new_tunnels,
        }
    }
}

struct Scan {
    valves: HashMap<String, Valve>,
}

impl Scan {
    fn get_closed_valve_count(&self) -> usize {
        self.valves
            .iter()
            .filter(|(_, valve)| valve.rate != 0)
            .count()
    }

    fn is_zero_rate(&self, valve_name: &String) -> bool {
        self.valves
            .values()
            .any(|valve| valve.name == *valve_name && valve.rate == 0)
    }

    fn bypass_useless_valves(self) -> Self {
        let useless_valve_names: Vec<&String> = self
            .valves
            .values()
            .filter(|valve| valve.rate == 0)
            .map(|valve| &valve.name)
            .collect();
        let mut valve_it = useless_valve_names.iter();
        if let Some(useless_valve_name) = valve_it.next() {
            let mut scooter = self.bypass_valve(useless_valve_name);
            while let Some(useless_valve_name) = valve_it.next() {
                scooter = scooter.bypass_valve(useless_valve_name);
            }
            scooter
        } else {
            self
        }
    }

    fn bypass_valve(&self, useless_valve_name: &String) -> Self {
        let useless_valve = self
            .valves
            .values()
            .filter(|valve| valve.name == *useless_valve_name)
            .next()
            .unwrap();
        let new_valves = self
            .valves
            .iter()
            .map(|(valve_name, valve)| (valve_name.to_string(), valve.bypass_valve(&useless_valve)))
            .collect();
        Scan { valves: new_valves }
    }

    fn print(&self) {
        println!();
        self.valves.iter().for_each(|(valve_name, valve)| {
            println!("{valve_name}");
            valve
                .tunnels
                .iter()
                .for_each(|tunnel| println!("  Tunnel to {} in {}", tunnel.0, tunnel.1));
        });
    }
}

enum ValveList<'a> {
    End,
    Node(&'a String, &'a ValveList<'a>),
}

impl ValveList<'_> {
    fn contains(&self, valve_name: &String) -> bool {
        match self {
            ValveList::End => false,
            ValveList::Node(name, next) => {
                if valve_name == *name {
                    true
                } else {
                    next.contains(valve_name)
                }
            }
        }
    }
}

fn main() {
    let first_node = "AA".to_string();
    let mut scan = read_input("input.txt");
    scan = scan.bypass_useless_valves();
    scan.print();
    let closed_valve_count = scan.get_closed_valve_count();
    let open_valves = if scan.is_zero_rate(&first_node) {
        ValveList::Node(&first_node, &ValveList::End)
    } else {
        ValveList::End
    };
    let trail_without_opening = ValveList::Node(&first_node, &ValveList::End);
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
    minutes_left: i8,
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
    if !open_valves.contains(player_a.current_valve_name) {
        let new_open_valves = ValveList::Node(player_a.current_valve_name, open_valves);
        let new_trail_without_opening =
            ValveList::Node(player_a.current_valve_name, &ValveList::End);
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
        let extra_release = scan.valves.get(player_a.current_valve_name).unwrap().rate
            * (player_a.minutes_left as u32 - 1);
        most_pressure_release = child_pressure_release + extra_release;
    }
    let max_tunnels_pressure_release = scan
        .valves
        .get(player_a.current_valve_name)
        .unwrap()
        .tunnels
        .iter()
        .filter(|tunnel| !player_a.trail_without_opening.contains(&tunnel.0))
        .map(|(tunnel_name, tunnel_length)| {
            let new_trail_without_opening =
                ValveList::Node(tunnel_name, &player_a.trail_without_opening);
            get_most_pressure_release(
                scan,
                closed_valve_count,
                open_valves,
                &PlayerState {
                    minutes_left: player_a.minutes_left - tunnel_length,
                    current_valve_name: tunnel_name,
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

fn read_input(filename: &str) -> Scan {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    Scan {
        valves: lines
            .map(|line| match line {
                Err(why) => panic!("couldn't read line: {why}"),
                Ok(l) => l,
            })
            .map(|line| parse_valve(&line))
            .collect(),
    }
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
        .map(|s| (s.to_string(), 1))
        .collect();
    let valve_name = name.to_string();
    (
        name,
        Valve {
            name: valve_name,
            rate,
            tunnels,
        },
    )
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
