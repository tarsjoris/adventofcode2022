use std::fs::File;
use std::io::{self, BufRead};

struct Direction {
    dx: i8,
    dy: i8,
}

const DIRECTIONS: [Direction; 4] = [
    Direction { dx: 0, dy: -1 },
    Direction { dx: 1, dy: 0 },
    Direction { dx: 0, dy: 1 },
    Direction { dx: -1, dy: 0 },
];

struct Landscape {
    lines: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

struct State {
    least_steps: Vec<Vec<usize>>,
}

struct Node {
    x: usize,
    y: usize,
    height: char,
    char: char,
    depth: usize,
}

fn main() {
    let landscape = read_input("input.txt");
    let min_steps = find_min_steps(&landscape);
    println!("Minimum required steps = {min_steps}");
}

fn find_min_steps(landscape: &Landscape) -> usize {
    let state = &mut State {
        least_steps: vec![vec![usize::MAX; landscape.width]; landscape.height],
    };
    let (start_x, start_y) = get_start_pos(&landscape);

    state.least_steps[start_y][start_x] = 0;
    let mut nodes = vec![Node {
        x: start_x,
        y: start_y,
        height: 'a',
        char: 'S',
        depth: 0,
    }];
    loop {
        let node = nodes.remove(0);
        if node.char == 'E' {
            return node.depth;
        }
        for dir in DIRECTIONS {
            let new_x = node.x as i32 + dir.dx as i32;
            let new_y = node.y as i32 + dir.dy as i32;
            if new_x >= 0
                && new_x < landscape.width as i32
                && new_y >= 0
                && new_y < landscape.height as i32
            {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                let new_char = landscape.lines[new_y][new_x];
                if state.least_steps[new_y][new_x] == usize::MAX {
                    let new_height = get_height_for_char(new_char);
                    if new_height as i32 - node.height as i32 <= 1 {
                        state.least_steps[new_y][new_x] = node.depth + 1;
                        nodes.push(Node {
                            x: new_x,
                            y: new_y,
                            height: new_height,
                            char: new_char,
                            depth: node.depth + 1,
                        });
                    }
                }
            }
        }
    }
}

fn get_height_for_char(char: char) -> char {
    match char {
        'S' => 'a',
        'E' => 'z',
        _ => char,
    }
}

fn get_start_pos(landscape: &Landscape) -> (usize, usize) {
    for y in 0..landscape.lines.len() {
        let line = &landscape.lines[y];
        for x in 0..line.len() {
            let char = line[x];
            if char == 'S' {
                return (x, y);
            }
        }
    }
    return (0, 0);
}

fn read_input(filename: &str) -> Landscape {
    let lines = match read_lines(filename) {
        Err(why) => panic!("couldn't open {filename}: {why}"),
        Ok(lines) => lines,
    };
    let lines: Vec<Vec<char>> = lines
        .map(|line| match line {
            Err(why) => panic!("couldn't read line: {why}"),
            Ok(l) => l,
        })
        .map(|line| line.chars().collect())
        .collect();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);
    let height = lines.len();
    return Landscape {
        lines,
        width,
        height,
    };
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
