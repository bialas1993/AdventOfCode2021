use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "day01/2/input.txt";
const WINDOW_SIZE: usize = 3;

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);
    let mut measurement_inc = 0;
    let mut last_depth = 0;
    
    let measurement: Vec<usize> = reader.lines().map(|x| x.unwrap().parse().unwrap()).collect();
    let iter = measurement.windows(WINDOW_SIZE);

    for (index, data) in iter.enumerate() {
        if index > 0 {
            if last_depth < data.iter().sum() {
                measurement_inc += 1;
            }
        }        

        last_depth = data.iter().sum();
    }
    
     println!("Result: {}", measurement_inc);   
}

