use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const FILENAME: &str = "day02/2/input.txt";
const DIRECTION_FORWARD: &str = "forward";
const DIRECTION_DOWN: &str = "down";
const DIRECTION_UP: &str = "up";

struct Position {
    a: i32,
    x: i32,
    y: i32,
}

impl Position {
    fn run(&mut self, direction: &str, steps: i32) {
        match direction {
            DIRECTION_DOWN => self.a += steps,
            DIRECTION_UP => self.a -= steps,
            DIRECTION_FORWARD => { 
                self.x += steps; 
                self.y += self.a * steps
            },
            _ => {},
        }
    }

    fn new(a: i32, x: i32, y: i32) -> Position {
        Position { a: a, x: x, y: y }
    }
}


fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut pos = Position::new(0, 0, 0);

    for (_index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut s = l.split_whitespace();
        pos.run(s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap());
    }
   

    println!("pos:[x:{}, y:{}, a: {}]", pos.x, pos.y, pos.a);
    println!("Result: {}", pos.x * pos.y)
}
