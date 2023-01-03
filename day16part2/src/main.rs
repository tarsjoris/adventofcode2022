use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Tunnel {
    next_valve_name: String,
    distance: i8,
}

type Tunnels = Vec<Tunnel>;

struct Valve {
    name: String,
    rate: u32,
    tunnels: Tunnels,
}

impl Valve {
    fn bypass_valve(&self, valve_to_bypass: &Valve) -> Self {
        let mut new_tunnels = Vec::new();
        self.tunnels
            .iter()
            .filter(|tunnel| valve_to_bypass.name != *tunnel.next_valve_name)
            .for_each(|tunnel| {
                new_tunnels.push(Tunnel {
                    next_valve_name: tunnel.next_valve_name.to_string(),
                    distance: tunnel.distance,
                })
            });
        let old_tunnel = self
            .tunnels
            .iter()
            .filter(|tunnel| *tunnel.next_valve_name == valve_to_bypass.name)
            .next();
        if let Some(old_tunnel) = old_tunnel {
            for new_tunnel in valve_to_bypass.tunnels.iter() {
                if new_tunnel.next_valve_name != self.name {
                    let new_tunnel_length = old_tunnel.distance + new_tunnel.distance;
                    if let Some(position) = new_tunnels
                        .iter()
                        .position(|t| t.next_valve_name == new_tunnel.next_valve_name)
                    {
                        let existing_tunnel = new_tunnels.get(position).unwrap();
                        if new_tunnel_length < existing_tunnel.distance {
                            new_tunnels.remove(position);
                            new_tunnels.push(Tunnel {
                                next_valve_name: new_tunnel.next_valve_name.to_string(),
                                distance: new_tunnel_length,
                            });
                        }
                    } else {
                        new_tunnels.push(Tunnel {
                            next_valve_name: new_tunnel.next_valve_name.to_string(),
                            distance: new_tunnel_length,
                        });
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
    fn get_valve_rates(&self) -> u32 {
        self.valves.values().map(|valve| valve.rate).sum()
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

    fn connect_all_valves(self) -> Self {
        let new_valves = self
            .valves
            .iter()
            .map(|(valve_name, valve)| (valve_name.to_string(), self.valve_with_all_tunnels(valve)))
            .collect();
        Scan { valves: new_valves }
    }

    fn valve_with_all_tunnels(&self, valve: &Valve) -> Valve {
        let mut new_tunnels = Vec::new();
        for existing_tunnel in valve.tunnels.iter() {
            new_tunnels.push(Tunnel {
                next_valve_name: existing_tunnel.next_valve_name.to_string(),
                distance: existing_tunnel.distance,
            });
        }
        let mut extra_tunnels =
            self.get_next_tunnels(&valve.tunnels, &valve.name, &mut new_tunnels);
        while !extra_tunnels.is_empty() {
            extra_tunnels = self.get_next_tunnels(&extra_tunnels, &valve.name, &mut new_tunnels);
        }
        Valve {
            name: valve.name.to_string(),
            rate: valve.rate,
            tunnels: new_tunnels,
        }
    }

    fn get_next_tunnels(
        &self,
        tunnels: &Tunnels,
        own_valve_name: &String,
        new_tunnels: &mut Tunnels,
    ) -> Tunnels {
        let mut next_tunnels = Vec::new();
        for tunnel in tunnels {
            if let Some(next_valve) = self.valves.get(&tunnel.next_valve_name) {
                for next_tunnel in next_valve.tunnels.iter() {
                    if *own_valve_name != next_tunnel.next_valve_name
                        && !new_tunnels
                            .iter()
                            .any(|t| t.next_valve_name == next_tunnel.next_valve_name)
                    {
                        next_tunnels.push(Tunnel {
                            next_valve_name: next_tunnel.next_valve_name.to_string(),
                            distance: tunnel.distance + next_tunnel.distance,
                        });
                        new_tunnels.push(Tunnel {
                            next_valve_name: next_tunnel.next_valve_name.to_string(),
                            distance: tunnel.distance + next_tunnel.distance,
                        });
                    }
                }
            }
        }
        next_tunnels
    }

    fn print(&self) {
        self.valves.iter().for_each(|(valve_name, valve)| {
            println!("{valve_name}");
            valve.tunnels.iter().for_each(|tunnel| {
                println!(
                    "  Tunnel to {} in {}",
                    tunnel.next_valve_name, tunnel.distance
                )
            });
        });
        println!();
    }
}

enum ValveList<'a> {
    End,
    Node(String, &'a ValveList<'a>),
}

impl ValveList<'_> {
    fn contains(&self, valve_name: &String) -> bool {
        match self {
            ValveList::End => false,
            ValveList::Node(name, next) => {
                if valve_name == name {
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
    println!("Bypassed tunnels");
    scan.print();
    scan = scan.connect_all_valves();
    println!("Connected all tunnels");
    scan.print();
    let open_valves = if scan.is_zero_rate(&first_node) {
        ValveList::Node(first_node.to_string(), &ValveList::End)
    } else {
        ValveList::End
    };
    let player_state = PlayerState {
        minutes_left: 26,
        current_valve_name: &first_node,
    };
    let closed_valve_rates = scan.get_valve_rates();
    let most_pressure_release = get_most_pressure_release(
        &scan,
        closed_valve_rates,
        &open_valves,
        &player_state,
        &player_state,
        0,
        &mut 0,
    );
    println!("Most pressure release = {most_pressure_release}");
}

struct PlayerState<'a> {
    minutes_left: i8,
    current_valve_name: &'a String,
}

fn get_most_pressure_release(
    scan: &Scan,
    closed_valve_rates: u32,
    open_valves: &ValveList,
    player_a: &PlayerState,
    player_b: &PlayerState,
    parent_release: u32,
    best_release: &mut u32,
) -> u32 {
    if closed_valve_rates == 0 {
        // no more valves to open, pressure release will stay the same
        return parent_release;
    }
    if player_a.minutes_left <= 0 {
        return parent_release;
    }
    if player_b.minutes_left > player_a.minutes_left {
        return get_most_pressure_release(
            scan,
            closed_valve_rates,
            open_valves,
            player_b,
            player_a,
            parent_release,
            best_release,
        );
    }
    if parent_release + closed_valve_rates * (player_a.minutes_left as u32 - 1) <= *best_release {
        // can't beat the best release anymore, abort
        return 0;
    }
    let mut most_pressure_release = 0;
    if !open_valves.contains(player_a.current_valve_name) {
        let valve_rate = scan.valves.get(player_a.current_valve_name).unwrap().rate;
        let current_release = parent_release + valve_rate * (player_a.minutes_left as u32 - 1);
        if current_release > *best_release {
            println!("Best release {current_release}");
            *best_release = current_release;
        }
        let new_open_valves = ValveList::Node(player_a.current_valve_name.to_string(), open_valves);
        let child_pressure_release = get_most_pressure_release(
            scan,
            closed_valve_rates - valve_rate,
            &new_open_valves,
            &PlayerState {
                minutes_left: player_a.minutes_left - 1,
                current_valve_name: player_a.current_valve_name,
            },
            player_b,
            current_release,
            best_release,
        );
        most_pressure_release = child_pressure_release;
    }
    let max_tunnels_pressure_release = scan
        .valves
        .get(player_a.current_valve_name)
        .unwrap()
        .tunnels
        .iter()
        .filter(|tunnel| !open_valves.contains(&tunnel.next_valve_name))
        .map(|tunnel| {
            get_most_pressure_release(
                scan,
                closed_valve_rates,
                open_valves,
                &PlayerState {
                    minutes_left: player_a.minutes_left - tunnel.distance,
                    current_valve_name: &tunnel.next_valve_name,
                },
                player_b,
                parent_release,
                best_release,
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
        .map(|s| Tunnel {
            next_valve_name: s.to_string(),
            distance: 1,
        })
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
