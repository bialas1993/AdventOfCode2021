use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "day01/1/input.txt";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut measurement_inc = 0;
    let mut last_depth = 0;

    for (index, line) in reader.lines().enumerate() {
        let measurement = line.unwrap();
        let depth: i32 = measurement.parse().unwrap();

        if index > 0 {
            if last_depth < depth {
                measurement_inc += 1;
            }
        }        

        last_depth = depth;
    }

    println!("Result: {}", measurement_inc);
}