use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::str::Chars;

type List = Vec<Entry>;

enum Entry {
    Value(i32),
    List(List),
}

type Entries = Vec<Entry>;

fn main() {
    let entries = read_input("input.txt");
    let dividers = [
        Entry::List(vec![Entry::List(vec![Entry::Value(2)])]),
        Entry::List(vec![Entry::List(vec![Entry::Value(6)])]),
    ];
    let dividers_index_sum: usize = dividers
        .iter()
        .enumerate()
        .map(|(divider_index, divider)| get_index_of(&divider, &entries) + divider_index)
        .fold(1, |old, index| old * index);
    println!("Dividers index sum = {dividers_index_sum}");
}

fn get_index_of(divider: &Entry, entries: &Entries) -> usize {
    entries
        .iter()
        .filter(|entry| compare_entries(entry, divider) < 0)
        .count()
        + 1
}

fn compare_entries(entry1: &Entry, entry2: &Entry) -> i8 {
    match entry1 {
        Entry::List(l1) => match entry2 {
            Entry::List(l2) => compare_lists(&l1, &l2),
            Entry::Value(v2) => {
                let mut l2: List = Vec::new();
                l2.push(Entry::Value(*v2));
                compare_lists(&l1, &l2)
            }
        },
        Entry::Value(v1) => match entry2 {
            Entry::List(l2) => {
                let mut l1: List = Vec::new();
                l1.push(Entry::Value(*v1));
                compare_lists(&l1, &l2)
            }
            Entry::Value(v2) => compare_values(v1, v2),
        },
    }
}

fn compare_lists(list1: &List, list2: &List) -> i8 {
    for i in 0..list1.len() {
        if i >= list2.len() {
            return 1;
        }
        match compare_entries(&list1[i], &list2[i]) {
            -1 => return -1,
            1 => return 1,
            _ => {}
        };
    }
    if list1.len() == list2.len() {
        0
    } else {
        -1
    }
}

fn compare_values(v1: &i32, v2: &i32) -> i8 {
    if v1 < v2 {
        -1
    } else if v1 == v2 {
        0
    } else {
        1
    }
}

fn read_input(filename: &str) -> Entries {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => l,
        })
        .filter(|line| line.len() > 0)
        .map(|line| parse_entry(&mut line.chars().peekable()))
        .collect()
}

fn parse_entry(line: &mut Peekable<Chars>) -> Entry {
    match line.peek() {
        None => panic!("Expected another entry"),
        Some(char) => match char {
            '[' => parse_list(line),
            _ => parse_value(line),
        },
    }
}

fn parse_list(line: &mut Peekable<Chars>) -> Entry {
    match line.next() {
        None => panic!("Expected ["),
        Some(ch) => {
            if ch != '[' {
                panic!("Expected [, but found {ch}");
            }
        }
    }
    let mut items = Vec::new();
    if match line.peek() {
        None => panic!("Expected number or ]"),
        Some(ch) => *ch == ']',
    } {
        line.next();
        return Entry::List(items);
    }
    items.push(parse_entry(line));
    while let Some(ch) = line.next() {
        match ch {
            ',' => items.push(parse_entry(line)),
            ']' => return Entry::List(items),
            _ => panic!("Expected , or ], but found {ch}"),
        }
    }
    panic!("Expected , or ]");
}

fn parse_value(line: &mut Peekable<Chars>) -> Entry {
    let mut chars: Vec<String> = Vec::new();
    while is_part_of_number(line.peek()) {
        let ch = line.next().unwrap().to_string();
        chars.push(ch);
    }
    let number_str = chars.join("");
    let number = match number_str.parse::<i32>() {
        Err(why) => panic!("Not a number {number_str}: {why}"),
        Ok(n) => n,
    };
    Entry::Value(number)
}

fn is_part_of_number(ch: Option<&char>) -> bool {
    match ch {
        None => false,
        Some(c) => *c == '-' || (*c >= '0' && *c <= '9'),
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
